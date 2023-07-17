pub use prettyplease;

use std::{collections::BTreeMap, sync::OnceLock};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use rust_gpu_bridge::glam::Vec2;
use syn::{
    parse_quote, token::Mut, BinOp, Block, Expr, ExprAssign, ExprBinary, ExprBlock, ExprCall,
    ExprField, ExprIf, ExprLet, ExprLit, ExprMethodCall, ExprPath, ExprReturn, ExprStruct,
    ExprUnary, Field, FieldMutability, FieldValue, Fields, FieldsNamed, File, FnArg, Generics,
    Item, ItemFn, ItemMod, ItemStruct, Lit, LitBool, LitFloat, Pat, PatPath, PatType, Path,
    PathSegment, ReturnType, Signature, Stmt, Type, TypePath, Visibility,
};

use elysian_interpreter::{evaluate_module, Interpreter};

use elysian_core::ast::Elysian;
use elysian_core::ir::{
    ast::{Block as IrBlock, Expr as IrExpr, Property, Stmt as IrStmt, Struct, CONTEXT},
    from_elysian::{elysian_module, CONTEXT_STRUCT},
};

/// Distributed slice of shape hash -> shape function pairs
/// Populated at link-time by auto-generated shape modules
#[linkme::distributed_slice]
pub static STATIC_SHAPES: [(u64, fn(Struct<f32, Vec2>) -> Struct<f32, Vec2>)] = [..];

/// Runtime storage for static shape data
static STATIC_SHAPES_MAP: OnceLock<BTreeMap<u64, fn(Struct<f32, Vec2>) -> Struct<f32, Vec2>>> =
    OnceLock::new();

/// Accessor for STATIC_SHAPES_MAP
pub fn static_shapes_map() -> &'static BTreeMap<u64, fn(Struct<f32, Vec2>) -> Struct<f32, Vec2>> {
    STATIC_SHAPES_MAP.get_or_init(|| STATIC_SHAPES.into_iter().copied().collect())
    //STATIC_SHAPES_MAP.get_or_init(Default::default)
}

/// Build.rs static shape registrar
pub fn static_shapes<'a, T: IntoIterator<Item = (&'a str, Elysian<f32, Vec2>)>>(t: T) {
    let source: String = t
        .into_iter()
        .map(|(name, shape)| {
            let syn = elysian_to_syn(&shape, name);
            prettyplease::unparse(&syn)
        })
        .collect();

    let out_dir = std::env::var_os("OUT_DIR").expect("No OUT_DIR environment variable");
    let dest_path = std::path::Path::new(&out_dir).join("static_shapes.rs");
    std::fs::write(&dest_path, source).unwrap();
}

/// Convenience macro for including generated static shape code
#[macro_export]
macro_rules! include_static_shapes {
    () => {
        include!(concat!(env!("OUT_DIR"), "/static_shapes.rs"));
    };
}

/// Return a function that calls the static implementation of a given shape if it exists,
/// falling back to the interpreter otherwise.
pub fn dispatch_shape(
    shape: &Elysian<f32, Vec2>,
) -> Box<dyn Fn(Struct<f32, Vec2>) -> Struct<f32, Vec2> + Send + Sync> {
    let hash = shape.shape_hash();

    if let Some(f) = static_shapes_map().get(&hash) {
        println!("Dispatching to static function");
        Box::new(|context| f(context))
    } else {
        println!("Dispatching to dynamic interpreter");
        let module = elysian_module(shape);
        Box::new(move |context| {
            evaluate_module(
                Interpreter {
                    context,
                    ..Default::default()
                },
                &module,
            )
        })
    }
}

pub fn type_to_syn(ty: &elysian_core::ir::module::Type) -> TokenStream {
    match ty {
        elysian_core::ir::module::Type::Boolean => quote!(Type::Boolean),
        elysian_core::ir::module::Type::Number => quote!(Type::Number),
        elysian_core::ir::module::Type::Vector => quote!(Type::Vector),
        elysian_core::ir::module::Type::Struct(_) => unimplemented!(),
    }
}

pub fn type_to_value(ty: &elysian_core::ir::module::Type) -> TokenStream {
    match ty {
        elysian_core::ir::module::Type::Boolean => quote!(Value::Boolean),
        elysian_core::ir::module::Type::Number => quote!(Value::Number),
        elysian_core::ir::module::Type::Vector => quote!(Value::Vector),
        elysian_core::ir::module::Type::Struct(_) => unimplemented!(),
    }
}

pub fn property_to_syn(prop: &Property) -> TokenStream {
    let name = prop.id().name();
    let ty = type_to_syn(prop.ty());
    let uuid = prop.id().uuid().as_u128();
    quote! {
        Property::new(#name, #ty, #uuid)
    }
}

pub fn elysian_to_syn(elysian: &Elysian<f32, Vec2>, name: &str) -> File {
    let mut attrs = vec![];

    attrs.push(parse_quote! {
        #![allow(unused_parens, non_camel_case_types)]
    });

    let mut items = vec![];

    items.push(parse_quote! {
        use rust_gpu_bridge::{glam::*, *};
    });

    items.push(parse_quote! {
        use elysian::core::ir::{ast::{Struct, StructIO, Property, Value}, module::Type};
    });

    let module = elysian_module(&elysian);

    for def in &module.struct_definitions {
        items.push(Item::Struct(ItemStruct {
            attrs: vec![parse_quote!(#[derive(Debug, Default, Copy, Clone)])],
            vis: if def.public {
                Visibility::Public(Default::default())
            } else {
                Visibility::Inherited
            },
            struct_token: Default::default(),
            ident: Ident::new(&def.name_unique(), Span::call_site()),
            generics: Generics {
                lt_token: None,
                params: Default::default(),
                gt_token: None,
                where_clause: None,
            },
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: def
                    .fields
                    .iter()
                    .map(|field| Field {
                        attrs: vec![],
                        vis: if field.public {
                            Visibility::Public(Default::default())
                        } else {
                            Visibility::Inherited
                        },
                        mutability: FieldMutability::None,
                        ident: Some(Ident::new(&field.prop.name_unique(), Span::call_site())),
                        colon_token: Default::default(),
                        ty: Type::Path(TypePath {
                            qself: None,
                            path: Ident::new(&field.prop.ty().name_unique(), Span::call_site())
                                .into(),
                        }),
                    })
                    .collect(),
            }),
            semi_token: None,
        }));
    }

    let struct_name = Ident::new(&CONTEXT_STRUCT.name_unique(), Span::call_site());

    let members: Vec<_> = CONTEXT_STRUCT
        .fields
        .iter()
        .map(|field| property_to_syn(&field.prop))
        .collect();

    let values: Vec<_> = CONTEXT_STRUCT
        .fields
        .iter()
        .map(|field| type_to_value(field.prop.ty()))
        .collect();

    let names: Vec<_> = CONTEXT_STRUCT
        .fields
        .iter()
        .map(|field| Ident::new(&field.prop.name_unique(), Span::call_site()))
        .collect();

    items.push(syn::parse_quote! {
        impl From<Struct<f32, Vec2>> for #struct_name {
            fn from(s: Struct<f32, Vec2>) -> Self {
                let mut out = Self::default();

                #(
                    if let Some(v) = s.try_get(&#members) {
                        let #values(v) = v else {
                            panic!("Unexpected type");
                        };

                        out.#names = v;
                    }
                )*

                out
            }
        }
    });

    items.push(syn::parse_quote! {
        impl From<#struct_name> for Struct<f32, Vec2> {
            fn from(s: #struct_name) -> Self {
                let mut out = Self::default();

                #(
                    out.set_mut(#members, #values(s.#names));
                )*

                out
            }
        }
    });

    for def in &module.function_definitions {
        let name = Ident::new(def.name(), Span::call_site());

        let args: Vec<FnArg> = def
            .inputs
            .iter()
            .map(|input| {
                let mutability: Option<Mut> = if input.mutable {
                    Some(Default::default())
                } else {
                    None
                };

                let pat = Ident::new(&input.prop.name_unique(), Span::call_site());
                let ty = Ident::new(&input.prop.ty().name_unique(), Span::call_site());

                parse_quote! {
                    #mutability #pat: #ty
                }
            })
            .collect();

        let output = Ident::new(&def.output.name_unique(), Span::call_site());

        let block = Block {
            brace_token: Default::default(),
            stmts: def.block.0.iter().map(stmt_to_syn).collect(),
        };

        let item = parse_quote! {
            fn #name(#(#args),*) -> #output #block
        };

        items.push(item);
    }

    let context_struct_name = Ident::new(CONTEXT_STRUCT.name(), Span::call_site());
    let context_struct_name_unique = Ident::new(&CONTEXT_STRUCT.name_unique(), Span::call_site());
    items.push(parse_quote! {
        pub type #context_struct_name = #context_struct_name_unique;
    });

    items.push(Item::Fn(ItemFn {
        attrs: vec![],
        vis: Visibility::Public(Default::default()),
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new("entry_point", Span::call_site()),
            generics: Generics {
                lt_token: None,
                params: Default::default(),
                gt_token: None,
                where_clause: None,
            },
            paren_token: Default::default(),
            inputs: [FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Path(PatPath {
                    attrs: vec![],
                    qself: None,
                    path: Path {
                        leading_colon: Default::default(),
                        segments: [PathSegment {
                            ident: Ident::new(CONTEXT.name(), Span::call_site()),
                            arguments: Default::default(),
                        }]
                        .into_iter()
                        .collect(),
                    },
                })),
                colon_token: Default::default(),
                ty: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: Default::default(),
                        segments: [PathSegment {
                            ident: Ident::new(CONTEXT_STRUCT.name(), Span::call_site()),
                            arguments: Default::default(),
                        }]
                        .into_iter()
                        .collect(),
                    },
                })),
            })]
            .into_iter()
            .collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Ident::new(CONTEXT_STRUCT.name(), Span::call_site()).into(),
                })),
            ),
        },
        block: Box::new({
            let mut block = block_to_syn(&module.entry_point.block);
            block.stmts.insert(
                0,
                Stmt::Expr(
                    Expr::Let(ExprLet {
                        attrs: vec![],
                        let_token: Default::default(),
                        pat: Box::new(Pat::Path(PatPath {
                            attrs: vec![],
                            qself: None,
                            path: Path {
                                leading_colon: Default::default(),
                                segments: [PathSegment {
                                    ident: Ident::new(&CONTEXT.name_unique(), Span::call_site()),
                                    arguments: Default::default(),
                                }]
                                .into_iter()
                                .collect(),
                            },
                        })),
                        eq_token: Default::default(),
                        expr: Box::new(Expr::Path(ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: Path {
                                leading_colon: Default::default(),
                                segments: [PathSegment {
                                    ident: Ident::new(CONTEXT.name(), Span::call_site()),
                                    arguments: Default::default(),
                                }]
                                .into_iter()
                                .collect(),
                            },
                        })),
                    }),
                    Some(Default::default()),
                ),
            );
            block
        }),
    }));

    items.push(parse_quote! {
        pub fn shape(context: Struct<f32, Vec2>) -> Struct<f32, Vec2> {
            entry_point(context.into()).into()
        }
    });

    let hash = elysian.shape_hash();
    items.push(parse_quote! {
        #[linkme::distributed_slice(elysian::syn::STATIC_SHAPES)]
        static STATIC_SHAPE: (u64, fn(Struct<f32, Vec2>) -> Struct<f32, Vec2>) = (#hash, shape);
    });

    let items = vec![Item::Mod(ItemMod {
        attrs,
        vis: Visibility::Inherited,
        unsafety: None,
        mod_token: Default::default(),
        ident: Ident::new(name, Span::call_site()),
        content: Some((Default::default(), items)),
        semi: None,
    })];

    File {
        shebang: None,
        attrs: vec![],
        items,
    }
}

fn block_to_syn(block: &IrBlock<f32, Vec2>) -> Block {
    Block {
        brace_token: Default::default(),
        stmts: block.0.iter().map(stmt_to_syn).collect(),
    }
}

fn stmt_to_syn(stmt: &IrStmt<f32, Vec2>) -> Stmt {
    match stmt {
        IrStmt::Block(block) => Stmt::Expr(
            Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block: block_to_syn(block),
            }),
            Default::default(),
        ),
        IrStmt::Write { path, expr } => Stmt::Expr(
            if path.len() == 1 {
                Expr::Let(syn::ExprLet {
                    attrs: vec![],
                    let_token: Default::default(),
                    pat: Box::new(Pat::Path(PatPath {
                        attrs: vec![],
                        qself: None,
                        path: Ident::new(
                            &path
                                .iter()
                                .map(|prop| prop.name_unique())
                                .collect::<String>(),
                            Span::call_site(),
                        )
                        .into(),
                    })),
                    eq_token: Default::default(),
                    expr: Box::new(expr_to_syn(expr)),
                })
            } else {
                Expr::Assign(ExprAssign {
                    attrs: vec![],
                    left: Box::new(path_to_syn(path)),
                    eq_token: Default::default(),
                    right: Box::new(expr_to_syn(expr)),
                })
            },
            Some(Default::default()),
        ),
        IrStmt::If {
            cond,
            then,
            otherwise,
        } => Stmt::Expr(
            Expr::If(ExprIf {
                attrs: vec![],
                if_token: Default::default(),
                cond: Box::new(expr_to_syn(cond)),
                then_branch: Block {
                    brace_token: Default::default(),
                    stmts: vec![stmt_to_syn(then)],
                },
                else_branch: otherwise.as_ref().map(|otherwise| {
                    (
                        Default::default(),
                        Box::new(Expr::Block(ExprBlock {
                            attrs: vec![],
                            label: None,
                            block: Block {
                                brace_token: Default::default(),
                                stmts: vec![stmt_to_syn(otherwise)],
                            },
                        })),
                    )
                }),
            }),
            Default::default(),
        ),
        IrStmt::Output(expr) => Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: vec![],
                return_token: Default::default(),
                expr: Some(Box::new(expr_to_syn(expr))),
            }),
            Default::default(),
        ),
        _ => unimplemented!(),
    }
}

fn expr_to_syn(expr: &IrExpr<f32, Vec2>) -> Expr {
    match expr {
        elysian_core::ir::ast::Expr::Literal(v) => match v {
            elysian_core::ir::ast::Value::Boolean(b) => Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Bool(LitBool {
                    value: *b,
                    span: Span::call_site(),
                }),
            }),
            elysian_core::ir::ast::Value::Number(n) => Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Float(LitFloat::new(&(n.to_string() + &"f32"), Span::call_site())),
            }),
            elysian_core::ir::ast::Value::Vector(v) => Expr::Call(ExprCall {
                attrs: vec![],
                func: Box::new(Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: Path {
                        leading_colon: Default::default(),
                        segments: [
                            PathSegment {
                                ident: Ident::new("Vec2", Span::call_site()),
                                arguments: Default::default(),
                            },
                            PathSegment {
                                ident: Ident::new("new", Span::call_site()),
                                arguments: Default::default(),
                            },
                        ]
                        .into_iter()
                        .collect(),
                    },
                })),
                paren_token: Default::default(),
                args: [
                    Expr::Lit(ExprLit {
                        attrs: vec![],
                        lit: Lit::Float(LitFloat::new(
                            &(v.x.to_string() + &"f32"),
                            Span::call_site(),
                        )),
                    }),
                    Expr::Lit(ExprLit {
                        attrs: vec![],
                        lit: Lit::Float(LitFloat::new(
                            &(v.y.to_string() + &"f32"),
                            Span::call_site(),
                        )),
                    }),
                ]
                .into_iter()
                .collect(),
            }),
            elysian_core::ir::ast::Value::Struct(_) => {
                unimplemented!()
            }
            _ => unimplemented!(),
        },
        IrExpr::Read(path) => path_to_syn(path),
        IrExpr::Call { function, args } => Expr::Call(ExprCall {
            attrs: vec![],
            func: Box::new(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Ident::new(function.name(), Span::call_site()).into(),
            })),
            paren_token: Default::default(),
            args: args.iter().map(|t| expr_to_syn(t)).collect(),
        }),
        IrExpr::Construct(structure, fields) => Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Ident::new(&structure.name_unique(), Span::call_site()).into(),
            brace_token: Default::default(),
            fields: fields
                .iter()
                .map(|(prop, expr)| {
                    let ident = Ident::new(&prop.name_unique(), Span::call_site());
                    let expr = expr_to_syn(expr);
                    let colon_token = if let Expr::Path(ExprPath { path, .. }) = &expr {
                        if let Some(i) = path.get_ident() {
                            if ident == *i {
                                None
                            } else {
                                Some(Default::default())
                            }
                        } else {
                            Some(Default::default())
                        }
                    } else {
                        Some(Default::default())
                    };

                    FieldValue {
                        attrs: vec![],
                        member: syn::Member::Named(ident),
                        colon_token,
                        expr,
                    }
                })
                .collect(),
            dot2_token: if fields.len() == structure.fields.len() {
                None
            } else {
                Some(Default::default())
            },
            rest: Some(Box::new(Expr::Call(ExprCall {
                attrs: vec![],
                func: Box::new(Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: Path {
                        leading_colon: Default::default(),
                        segments: [
                            PathSegment {
                                ident: Ident::new("Default", Span::call_site()),
                                arguments: Default::default(),
                            },
                            PathSegment {
                                ident: Ident::new("default", Span::call_site()),
                                arguments: Default::default(),
                            },
                        ]
                        .into_iter()
                        .collect(),
                    },
                })),
                paren_token: Default::default(),
                args: Default::default(),
            }))),
        }),
        IrExpr::Add(lhs, rhs)
        | IrExpr::Sub(lhs, rhs)
        | IrExpr::Mul(lhs, rhs)
        | IrExpr::Div(lhs, rhs)
        | IrExpr::Lt(lhs, rhs)
        | IrExpr::Gt(lhs, rhs) => Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: Default::default(),
            expr: Box::new(Expr::Binary(ExprBinary {
                attrs: vec![],
                left: Box::new(expr_to_syn(lhs)),
                op: match expr {
                    IrExpr::Add(_, _) => BinOp::Add(Default::default()),
                    IrExpr::Sub(_, _) => BinOp::Sub(Default::default()),
                    IrExpr::Mul(_, _) => BinOp::Mul(Default::default()),
                    IrExpr::Div(_, _) => BinOp::Div(Default::default()),
                    IrExpr::Lt(_, _) => BinOp::Lt(Default::default()),
                    IrExpr::Gt(_, _) => BinOp::Gt(Default::default()),
                    _ => unreachable!(),
                },
                right: Box::new(expr_to_syn(rhs)),
            })),
        }),
        IrExpr::Min(lhs, rhs) | IrExpr::Max(lhs, rhs) | IrExpr::Dot(lhs, rhs) => {
            Expr::MethodCall(ExprMethodCall {
                attrs: vec![],
                receiver: Box::new(expr_to_syn(lhs)),
                dot_token: Default::default(),
                method: match expr {
                    IrExpr::Min(_, _) => Ident::new("min", Span::call_site()),
                    IrExpr::Max(_, _) => Ident::new("max", Span::call_site()),
                    IrExpr::Dot(_, _) => Ident::new("dot", Span::call_site()),
                    _ => unreachable!(),
                },
                turbofish: None,
                paren_token: Default::default(),
                args: [expr_to_syn(rhs)].into_iter().collect(),
            })
        }
        IrExpr::Mix(lhs, rhs, t) => Expr::MethodCall(ExprMethodCall {
            attrs: vec![],
            receiver: Box::new(expr_to_syn(lhs)),
            dot_token: Default::default(),
            method: Ident::new("mix", Span::call_site()),
            turbofish: None,
            paren_token: Default::default(),
            args: [expr_to_syn(rhs), expr_to_syn(t)].into_iter().collect(),
        }),
        IrExpr::Neg(t) => Expr::Unary(ExprUnary {
            attrs: vec![],
            op: syn::UnOp::Neg(Default::default()),
            expr: Box::new(expr_to_syn(t)),
        }),
        IrExpr::Abs(t) | IrExpr::Sign(t) | IrExpr::Length(t) | IrExpr::Normalize(t) => {
            Expr::MethodCall(ExprMethodCall {
                attrs: vec![],
                receiver: Box::new(expr_to_syn(t)),
                dot_token: Default::default(),
                method: Ident::new(
                    match expr {
                        IrExpr::Abs(_) => "abs",
                        IrExpr::Sign(_) => "sign",
                        IrExpr::Length(_) => "length",
                        IrExpr::Normalize(_) => "normalize_or_zero",
                        _ => unreachable!(),
                    },
                    Span::call_site(),
                ),
                turbofish: None,
                paren_token: Default::default(),
                args: Default::default(),
            })
        }
        _ => unimplemented!(),
    }
}

fn path_to_syn(path: &Vec<Property>) -> Expr {
    let mut iter = path.iter();

    let base = Expr::Path(ExprPath {
        attrs: vec![],
        qself: None,
        path: Ident::new(
            &iter.next().expect("Empty path").name_unique(),
            Span::call_site(),
        )
        .into(),
    });

    if path.len() == 1 {
        base
    } else {
        iter.fold(base, |acc, next| {
            Expr::Field(ExprField {
                attrs: vec![],
                base: Box::new(acc),
                dot_token: Default::default(),
                member: syn::Member::Named(Ident::new(&next.name_unique(), Span::call_site())),
            })
        })
    }
}

pub mod output {}

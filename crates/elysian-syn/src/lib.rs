use elysian_core::ir::{
    ast::{
        Identifier, CONTEXT, MATRIX2, MATRIX3, MATRIX4, VECTOR2, VECTOR3, VECTOR4, W, W_AXIS_4, X,
        X_AXIS_2, X_AXIS_3, X_AXIS_4, Y, Y_AXIS_2, Y_AXIS_3, Y_AXIS_4, Z, Z_AXIS_3, Z_AXIS_4,
    },
    module::{Module, NumericType, SpecializationData},
};
pub use prettyplease;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, token::Mut, BinOp, Block, Expr, ExprAssign, ExprBinary, ExprBlock, ExprCall,
    ExprField, ExprIf, ExprLit, ExprLoop, ExprMethodCall, ExprPath, ExprReturn, ExprStruct,
    ExprUnary, Field, FieldMutability, FieldValue, Fields, FieldsNamed, File, FnArg, Generics,
    Item, ItemFn, ItemMod, ItemStruct, Lit, LitBool, LitFloat, LitInt, Pat, PatIdent, Path,
    PathSegment, ReturnType, Signature, Stmt, Type, TypePath, Visibility,
};

use elysian_core::ir::{
    ast::{Block as IrBlock, Expr as IrExpr, Stmt as IrStmt},
    module::AsModule,
};

pub fn type_to_syn(ty: &elysian_core::ir::module::Type) -> TokenStream {
    match ty {
        elysian_core::ir::module::Type::Boolean => quote!(Type::Boolean),
        elysian_core::ir::module::Type::Number(n) => match n {
            NumericType::UInt => quote!(Type::Number(NumericType::UInt)),
            NumericType::SInt => quote!(Type::Number(NumericType::SInt)),
            NumericType::Float => quote!(Type::Number(NumericType::Float)),
        },
        elysian_core::ir::module::Type::Struct(s) => match s.name() {
            "Vector2" => quote!(Type::Struct(VECTOR2)),
            "Vector3" => quote!(Type::Struct(VECTOR3)),
            "Vector4" => quote!(Type::Struct(VECTOR4)),
            "Matrix2" => quote!(Type::Struct(MATRIX2)),
            "Matrix3" => quote!(Type::Struct(MATRIX3)),
            "Matrix4" => quote!(Type::Struct(MATRIX4)),
            _ => unimplemented!(),
        },
    }
}

pub fn type_to_value(ty: &elysian_core::ir::module::Type) -> TokenStream {
    match ty {
        elysian_core::ir::module::Type::Boolean => quote!(Value::Boolean),
        elysian_core::ir::module::Type::Number(_) => quote!(Value::Number),
        elysian_core::ir::module::Type::Struct(_) => unimplemented!(),
    }
}

pub fn property_to_syn(id: &Identifier) -> TokenStream {
    let name = id.name();
    let uuid = id.uuid().as_u128();
    quote! {
        Identifier::new(#name, #uuid)
    }
}

pub fn module_to_string<T>(input: &T, spec: &SpecializationData, name: &str) -> String
where
    T: AsModule,
{
    prettyplease::unparse(&module_to_syn(input, spec, name))
}

pub fn module_to_syn<T>(input: &T, spec: &SpecializationData, name: &str) -> File
where
    T: AsModule,
{
    let name = Ident::new(name, Span::call_site());

    let mut attrs = vec![];

    attrs.push(parse_quote! {
        #![allow(unused_parens, non_camel_case_types)]
    });

    let mut items = vec![];

    items.push(parse_quote! {
        use rust_gpu_bridge::{glam::*, *};
    });

    items.push(parse_quote! {
        use elysian::{
            core::{
                ir::{
                    ast::{
                        Struct,
                        Identifier,
                        CONTEXT,
                    },
                },
            },
            r#static::StaticShape,
        };
    });

    let module = input.module(spec);

    for def in &module.struct_definitions {
        match &def.id {
            v if *v == VECTOR2
                || *v == VECTOR3
                || *v == VECTOR4
                || *v == MATRIX2
                || *v == MATRIX3
                || *v == MATRIX4 => {}
            _ => {
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
                                ident: Some(Ident::new(&field.id.name_unique(), Span::call_site())),
                                colon_token: Default::default(),
                                ty: Type::Path(TypePath {
                                    qself: None,
                                    path: Ident::new(
                                        &builtin_types(
                                            &module
                                                .types
                                                .get(&field.id)
                                                .unwrap_or_else(|| {
                                                    panic!("No type for {}", field.id.name())
                                                })
                                                .name_unique(),
                                        ),
                                        Span::call_site(),
                                    )
                                    .into(),
                                }),
                            })
                            .collect(),
                    }),
                    semi_token: None,
                }));
            }
        }
    }

    let def = module
        .struct_definitions
        .iter()
        .find(|cand| cand.id == CONTEXT)
        .unwrap();

    let struct_name = Ident::new(&def.id.name_unique(), Span::call_site());

    let members: Vec<_> = def
        .fields
        .iter()
        .map(|field| property_to_syn(&field.id))
        .collect();

    let names: Vec<_> = def
        .fields
        .iter()
        .map(|field| Ident::new(&field.id.name_unique(), Span::call_site()))
        .collect();

    items.push(syn::parse_quote! {
        impl From<Struct> for #struct_name {
            fn from(s: Struct) -> Self {
                let mut out = Self::default();

                #(
                    if let Some(v) = s.try_get(&#members) {
                        out.#names = v.into();
                    }
                )*

                out
            }
        }
    });

    items.push(syn::parse_quote! {
        impl From<#struct_name> for Struct {
            fn from(s: #struct_name) -> Self {
                let mut out = Self::new(CONTEXT);

                #(
                    out.set_mut(#members, s.#names.into());
                )*

                out
            }
        }
    });

    for def in &module.function_definitions {
        let name = Ident::new(&def.name_unique(), Span::call_site());

        let args: Vec<FnArg> = def
            .inputs
            .iter()
            .map(|input| {
                let mutability: Option<Mut> = if input.mutable {
                    Some(Default::default())
                } else {
                    None
                };

                let pat = Ident::new(&input.id.name_unique(), Span::call_site());
                let ty = Ident::new(
                    &builtin_types(
                        &module
                            .types
                            .get(&input.id)
                            .unwrap_or_else(|| panic!("No type for {}", input.id.name()))
                            .name_unique(),
                    ),
                    Span::call_site(),
                );

                parse_quote! {
                    #mutability #pat: #ty
                }
            })
            .collect();

        let output = Ident::new(&def.output.name_unique(), Span::call_site());

        let block = Block {
            brace_token: Default::default(),
            stmts: def
                .block
                .0
                .iter()
                .map(|stmt| stmt_to_syn(&module, stmt))
                .collect(),
        };

        let item = Item::Fn(ItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: name,
                generics: Generics {
                    lt_token: None,
                    params: Default::default(),
                    gt_token: None,
                    where_clause: None,
                },
                paren_token: Default::default(),
                inputs: args.into_iter().collect(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: Default::default(),
                            segments: [PathSegment {
                                ident: output,
                                arguments: Default::default(),
                            }]
                            .into_iter()
                            .collect(),
                        },
                    })),
                ),
            },
            block: Box::new(block),
        });

        items.push(item);
    }

    let entry_point_name = Ident::new(&module.entry_point.name_unique(), Span::call_site());
    items.push(parse_quote! {
        pub fn #name(context: Struct) -> Struct {
            #entry_point_name(context.into()).into()
        }
    });

    let hash = input.hash_ir();
    items.push(parse_quote! {
        #[linkme::distributed_slice(elysian::r#static::STATIC_SHAPES)]
        static STATIC_SHAPE: StaticShape = StaticShape {
            hash: #hash,
            function: #name
        };
    });

    let items = vec![Item::Mod(ItemMod {
        attrs,
        vis: Visibility::Inherited,
        unsafety: None,
        mod_token: Default::default(),
        ident: name,
        content: Some((Default::default(), items)),
        semi: None,
    })];

    File {
        shebang: None,
        attrs: vec![],
        items,
    }
}

fn builtin_types(name: &str) -> &str {
    match name {
        "UInt" => "u32",
        "SInt" => "i32",
        "Float" => "f32",
        "Vector2" => "Vec2",
        "Vector3" => "Vec3",
        "Vector4" => "Vec4",
        "Matrix2" => "Mat2",
        "Matrix3" => "Mat3",
        "Matrix4" => "Mat4",
        _ => name,
    }
}

fn block_to_syn(module: &Module, block: &IrBlock) -> Block {
    Block {
        brace_token: Default::default(),
        stmts: block
            .0
            .iter()
            .map(|stmt| stmt_to_syn(module, stmt))
            .collect(),
    }
}

fn stmt_to_syn(module: &Module, stmt: &IrStmt) -> Stmt {
    match stmt {
        IrStmt::Block(block) => Stmt::Expr(
            Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block: block_to_syn(module, block),
            }),
            Default::default(),
        ),
        IrStmt::Bind { prop, expr } => Stmt::Expr(
            Expr::Let(syn::ExprLet {
                attrs: vec![],
                let_token: Default::default(),
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: Some(Default::default()),
                    ident: Ident::new(&prop.name_unique(), Span::call_site()),
                    subpat: None,
                })),
                eq_token: Default::default(),
                expr: Box::new(expr_to_syn(module, expr)),
            }),
            Some(Default::default()),
        ),
        IrStmt::Write { path, expr } => Stmt::Expr(
            Expr::Assign(ExprAssign {
                attrs: vec![],
                left: Box::new(path_to_syn(path)),
                eq_token: Default::default(),
                right: Box::new(expr_to_syn(module, expr)),
            }),
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
                cond: Box::new(expr_to_syn(module, cond)),
                then_branch: Block {
                    brace_token: Default::default(),
                    stmts: vec![stmt_to_syn(module, then)],
                },
                else_branch: otherwise.as_ref().map(|otherwise| {
                    (
                        Default::default(),
                        Box::new(Expr::Block(ExprBlock {
                            attrs: vec![],
                            label: None,
                            block: Block {
                                brace_token: Default::default(),
                                stmts: vec![stmt_to_syn(module, otherwise)],
                            },
                        })),
                    )
                }),
            }),
            Default::default(),
        ),
        IrStmt::Loop { stmt } => Stmt::Expr(
            Expr::Loop(ExprLoop {
                attrs: vec![],
                label: None,
                loop_token: Default::default(),
                body: Block {
                    brace_token: Default::default(),
                    stmts: vec![stmt_to_syn(module, stmt)],
                },
            }),
            None,
        ),
        IrStmt::Break => Stmt::Expr(
            Expr::Break(syn::ExprBreak {
                attrs: vec![],
                break_token: Default::default(),
                label: None,
                expr: None,
            }),
            Some(Default::default()),
        ),
        IrStmt::Output(expr) => Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: vec![],
                return_token: Default::default(),
                expr: Some(Box::new(expr_to_syn(module, expr))),
            }),
            Default::default(),
        ),
    }
}

fn expr_to_syn(module: &Module, expr: &IrExpr) -> Expr {
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
                lit: match n {
                    elysian_core::ir::ast::Number::UInt(n) => {
                        let n = *n as u32;
                        Lit::Int(LitInt::new(&(n.to_string() + &"u32"), Span::call_site()))
                    }
                    elysian_core::ir::ast::Number::SInt(n) => {
                        let n = *n as i32;
                        Lit::Int(LitInt::new(&(n.to_string() + &"i32"), Span::call_site()))
                    }
                    elysian_core::ir::ast::Number::Float(n) => {
                        let n = *n as f32;
                        Lit::Float(LitFloat::new(&(n.to_string() + &"f32"), Span::call_site()))
                    }
                },
            }),
            elysian_core::ir::ast::Value::Struct(s) => expr_to_syn(
                module,
                &IrExpr::Struct(
                    s.id.clone(),
                    s.members
                        .iter()
                        .map(|(k, v)| (k.clone(), IrExpr::Literal(v.clone())))
                        .collect(),
                ),
            ),
        },
        IrExpr::Read(path) => path_to_syn(path),
        IrExpr::Call { function, args } => Expr::Call(ExprCall {
            attrs: vec![],
            func: Box::new(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Ident::new(&function.name_unique(), Span::call_site()).into(),
            })),
            paren_token: Default::default(),
            args: args.iter().map(|t| expr_to_syn(module, t)).collect(),
        }),
        IrExpr::Struct(structure, fields) => match structure.name() {
            "Vector2" | "Vector3" | "Vector4" | "Matrix2" | "Matrix3" | "Matrix4" => {
                Expr::Call(ExprCall {
                    attrs: vec![],
                    func: Box::new(Expr::Path(ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident::new(
                                        builtin_types(structure.name()),
                                        Span::call_site(),
                                    ),
                                    arguments: Default::default(),
                                },
                                PathSegment {
                                    ident: Ident::new(
                                        match structure.name() {
                                            "Vector2" | "Vector3" | "Vector4" => "new",
                                            "Matrix2" | "Matrix3" | "Matrix4" => "from_cols",
                                            _ => unreachable!(),
                                        },
                                        Span::call_site(),
                                    ),
                                    arguments: Default::default(),
                                },
                            ]
                            .into_iter()
                            .collect(),
                        },
                    })),
                    paren_token: Default::default(),
                    args: match structure.name() {
                        "Vector2" => [
                            expr_to_syn(module, fields.get(&X).expect("No X for Vec2")),
                            expr_to_syn(module, fields.get(&Y).expect("No Y for Vec2")),
                        ]
                        .into_iter()
                        .collect(),
                        "Vector3" => [
                            expr_to_syn(module, fields.get(&X).expect("No X for Vec3")),
                            expr_to_syn(module, fields.get(&Y).expect("No Y for Vec3")),
                            expr_to_syn(module, fields.get(&Z).expect("No Z for Vec3")),
                        ]
                        .into_iter()
                        .collect(),
                        "Vector4" => [
                            expr_to_syn(
                                module,
                                fields.get(&X).unwrap_or_else(|| {
                                    panic!("No X in {fields:#?} for {structure:#?}")
                                }),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Y).unwrap_or_else(|| {
                                    panic!("No Y in {fields:#?} for {structure:#?}")
                                }),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Z).unwrap_or_else(|| {
                                    panic!("No Z in {fields:#?} for {structure:#?}")
                                }),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&W).unwrap_or_else(|| {
                                    panic!("No W in {fields:#?} for {structure:#?}")
                                }),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                        "Matrix2" => [
                            expr_to_syn(
                                module,
                                fields.get(&X_AXIS_2).expect("No X_AXIS for Matrix2"),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Y_AXIS_2).expect("No Y_AXIS for Matrix2"),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                        "Matrix3" => [
                            expr_to_syn(
                                module,
                                fields.get(&X_AXIS_3).expect("No X_AXIS for Matrix3"),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Y_AXIS_3).expect("No Y_AXIS for Matrix3"),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Z_AXIS_3).expect("No Z_AXIS for Matrix3"),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                        "Matrix4" => [
                            expr_to_syn(
                                module,
                                fields.get(&X_AXIS_4).expect("No X_AXIS for Matrix4"),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Y_AXIS_4).expect("No Y_AXIS for Matrix4"),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&Z_AXIS_4).expect("No Z_AXIS for Matrix4"),
                            ),
                            expr_to_syn(
                                module,
                                fields.get(&W_AXIS_4).expect("No W_AXIS for Matrix4"),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                        _ => unreachable!(),
                    },
                })
            }
            _ => Expr::Struct(ExprStruct {
                attrs: vec![],
                qself: None,
                path: Ident::new(&structure.name_unique(), Span::call_site()).into(),
                brace_token: Default::default(),
                fields: fields
                    .iter()
                    .map(|(prop, expr)| {
                        let ident = Ident::new(&prop.name_unique(), Span::call_site());
                        let expr = expr_to_syn(module, expr);
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
                dot2_token: {
                    let structure = module
                        .struct_definitions
                        .iter()
                        .find(|cand| cand.id == *structure)
                        .unwrap();
                    if fields.len() == structure.fields.len() {
                        None
                    } else {
                        Some(Default::default())
                    }
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
        },
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
                left: Box::new(expr_to_syn(module, lhs)),
                op: match expr {
                    IrExpr::Add(_, _) => BinOp::Add(Default::default()),
                    IrExpr::Sub(_, _) => BinOp::Sub(Default::default()),
                    IrExpr::Mul(_, _) => BinOp::Mul(Default::default()),
                    IrExpr::Div(_, _) => BinOp::Div(Default::default()),
                    IrExpr::Lt(_, _) => BinOp::Lt(Default::default()),
                    IrExpr::Gt(_, _) => BinOp::Gt(Default::default()),
                    _ => unreachable!(),
                },
                right: Box::new(expr_to_syn(module, rhs)),
            })),
        }),
        IrExpr::Min(lhs, rhs) | IrExpr::Max(lhs, rhs) | IrExpr::Dot(lhs, rhs) => {
            Expr::MethodCall(ExprMethodCall {
                attrs: vec![],
                receiver: Box::new(expr_to_syn(module, lhs)),
                dot_token: Default::default(),
                method: match expr {
                    IrExpr::Min(_, _) => Ident::new("min", Span::call_site()),
                    IrExpr::Max(_, _) => Ident::new("max", Span::call_site()),
                    IrExpr::Dot(_, _) => Ident::new("dot", Span::call_site()),
                    _ => unreachable!(),
                },
                turbofish: None,
                paren_token: Default::default(),
                args: [expr_to_syn(module, rhs)].into_iter().collect(),
            })
        }
        IrExpr::Mix(lhs, rhs, t) => Expr::MethodCall(ExprMethodCall {
            attrs: vec![],
            receiver: Box::new(expr_to_syn(module, lhs)),
            dot_token: Default::default(),
            method: Ident::new("mix", Span::call_site()),
            turbofish: None,
            paren_token: Default::default(),
            args: [expr_to_syn(module, rhs), expr_to_syn(module, t)]
                .into_iter()
                .collect(),
        }),
        IrExpr::Neg(t) => Expr::Unary(ExprUnary {
            attrs: vec![],
            op: syn::UnOp::Neg(Default::default()),
            expr: Box::new(expr_to_syn(module, t)),
        }),
        IrExpr::Abs(t) | IrExpr::Sign(t) | IrExpr::Length(t) | IrExpr::Normalize(t) => {
            Expr::MethodCall(ExprMethodCall {
                attrs: vec![],
                receiver: Box::new(expr_to_syn(module, t)),
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
    }
}

fn path_to_syn(path: &Vec<Identifier>) -> Expr {
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

    iter.fold(base, |acc, next| {
        Expr::Field(ExprField {
            attrs: vec![],
            base: Box::new(acc),
            dot_token: Default::default(),
            member: syn::Member::Named(Ident::new(&next.name_unique(), Span::call_site())),
        })
    })
}

pub mod output {}

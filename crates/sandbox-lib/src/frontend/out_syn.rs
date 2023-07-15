use std::fmt::Debug;

use proc_macro2::{Ident, Span};
use quote::quote;
use rust_gpu_bridge::glam::Vec2;
use syn::{
    AttrStyle, Attribute, BinOp, Block, Expr, ExprAssign, ExprBinary, ExprBlock, ExprCall,
    ExprField, ExprIf, ExprLit, ExprMethodCall, ExprPath, ExprReturn, ExprStruct, ExprUnary, Field,
    FieldMutability, FieldValue, Fields, FieldsNamed, File, FnArg, Generics, Item, ItemFn,
    ItemStruct, ItemUse, Lit, LitBool, LitFloat, Meta, MetaList, Pat, PatIdent, PatPath, PatType,
    Path, PathSegment, ReturnType, Signature, Stmt, Type, TypePath, UseGlob, UseGroup, UsePath,
    UseTree, Visibility,
};

use crate::{
    elysian::{expand::Expand, Elysian},
    ir::{
        ast::{Block as IrBlock, Expr as IrExpr, Property, Stmt as IrStmt, CONTEXT},
        from_elysian::{elysian_module, CONTEXT_STRUCT},
    },
};

pub fn elysian_to_syn<N, V>(elysian: &Elysian<f32, Vec2>) -> File
where
    N: Debug + Copy,
    V: Debug + Copy,
{
    let elysian = elysian.expand();

    let mut attrs = vec![];

    attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Inner(Default::default()),
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: Path {
                leading_colon: Default::default(),
                segments: [PathSegment {
                    ident: Ident::new("allow", Span::call_site()),
                    arguments: Default::default(),
                }]
                .into_iter()
                .collect(),
            },
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: quote!(unused_parens),
        }),
    });

    let mut items = vec![];

    items.push(Item::Use(ItemUse {
        attrs: vec![],
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("rust_gpu_bridge", Span::call_site()),
            colon2_token: Default::default(),
            tree: Box::new(UseTree::Group(UseGroup {
                brace_token: Default::default(),
                items: [
                    UseTree::Glob(UseGlob {
                        star_token: Default::default(),
                    }),
                    UseTree::Path(UsePath {
                        ident: Ident::new("glam", Span::call_site()),
                        colon2_token: Default::default(),
                        tree: Box::new(UseTree::Glob(UseGlob {
                            star_token: Default::default(),
                        })),
                    }),
                ]
                .into_iter()
                .collect(),
            })),
        }),
        semi_token: Default::default(),
    }));

    let module = elysian_module(&elysian);

    for def in &module.struct_definitions {
        items.push(Item::Struct(ItemStruct {
            attrs: vec![Attribute {
                pound_token: Default::default(),
                style: AttrStyle::Outer,
                bracket_token: Default::default(),
                meta: Meta::List(MetaList {
                    path: Path {
                        leading_colon: None,
                        segments: [PathSegment {
                            ident: Ident::new("derive", Span::call_site()),
                            arguments: Default::default(),
                        }]
                        .into_iter()
                        .collect(),
                    },
                    delimiter: syn::MacroDelimiter::Paren(Default::default()),
                    tokens: quote!(Debug, Default, Copy, Clone),
                }),
            }],
            vis: if def.public {
                Visibility::Public(Default::default())
            } else {
                Visibility::Inherited
            },
            struct_token: Default::default(),
            ident: Ident::new(def.id.name(), Span::call_site()),
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
                        ident: Some(Ident::new(field.prop.name(), Span::call_site())),
                        colon_token: Default::default(),
                        ty: Type::Path(TypePath {
                            qself: None,
                            path: Ident::new(field.prop.ty().name(), Span::call_site()).into(),
                        }),
                    })
                    .collect(),
            }),
            semi_token: None,
        }));
    }

    for def in &module.function_definitions {
        items.push(Item::Fn(syn::ItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: Ident::new(def.id.name(), Span::call_site()),
                generics: Generics {
                    lt_token: None,
                    params: Default::default(),
                    gt_token: None,
                    where_clause: None,
                },
                paren_token: Default::default(),
                inputs: def
                    .inputs
                    .iter()
                    .map(|input| {
                        FnArg::Typed(PatType {
                            attrs: vec![],
                            pat: Box::new(Pat::Ident(PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: if input.mutable {
                                    Some(Default::default())
                                } else {
                                    None
                                },
                                ident: Ident::new(input.prop.name(), Span::call_site()).into(),
                                subpat: None,
                            })),
                            colon_token: Default::default(),
                            ty: Box::new(Type::Path(TypePath {
                                qself: None,
                                path: Ident::new(input.prop.ty().name(), Span::call_site()).into(),
                            })),
                        })
                    })
                    .collect(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Ident::new(def.output.id.name(), Span::call_site()).into(),
                    })),
                ),
            },
            block: Box::new(Block {
                brace_token: Default::default(),
                stmts: def.block.0.iter().map(stmt_to_syn).collect(),
            }),
        }))
    }

    items.push(Item::Fn(ItemFn {
        attrs: vec![],
        vis: Visibility::Public(Default::default()),
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new("shape", Span::call_site()),
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
                            ident: Ident::new(CONTEXT.id().name(), Span::call_site()),
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
                            ident: Ident::new(CONTEXT_STRUCT.id.name(), Span::call_site()),
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
                    path: Ident::new(CONTEXT_STRUCT.id.name(), Span::call_site()).into(),
                })),
            ),
        },
        block: Box::new(block_to_syn(&module.entry_point.block)),
    }));

    File {
        shebang: None,
        attrs,
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
                            &path.iter().map(|prop| prop.name()).collect::<String>(),
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
    }
}

fn expr_to_syn(expr: &IrExpr<f32, Vec2>) -> Expr {
    match expr {
        crate::ir::ast::Expr::Literal(v) => match v {
            crate::ir::ast::Value::Boolean(b) => Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Bool(LitBool {
                    value: *b,
                    span: Span::call_site(),
                }),
            }),
            crate::ir::ast::Value::Number(n) => Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Float(LitFloat::new(&(n.to_string() + &"f32"), Span::call_site())),
            }),
            crate::ir::ast::Value::Vector(v) => Expr::Call(ExprCall {
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
            crate::ir::ast::Value::Struct(_) => {
                unimplemented!()
            }
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
            path: Ident::new(structure.id.name(), Span::call_site()).into(),
            brace_token: Default::default(),
            fields: fields
                .iter()
                .map(|(prop, expr)| FieldValue {
                    attrs: vec![],
                    member: syn::Member::Named(Ident::new(prop.name(), Span::call_site())),
                    colon_token: Some(Default::default()),
                    expr: expr_to_syn(expr),
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
                        IrExpr::Normalize(_) => "normalize",
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

fn path_to_syn(path: &Vec<Property>) -> Expr {
    let mut iter = path.iter();

    let base = Expr::Path(ExprPath {
        attrs: vec![],
        qself: None,
        path: Ident::new(iter.next().expect("Empty path").name(), Span::call_site()).into(),
    });

    if path.len() == 1 {
        base
    } else {
        iter.fold(base, |acc, next| {
            Expr::Field(ExprField {
                attrs: vec![],
                base: Box::new(acc),
                dot_token: Default::default(),
                member: syn::Member::Named(Ident::new(next.name(), Span::call_site())),
            })
        })
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::glam::Vec2;

    use crate::elysian::{
        alias::Ring,
        attribute::Attribute,
        combinator::{Blend, Boolean, Combinator},
        expr::IntoLiteral,
        Elysian,
        Field::*,
        IntoAlias, IntoCombine,
    };

    use super::elysian_to_syn;

    #[test]
    fn test_syn_out() {
        let shape: Elysian<f32, Vec2> = [
            [Point
                .field()
                .elongate(Vec2::X.literal(), false)
                .isosurface(0.5.literal())
                .translate(Vec2::X.literal())]
            .combine([
                Combinator::Boolean(Boolean::Union),
                Combinator::Blend(Blend::SmoothUnion {
                    attr: Attribute::Distance,
                    k: 0.2.literal(),
                }),
            ]),
            Ring {
                radius: 0.8.literal(),
                width: 0.1.literal(),
            }
            .alias(),
        ]
        .combine([
            Combinator::Boolean(Boolean::Subtraction),
            Combinator::Blend(Blend::SmoothSubtraction {
                attr: Attribute::Distance,
                k: 0.2.literal(),
            }),
        ]);

        let foo = elysian_to_syn::<f32, Vec2>(&shape);
        let foo = prettyplease::unparse(&foo);
        panic!("\n\n{foo:}\n");
    }
}

pub mod foo {
    #![allow(unused_parens)]
    use rust_gpu_bridge::{glam::*, *};
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Context {
        pub position: Vec2,
        pub time: f32,
        pub distance: f32,
        pub gradient: Vec2,
        pub uv: Vec2,
        pub tangent: Vec2,
        pub color: Vec2,
        pub light: f32,
        pub support: Vec2,
        pub error: f32,
        pub num: f32,
        pub vect: Vec2,
    }
    #[derive(Debug, Default, Copy, Clone)]
    struct CombineContext {
        left: Context,
        right: Context,
        out: Context,
    }
    fn elongate(vect: Vec2, mut context: Context) -> Context {
        context.position = (context.position
            - (vect.normalize()
                * context
                    .position
                    .dot(vect.normalize())
                    .max(-vect.length())
                    .min(vect.length())));
        return context;
    }
    fn isosurface(num: f32, mut context: Context) -> Context {
        context.distance = (context.distance - num);
        return context;
    }
    fn manifold(mut context: Context) -> Context {
        let num = context.distance;
        context.distance = num.abs();
        context.gradient = (context.gradient * num.sign());
        return context;
    }
    fn point(mut context: Context) -> Context {
        context.distance = context.position.length();
        context.gradient = context.position.normalize();
        return context;
    }
    fn smooth_subtraction(k: f32, mut combine_context: CombineContext) -> CombineContext {
        let num = ((1f32 / 2f32)
            - (((1f32 / 2f32) * (combine_context.right.distance + combine_context.left.distance))
                / k))
            .max(0f32)
            .min(1f32);
        combine_context.out.distance = combine_context
            .left
            .distance
            .mix(-combine_context.right.distance, num);
        combine_context.out.distance = (combine_context.out.distance + ((k * num) * (1f32 - num)));
        return combine_context;
    }
    fn smooth_union(k: f32, mut combine_context: CombineContext) -> CombineContext {
        let num = ((1f32 / 2f32)
            + (((1f32 / 2f32) * (combine_context.right.distance - combine_context.left.distance))
                / k))
            .max(0f32)
            .min(1f32);
        combine_context.out.distance = combine_context
            .right
            .distance
            .mix(combine_context.left.distance, num);
        combine_context.out.distance = (combine_context.out.distance - ((k * num) * (1f32 - num)));
        return combine_context;
    }
    fn subtraction(mut combine_context: CombineContext) -> CombineContext {
        combine_context.out = combine_context.right;
        combine_context.out.distance = -combine_context.out.distance;
        if (combine_context.left.distance > combine_context.out.distance) {
            combine_context.out = combine_context.left;
        }
        return combine_context;
    }
    fn union(mut combine_context: CombineContext) -> CombineContext {
        if (combine_context.left.distance < combine_context.right.distance) {
            combine_context.out = combine_context.left;
        } else {
            combine_context.out = combine_context.right;
        }
        return combine_context;
    }
    pub fn shape(context: Context) -> Context {
        let combine_context = (CombineContext {
            out: isosurface(0.5f32, elongate(Vec2::new(1f32, 0f32), point(context))),
            ..Default::default()
        });
        let combine_context = (CombineContext {
            left: combine_context.out,
            right: isosurface(0.5f32, elongate(Vec2::new(0f32, 1f32), point(context))),
            ..Default::default()
        });
        let combine_context = smooth_union(0.2f32, union(combine_context));
        let combine_context = (CombineContext {
            left: combine_context.out,
            right: isosurface(0.5f32, elongate(Vec2::new(1f32, 1f32), point(context))),
            ..Default::default()
        });
        let combine_context = smooth_union(0.2f32, union(combine_context));
        let combine_context = (CombineContext {
            left: combine_context.out,
            right: isosurface(0.5f32, elongate(Vec2::new(1f32, -1f32), point(context))),
            ..Default::default()
        });
        let combine_context = smooth_union(0.2f32, union(combine_context));
        let combine_context = (CombineContext {
            left: combine_context.out,
            right: isosurface(0.1f32, manifold(isosurface(0.8f32, point(context)))),
            ..Default::default()
        });
        let combine_context = smooth_subtraction(0.2f32, subtraction(combine_context));
        return combine_context.out;
    }
}

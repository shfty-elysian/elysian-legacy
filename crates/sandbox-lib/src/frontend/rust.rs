use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use rust_gpu_bridge::glam::Vec2;

use crate::ir::{
    ast::{Block, Expr, Property, Stmt, Struct, Value},
    module::{FunctionDefinition, Type},
};

pub trait Tokenize {
    fn tokenize(&self) -> TokenStream;
}

impl Tokenize for [f32; 2] {
    fn tokenize(&self) -> TokenStream {
        let a = self[0];
        let b = self[1];
        quote!([#a, #b])
    }
}

impl Tokenize for Vec2 {
    fn tokenize(&self) -> TokenStream {
        let a = self.x;
        let b = self.y;
        quote!(Vec2::new(#a, #b))
    }
}

impl<N, V> ToTokens for Block<N, V>
where
    N: ToTokens,
    V: Tokenize,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let inner = self.0.iter().fold(quote!(), |acc, next| quote!(#acc #next));
        tokens.extend(quote!({ #inner }));
    }
}

impl<N, V> ToTokens for Stmt<N, V>
where
    N: ToTokens,
    V: Tokenize,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Stmt::Block(block) => quote!(#block),
            Stmt::Write { path, expr } => quote!(#(#path).* = #expr;),
            Stmt::IfElse {
                cond,
                then,
                otherwise,
            } => quote!(if #cond { #then } else { #otherwise }),
            Stmt::Nop => quote!(),
            Stmt::Output(prop) => quote!(return #prop),
        })
    }
}

impl<N, V> ToTokens for Expr<N, V>
where
    N: ToTokens,
    V: Tokenize,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            crate::ir::ast::Expr::Literal(value) => quote!(#value),
            crate::ir::ast::Expr::Read(context) => quote!(#(#context).*),
            crate::ir::ast::Expr::Construct(structure, expr) => {
                let structure = Ident::new(structure, Span::call_site());
                let (keys, values): (Vec<_>, Vec<_>) =
                    expr.iter().map(|(a, b)| (a.clone(), b.clone())).unzip();

                quote!(#structure {
                    #(#keys: #values,)*
                    ..Default::default()
                })
            }
            crate::ir::ast::Expr::Call { function, args } => {
                let function = Ident::new(function, Span::call_site());
                let args = args
                    .into_iter()
                    .map(|arg| quote!(#arg,))
                    .collect::<TokenStream>();
                quote!(#function(#args))
            }
            crate::ir::ast::Expr::Add(lhs, rhs) => quote!((#lhs + #rhs)),
            crate::ir::ast::Expr::Sub(lhs, rhs) => quote!((#lhs - #rhs)),
            crate::ir::ast::Expr::Mul(lhs, rhs) => quote!((#lhs * #rhs)),
            crate::ir::ast::Expr::Div(lhs, rhs) => quote!((#lhs / #rhs)),
            crate::ir::ast::Expr::Min(lhs, rhs) => quote!(#lhs.min(#rhs)),
            crate::ir::ast::Expr::Max(lhs, rhs) => quote!(#lhs.max(#rhs)),
            crate::ir::ast::Expr::Mix(lhs, rhs, t) => quote!(#lhs.mix(#rhs, #t)),
            crate::ir::ast::Expr::Lt(lhs, rhs) => quote!((#lhs < #rhs)),
            crate::ir::ast::Expr::Gt(lhs, rhs) => quote!((#lhs > #rhs)),
            crate::ir::ast::Expr::Neg(t) => quote!((-#t)),
            crate::ir::ast::Expr::Abs(t) => quote!(#t.abs()),
            crate::ir::ast::Expr::Sign(t) => quote!(#t.sign()),
            crate::ir::ast::Expr::Length(t) => quote!(#t.length()),
            crate::ir::ast::Expr::Normalize(t) => quote!(#t.normalize()),
            crate::ir::ast::Expr::Dot(lhs, rhs) => quote!(#lhs.dot(#rhs)),
        })
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = Ident::new(self.name(), Span::call_site());
        tokens.extend(quote!(#ident))
    }
}

impl<N, V> ToTokens for Value<N, V>
where
    N: ToTokens,
    V: Tokenize,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Value::Boolean(b) => {
                if *b {
                    quote!(true)
                } else {
                    quote!(false)
                }
            }
            Value::Number(n) => quote!(#n),
            Value::Vector(v) => v.tokenize(),
            Value::Struct(s) => quote!(#s),
        })
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(
            match self {
                Type::Boolean => "bool",
                Type::Number => "f32",
                Type::Vector => "Vec2",
                Type::Struct(s) => s,
            },
            Span::call_site(),
        );

        tokens.extend(quote!(#name));
    }
}

impl<N, V> ToTokens for Struct<N, V>
where
    N: ToTokens,
    V: Tokenize,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (keys, values): (Vec<_>, Vec<_>) = self
            .members
            .iter()
            .map(|(a, b)| (a.clone(), b.clone()))
            .unzip();
        tokens.extend(quote!(Context {
            #(#keys: #values)*
        }))
    }
}

fn evaluate_function<N, V>(function: &FunctionDefinition<N, V>) -> TokenStream
where
    N: ToTokens,
    V: Tokenize,
{
    let name = Ident::new(function.name, Span::call_site());
    let public = if function.public {
        quote!(pub)
    } else {
        quote!()
    };

    let mut inputs = quote!();
    for (prop, def) in &function.inputs {
        let mutable = if def.mutable { quote!(mut) } else { quote!() };
        let ty = prop.ty();
        inputs = quote!(#inputs #mutable #prop: #ty,);
    }

    let output = &function.output;
    let output = quote!(#output);

    let block = &function.block;

    quote!(
        #public fn #name(#inputs) -> #output #block
    )
}

pub fn evaluate_module<N, V>(module: &crate::ir::module::Module<N, V>) -> TokenStream
where
    N: ToTokens,
    V: Tokenize,
{
    let mut tokens = quote!(
        #![allow(unused_parens)]
        use rust_gpu_bridge::{*, glam::*};
    );

    for def in &module.struct_definitions {
        let name = Ident::new(def.name, Span::call_site());
        let public = if def.public { quote!(pub) } else { quote!() };

        let members = def
            .fields
            .iter()
            .map(|(prop, def)| {
                let public = if def.public { quote!(pub) } else { quote!() };
                let ty = prop.ty();
                quote!(#public #prop: #ty,)
            })
            .collect::<TokenStream>();

        let tokens = &mut tokens;
        *tokens = quote!(
            #tokens

            #[derive(Debug, Default, Copy, Clone)]
            #public struct #name {
                #members
            }
        )
    }

    for function in &module.function_definitions {
        let f = evaluate_function(function);
        let tokens = &mut tokens;
        *tokens = quote! {
            #tokens
            #f
        };
    }

    let entry_point = evaluate_function(&module.entry_point);
    let tokens = quote!(
        #tokens

        #entry_point
    );

    tokens
}

#[cfg(test)]
mod test {
    use rust_gpu_bridge::glam::Vec2;

    use crate::{
        elysian::{
            alias::Ring,
            attribute::Attribute,
            combinator::{Blend, Boolean, Combinator},
            expand::Expand,
            expr::IntoLiteral,
            Elysian,
            Field::*,
            IntoAlias, IntoCombine,
        },
        frontend::rust::evaluate_module,
        ir::from_elysian::elysian_module,
    };

    #[test]
    fn test_rust_out() {
        let shape: Elysian<f32, Vec2> = [
            [
                Point
                    .field()
                    .elongate(Vec2::X.literal(), false)
                    .isosurface(0.5.literal()),
                Point
                    .field()
                    .elongate(Vec2::Y.literal(), false)
                    .isosurface(0.5.literal()),
                Point
                    .field()
                    .elongate(Vec2::ONE.literal(), false)
                    .isosurface(0.5.literal()),
                Point
                    .field()
                    .elongate(Vec2::new(1.0, -1.0).literal(), false)
                    .isosurface(0.5.literal()),
            ]
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
        let shape = shape.expand();
        let module = elysian_module(&shape);
        let res = evaluate_module(&module);
        panic!("{res:}")
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
                    .max((-vect.length()))
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
            .mix((-combine_context.right.distance), num);
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
        combine_context.out.distance = (-combine_context.out.distance);
        if (combine_context.left.distance > combine_context.out.distance) {
            combine_context.out = combine_context.left;
        } else {
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
        let combine_context = CombineContext {
            out: isosurface(0.5f32, elongate(Vec2::new(1f32, 0f32), point(context))),
            ..Default::default()
        };
        let combine_context = CombineContext {
            left: combine_context.out,
            right: isosurface(0.5f32, elongate(Vec2::new(0f32, 1f32), point(context))),
            ..Default::default()
        };
        let combine_context = smooth_union(0.2f32, union(combine_context));
        let combine_context = CombineContext {
            left: combine_context.out,
            right: isosurface(0.5f32, elongate(Vec2::new(1f32, 1f32), point(context))),
            ..Default::default()
        };
        let combine_context = smooth_union(0.2f32, union(combine_context));
        let combine_context = CombineContext {
            left: combine_context.out,
            right: isosurface(0.5f32, elongate(Vec2::new(1f32, -1f32), point(context))),
            ..Default::default()
        };
        let combine_context = smooth_union(0.2f32, union(combine_context));
        let combine_context = CombineContext {
            left: combine_context.out,
            right: isosurface(0.1f32, manifold(isosurface(0.8f32, point(context)))),
            ..Default::default()
        };
        let combine_context = smooth_subtraction(0.2f32, subtraction(combine_context));
        return combine_context.out;
    }
}

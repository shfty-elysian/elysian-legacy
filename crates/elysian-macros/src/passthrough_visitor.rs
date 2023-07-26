use proc_macro::Ident;
use syn::{
    parse_quote, visit_mut::VisitMut, Expr, ExprCall, ExprLit, ExprPath, ExprStruct, FieldValue,
    Lit, Member, Path, PathSegment,
};

pub struct PassthroughVisitor {
    pub passthrough_idents: Vec<Ident>,
}

impl VisitMut for PassthroughVisitor {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        match i.clone() {
            Expr::Call(ExprCall { func, args, .. }) => match *func {
                Expr::Path(ExprPath { ref path, .. }) => {
                    if let Some(seg) = path.segments.last() {
                        if seg.ident.to_string() == "PropertyIdentifier" {
                            let Expr::Struct(ExprStruct { fields, .. }) =
                            args.first().unwrap() else {
                            panic!("PropertyIdentifier inner is not a Struct");
                        };

                            let name_field = fields
                                .iter()
                                .find(|FieldValue { member, .. }| {
                                    let Member::Named(named) = member else {
                                    panic!("Members must be named");
                                };

                                    named.to_string() == "name"
                                })
                                .expect("No name field");

                            match &name_field.expr {
                                Expr::Call(ExprCall { args, .. }) => {
                                    let arg = args.first().expect("No arg");
                                    match arg {
                                        Expr::Lit(ExprLit {
                                            lit: Lit::Str(name),
                                            ..
                                        }) => {
                                            if let Some(_) = self
                                                .passthrough_idents
                                                .iter()
                                                .find(|ident| ident.to_string() == name.value())
                                            {
                                                *i = Expr::Path(ExprPath {
                                                    attrs: vec![],
                                                    qself: None,
                                                    path: Path {
                                                        leading_colon: None,
                                                        segments: [PathSegment {
                                                            ident: syn::Ident::new(
                                                                &name.value(),
                                                                proc_macro2::Span::call_site(),
                                                            ),
                                                            arguments: Default::default(),
                                                        }]
                                                        .into_iter()
                                                        .collect(),
                                                    },
                                                });

                                                *i = parse_quote!(#i.clone());
                                            }
                                        }
                                        _ => unimplemented!(),
                                    }
                                }
                                _ => unimplemented!(),
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }

        syn::visit_mut::visit_expr_mut(self, i)
    }
}

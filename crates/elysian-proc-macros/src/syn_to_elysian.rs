use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Block as SynBlock, Expr as SynExpr, ExprAssign, ExprBlock, ExprCall, ExprField, ExprIf,
    ExprLoop, ExprMethodCall, ExprParen, ExprPath, ExprReturn, ExprStruct, FieldValue, Ident,
    Member, Pat, PatIdent, Stmt as SynStmt,
};

pub struct SynToElysian {
    passthrough_idents: Vec<Ident>,
}

impl SynToElysian {
    pub fn new<T: IntoIterator<Item = proc_macro::Ident>>(idents: T) -> Self {
        SynToElysian {
            passthrough_idents: idents
                .into_iter()
                .map(|ident| syn::Ident::new(&ident.to_string(), ident.span().into()))
                .collect(),
        }
    }
}

#[cfg(feature = "internal")]
macro_rules ! quote_crate {
    ($($p:tt)*) => {
        quote!(elysian_core::$($p)*)
    }
}

#[cfg(not(feature = "internal"))]
macro_rules ! quote_crate {
    ($($p:tt)*) => {
        quote!(elysian::core::$($p)*)
    }
}

impl SynToElysian {
    fn expr_path_inner(&self, expr: &SynExpr) -> Vec<Ident> {
        match expr {
            SynExpr::Path(ExprPath { path, .. }) => {
                path.segments.iter().map(|seg| seg.ident.clone()).collect()
            }
            SynExpr::Field(ExprField { base, member, .. }) => {
                let base = self.expr_path_inner(base);

                let Member::Named(member) = member else {
                    panic!("Members must be named")
                };

                base.into_iter().chain([member.clone()]).collect()
            }
            t => unimplemented!("expr_path_inner: {t:#?}"),
        }
    }

    pub fn parse_block(&self, block: &SynBlock) -> TokenStream {
        let stmts: Vec<_> = block
            .stmts
            .iter()
            .map(|stmt| self.parse_stmt(stmt))
            .collect();

        quote_crate!(ir::ast::Block([#(#stmts),*].into_iter().collect()))
    }

    pub fn parse_stmt(&self, stmt: &SynStmt) -> TokenStream {
        match stmt {
            SynStmt::Local(local) => {
                let ident = if let Pat::Ident(PatIdent { ident, .. }) = &local.pat {
                    ident
                } else {
                    panic!("Invalid pattern")
                };

                let prop = quote!(#ident.clone().into());

                let expr = self.parse_expr(
                    &local
                        .init
                        .as_ref()
                        .expect("Uninitialized variables are unsupported")
                        .expr,
                );

                quote_crate!(ir::ast::Stmt::Bind { prop: #prop, expr: #expr })
            }
            SynStmt::Expr(expr, _) => self.parse_expr(expr),
            t => unimplemented!("read_stmt: {t:#?}"),
        }
    }

    pub fn parse_expr(&self, expr: &SynExpr) -> TokenStream {
        match expr {
            SynExpr::Assign(ExprAssign { left, right, .. }) => {
                let expr = self.parse_expr(right);

                let path = self.expr_path_inner(left);

                if self.passthrough_idents.contains(&path[0]) {
                    quote_crate! {
                        ir::ast::Stmt::Write {
                            path: (#(#path.clone()),*).clone(),
                            expr: #expr,
                        }
                    }
                } else {
                    quote_crate! {
                        ir::ast::Stmt::Write {
                            path: [#(#path.clone().into()),*].into_iter().collect(),
                            expr: #expr,
                        }
                    }
                }
            }
            SynExpr::Binary(b) => {
                let left = self.parse_expr(&b.left);
                let right = self.parse_expr(&b.right);

                match b.op {
                    syn::BinOp::Add(_) => quote!((#left + #right)),
                    syn::BinOp::Sub(_) => quote!((#left - #right)),
                    syn::BinOp::Mul(_) => quote!((#left * #right)),
                    syn::BinOp::Div(_) => quote!((#left / #right)),
                    syn::BinOp::Eq(_) => quote!(#left.eq(#right)),
                    syn::BinOp::Ne(_) => quote!(#left.ne(#right)),
                    syn::BinOp::Lt(_) => quote!(#left.lt(#right)),
                    syn::BinOp::Le(_) => quote!((#left.lt(#right) | #left.eq(#right))),
                    syn::BinOp::Gt(_) => quote!(#left.gt(#right)),
                    syn::BinOp::Ge(_) => quote!((#left.gt(#right) | #left.eq(#right))),
                    syn::BinOp::And(_) => quote!((#left.and(#right))),
                    syn::BinOp::Or(_) => quote!((#left.or(#right))),
                    t => unimplemented!("SynExpr::Binary: {t:#?}"),
                }
            }
            SynExpr::Block(ExprBlock { block, .. }) => self.parse_block(block),
            SynExpr::Break(_) => quote_crate!(ir::ast::Stmt::Break),
            SynExpr::Call(ExprCall { func, args, .. }) => {
                let args: Vec<_> = args.into_iter().map(|arg| self.parse_expr(arg)).collect();
                let function = match &**func {
                    SynExpr::Path(ExprPath { path, .. }) => {
                        let ident = path.get_ident().expect("Function path is not an ident");
                        quote!(#ident.clone().into())
                    }
                    t => unimplemented!("SynExpr::Call: {t:#?}"),
                };

                quote_crate!(ir::ast::Expr::Call { function: #function, args: [#(#args),*].into_iter().collect() })
            }
            SynExpr::Field(_) | SynExpr::Path(_) => {
                let path = self.expr_path_inner(expr);

                if self.passthrough_idents.contains(&path[0]) {
                    quote!((#(#path),*).clone())
                } else {
                    quote_crate!(ir::ast::Expr::Read([#(#path.clone().into()),*].into_iter().collect()))
                }
            }
            SynExpr::If(ExprIf {
                cond,
                then_branch,
                else_branch,
                ..
            }) => {
                let cond = self.parse_expr(cond);

                let then = self.parse_block(then_branch);
                let then = quote_crate!(ir::ast::Stmt::Block(#then));

                let otherwise = match else_branch {
                    Some((_, otherwise)) => {
                        let otherwise = self.parse_expr(otherwise);
                        let otherwise = quote_crate!(ir::ast::Stmt::Block(#otherwise));
                        quote!(Some(Box::new(#otherwise)))
                    }
                    None => quote!(None),
                };

                quote_crate!(ir::ast::Stmt::If {
                    cond: #cond,
                    then: Box::new(#then),
                    otherwise: #otherwise,
                })
            }
            SynExpr::Lit(l) => match &l.lit {
                syn::Lit::Int(i) => match i.suffix() {
                    "" | "u" | "u8" | "u16" | "u32" | "u64" => {
                        let i: u64 = i.base10_parse().expect("Failed to parse UInt");
                        let num = quote_crate!(ir::ast::Number::UInt(#i));
                        let val = quote_crate!(ir::ast::Value::Number(#num));
                        quote_crate!(ir::ast::Expr::Literal(#val))
                    }
                    "i" | "i8" | "i16" | "i32" | "i64" => {
                        let i: i64 = i.base10_parse().expect("Failed to parse SInt");
                        let num = quote_crate!(ir::ast::Number::SInt(#i));
                        let val = quote_crate!(ir::ast::Value::Number(#num));
                        quote_crate!(ir::ast::Expr::Literal(#val))
                    }
                    _ => panic!("Unrecognized suffix"),
                },
                syn::Lit::Float(f) => {
                    let f: f64 = f.base10_parse::<f64>().expect("Failed to parse float");
                    let num = quote_crate!(ir::ast::Number::Float(#f));
                    let val = quote_crate!(ir::ast::Value::Number(#num));
                    quote_crate!(ir::ast::Expr::Literal(#val))
                }
                syn::Lit::Bool(b) => {
                    let b = b.value;
                    quote_crate!(ir::ast::Expr::Literal(Value::Boolean(#b)))
                }
                t => unimplemented!("SynExpr::Lit: {t:#?}"),
            },
            SynExpr::Loop(ExprLoop { body, .. }) => {
                let block = self.parse_block(body);
                quote_crate!(ir::ast::Stmt::Loop {
                    stmt: Box::new(Stmt::Block(#block)),
                })
            }
            SynExpr::Paren(ExprParen { expr, .. }) => self.parse_expr(expr),
            SynExpr::Return(ExprReturn { expr, .. }) => {
                let expr = self.parse_expr(expr.as_ref().expect("No expression for return"));
                quote_crate!(ir::ast::Stmt::Output(#expr))
            }
            SynExpr::Struct(ExprStruct { path, fields, .. }) => {
                let ident = path.get_ident().expect("Struct path is not an ident");

                let structure = quote_crate!(ir::module::StructIdentifier(#ident.clone()));

                let (keys, values): (Vec<_>, Vec<_>) = fields
                    .into_iter()
                    .map(|FieldValue { member, expr, .. }| {
                        let Member::Named(member) = member else {
                            panic!("Unnamed members are not supported");
                        };

                        let key = quote!(#member.clone().into());

                        (key, self.parse_expr(expr))
                    })
                    .unzip();

                quote_crate!(ir::ast::Expr::Struct(#structure, [#((#keys, #values)),*].into_iter().collect()))
            }
            SynExpr::Unary(u) => match u.op {
                syn::UnOp::Neg(_) => {
                    let expr = self.parse_expr(&u.expr);
                    quote!((-#expr))
                }
                t => unimplemented!("SynExpr::Unary: {t:#?}"),
            },
            SynExpr::MethodCall(ExprMethodCall {
                receiver,
                method,
                args,
                ..
            }) => {
                let receiver = self.parse_expr(receiver);

                match args.len() {
                    0 => match method.to_string().as_str() {
                        "abs" => quote!(#receiver.abs()),
                        "length" => quote!(#receiver.length()),
                        "normalize" => quote!(#receiver.normalize()),
                        "sign" => quote!(#receiver.sign()),
                        "acos" => quote!(#receiver.acos()),
                        "atan" => quote!(#receiver.atan()),
                        _ => panic!("Unsupported method"),
                    },
                    1 => {
                        let rhs = self.parse_expr(&args[0]);

                        match method.to_string().as_str() {
                            "min" => quote!(#receiver.min(#rhs)),
                            "max" => quote!(#receiver.max(#rhs)),
                            "dot" => quote!(#receiver.dot(#rhs)),
                            "atan2" => quote!(#receiver.atan2(#rhs)),
                            _ => panic!("Unsupported method"),
                        }
                    }
                    2 => {
                        let arg0 = self.parse_expr(&args[0]);

                        let arg1 = self.parse_expr(&args[1]);

                        match method.to_string().as_str() {
                            "mix" => quote!(#receiver.mix(#arg0, #arg1)),
                            _ => panic!("Unsupported method"),
                        }
                    }
                    _ => panic!("Unsupported method"),
                }
            }
            _ => unimplemented!("parse_expr"),
        }
    }
}

use std::borrow::Cow;

use elysian_core::ir::{
    ast::{
        Block as ElysianBlock, Expr as ElysianExpr, Identifier, Number, Stmt as ElysianStmt, Value,
    },
    module::{FunctionIdentifier, PropertyIdentifier, StructIdentifier},
};
use quote::ToTokens;
use syn::{
    Block as SynBlock, Expr as SynExpr, ExprAssign, ExprBlock, ExprCall, ExprField, ExprIf,
    ExprLoop, ExprMethodCall, ExprPath, ExprReturn, ExprStruct, FieldValue, Member, Pat, PatIdent,
    Stmt as SynStmt,
};

#[derive(Debug, Default, Clone)]
pub struct RustReader {
    block_stack: Vec<ElysianBlock>,
    stmt_stack: Vec<ElysianStmt>,
    expr_stack: Vec<ElysianExpr>,
}

impl RustReader {
    fn block_mut(&mut self) -> &mut ElysianBlock {
        self.block_stack.last_mut().expect("Not inside a block")
    }

    pub fn push_block(&mut self, block: ElysianBlock) {
        self.block_stack.push(block);
    }

    pub fn pop_block(&mut self) -> ElysianBlock {
        self.block_stack.pop().expect("No block")
    }

    pub fn push_stmt(&mut self, stmt: ElysianStmt) {
        self.stmt_stack.push(stmt);
    }

    pub fn pop_stmt(&mut self) -> ElysianStmt {
        self.stmt_stack.pop().expect("Not inside a block")
    }

    pub fn push_expr(&mut self, expr: ElysianExpr) {
        self.expr_stack.push(expr);
    }

    pub fn pop_expr(&mut self) -> ElysianExpr {
        self.expr_stack.pop().expect("Not inside a statement")
    }

    pub fn read_block(&mut self, block: &SynBlock) {
        self.push_block(Default::default());
        for stmt in &block.stmts {
            self.read_stmt(stmt);
            let stmt = self.pop_stmt();
            self.block_mut().0.push(stmt);
        }
    }

    pub fn read_stmt(&mut self, stmt: &SynStmt) {
        match stmt {
            SynStmt::Local(local) => {
                self.read_expr(
                    &local
                        .init
                        .as_ref()
                        .expect("Uninitialized variables are unsupported")
                        .expr,
                );

                let expr = self.pop_expr();

                self.push_stmt(ElysianStmt::Bind {
                    prop: {
                        if let Pat::Ident(PatIdent { ident, .. }) = &local.pat {
                            PropertyIdentifier(Identifier {
                                name: Cow::Owned(ident.to_string()),
                                uuid: Default::default(),
                            })
                        } else {
                            panic!("Invalid pattern")
                        }
                    },
                    expr,
                })
            }
            SynStmt::Expr(expr, _) => {
                self.read_expr(expr);
            }
            _ => unimplemented!(),
        }
    }

    fn expr_path(&self, expr: &SynExpr) -> Vec<PropertyIdentifier> {
        match expr {
            SynExpr::Path(ExprPath { path, .. }) => path
                .segments
                .iter()
                .map(|seg| {
                    PropertyIdentifier(Identifier {
                        name: Cow::Owned(seg.ident.to_string()),
                        uuid: Default::default(),
                    })
                })
                .collect(),
            SynExpr::Field(ExprField { base, member, .. }) => {
                let base = self.expr_path(base);

                let Member::Named(member) = member else {
                panic!("Members must be named")
            };

                base.into_iter()
                    .chain([PropertyIdentifier(Identifier {
                        name: Cow::Owned(member.to_string()),
                        uuid: Default::default(),
                    })])
                    .collect()
            }
            _ => unimplemented!(),
        }
    }

    pub fn read_expr(&mut self, expr: &SynExpr) {
        match &expr {
            SynExpr::Assign(ExprAssign { left, right, .. }) => {
                let path = match &**left {
                    SynExpr::Field(ExprField { base, member, .. }) => {
                        let base = match &**base {
                            SynExpr::Path(ExprPath { path, .. }) => {
                                path.segments.iter().map(|seg| {
                                    PropertyIdentifier(Identifier {
                                        name: Cow::Owned(seg.ident.to_string()),
                                        uuid: Default::default(),
                                    })
                                })
                            }
                            _ => unimplemented!(),
                        };

                        let Member::Named(member) = member else {
                            panic!("Members must be named")
                        };

                        base.chain([PropertyIdentifier(Identifier {
                            name: Cow::Owned(member.to_string()),
                            uuid: Default::default(),
                        })])
                        .collect()
                    }
                    _ => panic!("Unrecognized assign expression {}", left.to_token_stream()),
                };

                self.read_expr(right);
                let expr = self.pop_expr();

                let stmt = ElysianStmt::Write { path, expr };
                self.push_stmt(stmt);
            }
            SynExpr::Binary(b) => {
                self.read_expr(&b.left);
                let left = self.pop_expr();

                self.read_expr(&b.right);
                let right = self.pop_expr();

                let expr = match b.op {
                    syn::BinOp::Add(_) => left + right,
                    syn::BinOp::Sub(_) => left - right,
                    syn::BinOp::Mul(_) => left * right,
                    syn::BinOp::Div(_) => left / right,
                    syn::BinOp::Lt(_) => left.lt(right),
                    syn::BinOp::Gt(_) => left.gt(right),
                    _ => unimplemented!(),
                };
                self.push_expr(expr);
            }
            SynExpr::Block(ExprBlock { block, .. }) => {
                self.read_block(block);
            }
            SynExpr::Break(_) => {
                self.push_stmt(ElysianStmt::Break);
            }
            SynExpr::Call(ExprCall { func, args, .. }) => {
                let args = args
                    .into_iter()
                    .map(|arg| {
                        self.read_expr(arg);
                        self.pop_expr()
                    })
                    .collect();
                let expr = ElysianExpr::Call {
                    function: match &**func {
                        SynExpr::Path(ExprPath { path, .. }) => FunctionIdentifier(Identifier {
                            name: Cow::Owned(
                                path.get_ident()
                                    .expect("Function path is not an ident")
                                    .to_string(),
                            ),
                            uuid: Default::default(),
                        }),
                        _ => unimplemented!(),
                    },
                    args,
                };

                self.push_expr(expr);
            }
            SynExpr::Field(ExprField { .. }) => {
                let path = self.expr_path(expr);
                self.push_expr(ElysianExpr::Read(path));
            }
            SynExpr::If(ExprIf {
                cond,
                then_branch,
                else_branch,
                ..
            }) => {
                self.read_block(then_branch);
                let then = self.pop_block();
                let then = ElysianStmt::Block(then);

                let otherwise = else_branch.as_ref().map(|(_, else_branch)| {
                    self.read_expr(else_branch);
                    Box::new(self.pop_stmt())
                });

                let stmt = ElysianStmt::If {
                    cond: {
                        self.read_expr(cond);
                        self.pop_expr()
                    },
                    then: Box::new(then),
                    otherwise,
                };
                self.push_stmt(stmt);
            }
            SynExpr::Lit(l) => {
                let expr = match &l.lit {
                    syn::Lit::Int(i) => match i.suffix() {
                        "" | "u" | "u8" | "u16" | "u32" | "u64" => {
                            ElysianExpr::Literal(Value::Number(Number::UInt(
                                i.base10_parse().expect("Failed to parse UInt"),
                            )))
                        }
                        "i" | "i8" | "i16" | "i32" | "i64" => ElysianExpr::Literal(Value::Number(
                            Number::SInt(i.base10_parse().expect("Failed to parse SInt")),
                        )),
                        _ => panic!("Unrecognized suffix"),
                    },
                    syn::Lit::Float(f) => ElysianExpr::Literal(Value::Number(Number::Float(
                        f.base10_parse::<f64>().expect("Failed to parse float"),
                    ))),
                    syn::Lit::Bool(b) => ElysianExpr::Literal(Value::Boolean(b.value)),
                    _ => unimplemented!(),
                };
                self.push_expr(expr);
            }
            SynExpr::Loop(ExprLoop { body, .. }) => {
                self.read_block(body);
                let block = self.pop_block();
                let stmt = ElysianStmt::Loop {
                    stmt: Box::new(ElysianStmt::Block(block)),
                };
                self.push_stmt(stmt);
            }
            SynExpr::Paren(_) => todo!(),
            SynExpr::Return(ExprReturn { expr, .. }) => {
                self.read_expr(expr.as_ref().expect("No expression for return"));
                let expr = self.pop_expr();
                self.push_stmt(ElysianStmt::Output(expr));
            }
            SynExpr::Struct(ExprStruct { path, fields, .. }) => {
                let structure = StructIdentifier(Identifier {
                    name: Cow::Owned(
                        path.get_ident()
                            .expect("Struct path is not an ident")
                            .to_string(),
                    ),
                    uuid: Default::default(),
                });
                let fields = fields
                    .into_iter()
                    .map(|FieldValue { member, expr, .. }| {
                        let Member::Named(member) = member else {
                        panic!("Unnamed members are not supported");
                    };

                        self.read_expr(expr);
                        let expr = self.pop_expr();

                        (
                            PropertyIdentifier(Identifier {
                                name: Cow::Owned(member.to_string()),
                                uuid: Default::default(),
                            }),
                            expr,
                        )
                    })
                    .collect();
                let expr = ElysianExpr::Struct(structure, fields);
                self.push_expr(expr);
            }
            SynExpr::Unary(u) => match u.op {
                syn::UnOp::Neg(_) => {
                    self.read_expr(&u.expr);
                    let expr = self.pop_expr();
                    self.push_expr(-expr);
                }
                _ => unimplemented!(),
            },
            SynExpr::Path(ExprPath { path, .. }) => {
                let expr = ElysianExpr::Read(
                    path.segments
                        .iter()
                        .map(|seg| {
                            PropertyIdentifier(Identifier {
                                name: Cow::Owned(seg.ident.to_string()),
                                uuid: Default::default(),
                            })
                        })
                        .collect(),
                );
                self.push_expr(expr);
            }
            SynExpr::MethodCall(ExprMethodCall {
                receiver, method, ..
            }) => {
                self.read_expr(receiver);
                let receiver = self.pop_expr();
                self.push_expr(match method.to_string().as_str() {
                    "abs" => receiver.abs(),
                    "length" => receiver.length(),
                    "normalize" => receiver.normalize(),
                    "sign" => receiver.sign(),
                    _ => panic!("Unsupported method"),
                });
            }
            _ => unimplemented!(),
        }
    }
}

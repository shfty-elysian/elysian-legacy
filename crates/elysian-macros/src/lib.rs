mod passthrough_visitor;
mod rust_reader;

use passthrough_visitor::PassthroughVisitor;
use proc_macro::{Group, Ident, Punct, Spacing, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_quote, visit_mut::VisitMut, Block, Expr, Stmt};

use crate::rust_reader::RustReader;

fn passthrough_idents(tokens: TokenStream) -> (TokenStream, Vec<Ident>) {
    let mut iter = tokens.clone().into_iter();

    let mut tokens = vec![];
    let mut passthroughs = vec![];
    while let Some(token) = iter.next() {
        match token {
            TokenTree::Punct(ref p) => {
                if p.as_char() == '#' {
                    let TokenTree::Ident(ident) = iter.next().unwrap() else {
                        panic!("Ident does not follow hash");
                    };

                    tokens.push(TokenTree::Ident(ident.clone()));
                    passthroughs.push(ident);
                } else {
                    tokens.push(token);
                }
            }
            TokenTree::Group(ref g) => {
                let (toks, ids) = passthrough_idents(g.stream());
                tokens.push(TokenTree::Group(Group::new(g.delimiter(), toks)));
                passthroughs.extend(ids);
            }
            _ => tokens.push(token),
        }
    }

    let tokens: TokenStream = tokens.into_iter().collect();

    (tokens, passthroughs)
}

#[proc_macro]
pub fn elysian_expr(tokens: TokenStream) -> TokenStream {
    let (tokens, passthrough_idents) = passthrough_idents(tokens);

    let expr: Expr = syn::parse(tokens).expect("Failed to parse Expr");

    let mut reader = RustReader::default();
    reader.read_expr(&expr);
    let block = reader.pop_expr();

    let mut tokens: Expr = parse_quote!({#block});
    PassthroughVisitor { passthrough_idents }.visit_expr_mut(&mut tokens);

    quote!(#tokens).into()
}

#[proc_macro]
pub fn elysian_stmt(tokens: TokenStream) -> TokenStream {
    let tokens = tokens
        .into_iter()
        .chain([TokenTree::Punct(Punct::new(
            ';',
            proc_macro::Spacing::Alone,
        ))])
        .collect();
    let (tokens, passthrough_idents) = passthrough_idents(tokens);

    let stmt: Stmt = syn::parse(tokens).expect("Failed to parse Stmt");

    let mut reader = RustReader::default();
    reader.read_stmt(&stmt);
    let block = reader.pop_stmt();

    let mut tokens: Stmt = parse_quote!({#block});
    PassthroughVisitor { passthrough_idents }.visit_stmt_mut(&mut tokens);

    quote!(#tokens).into()
}

#[proc_macro]
pub fn elysian_block(tokens: TokenStream) -> TokenStream {
    let tokens = TokenStream::from(TokenTree::Group(Group::new(
        proc_macro::Delimiter::Brace,
        tokens,
    )));

    let (tokens, passthrough_idents) = passthrough_idents(tokens);

    let block: Block = syn::parse(tokens).expect("Failed to parse Block");

    let mut reader = RustReader::default();
    reader.read_block(&block);
    let block = reader.pop_block();

    let mut tokens: Block = parse_quote!({#block});
    PassthroughVisitor { passthrough_idents }.visit_block_mut(&mut tokens);

    quote!(#tokens).into()
}

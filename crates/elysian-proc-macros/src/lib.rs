mod syn_to_elysian;

use proc_macro::{Group, Ident, Punct, TokenStream, TokenTree};
use quote::quote;
use syn::{Block, Expr, Stmt};
use syn_to_elysian::SynToElysian;

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
    let expr = SynToElysian::new(passthrough_idents).parse_expr(&expr);
    //panic!("{expr:}");

    quote!(#expr).into()
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
    let stmt = SynToElysian::new(passthrough_idents).parse_stmt(&stmt);
    //panic!("{stmt:}");

    quote!(#stmt).into()
}

#[proc_macro]
pub fn elysian_block(tokens: TokenStream) -> TokenStream {
    let tokens = TokenStream::from(TokenTree::Group(Group::new(
        proc_macro::Delimiter::Brace,
        tokens,
    )));

    let (tokens, passthrough_idents) = passthrough_idents(tokens);

    let block: Block = syn::parse(tokens).expect("Failed to parse Block");
    let block = SynToElysian::new(passthrough_idents).parse_block(&block);
    //panic!("{block:}");

    quote!(#block).into()
}

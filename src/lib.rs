use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Block, Ident, LitInt, Token};


struct RepeatInput {
    n: LitInt,
    _comma0: Token![,],
    ident: Ident,
    _comma1: Token![,],
    block: Box<Block>,
}

impl Parse for RepeatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RepeatInput {
            n: input.parse()?,
            _comma0: input.parse()?,
            ident: input.parse()?,
            _comma1: input.parse()?,
            block: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn array(input: TokenStream) -> TokenStream {
    let RepeatInput {
        n, ident, block, ..
    } = parse_macro_input!(input as RepeatInput);
    let count = n.base10_parse::<usize>().expect("N must be an integer");

    let blocks = expand(count, ident, block);
    let output = quote! {
         [#(#blocks),*]
    };

    output.into()
}
#[proc_macro]
pub fn tuple(input: TokenStream) -> TokenStream {
    let RepeatInput {
        n, ident, block, ..
    } = parse_macro_input!(input as RepeatInput);
    let count = n.base10_parse::<usize>().expect("N must be an integer");

    let blocks = expand(count, ident, block);
    let output = quote! {
        ( #(#blocks),* )
    };

    output.into()
}

fn expand(count: usize, ident: Ident, block: Box<Block>) -> Vec<proc_macro2::TokenStream> {
    let mut blocks = Vec::new();
    for i in 0..count {
        blocks.push(quote! {
            {
                #[allow(unused)]
                let #ident = #i;
                #[allow(unused_braces)]
                #block
            }
        });
    }
    blocks
}

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

/// This macro generates an array with repeated blocks of code.
/// 
/// # Arguments
/// 
/// * `n` - The number of times to repeat the block.
/// * `ident` - The identifier to use within the block.
/// * `block` - The block of code to repeat.
/// 
/// # Example
/// 
/// ```rust
/// use op_proc::array;
/// let arr = [4, 5, 6, 7, 8];
/// let narr = array!(3, i, {
///    arr[i + 1] + 1
/// });
/// assert_eq!(narr, [6, 7, 8]);
/// ```
/// 
/// This will expand to:
/// 
/// ```rust,ignore
/// [
///     {
///         let i = 0;
///         arr[i + 1] + 1
///     },
///     {
///         let i = 1;
///         arr[i + 1] + 1
///     },
///     {
///         let i = 2;
///         arr[i + 1] + 1
///     }
/// ]
/// ```
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
/// This macro generates a tuple with repeated blocks of code.
/// 
/// # Arguments
/// 
/// * `n` - The number of times to repeat the block.
/// * `ident` - The identifier to use within the block.
/// * `block` - The block of code to repeat.
/// 
/// # Example
/// 
/// ```rust
/// use op_proc::tuple;
/// let arr = [4, 5, 6, 7, 8];
/// let ntuple = tuple!(3, i, {
///    arr[i + 1] + 1
/// });
/// assert_eq!(ntuple, (6, 7, 8));
/// ```
/// 
/// This will expand to:
/// 
/// ```rust,ignore
/// (
///     {
///         let i = 0;
///         arr[i + 1] + 1
///     },
///     {
///         let i = 1;
///         arr[i + 1] + 1
///     },
///     {
///         let i = 2;
///         arr[i + 1] + 1
///     }
/// )
/// ```
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

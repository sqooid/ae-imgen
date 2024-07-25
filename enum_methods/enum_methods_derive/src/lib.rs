use std::error::Error;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Variant};

type Res<T> = Result<T, Box<dyn Error>>;

#[proc_macro_derive(EnumMethods)]
pub fn enum_methods_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);
    println!("{:?}", &ast);
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(x) => &x.variants,
        _ => todo!(),
    };
    let mut inner_count = 0;
    let inner = match &variants.first().unwrap().fields {
        syn::Fields::Unnamed(x) => {
            x.unnamed.iter().for_each(|_| inner_count += 1);
            match &x.unnamed.first().unwrap().ty {
                syn::Type::Path(x) => &x.path.segments.first().unwrap().ident,
                _ => todo!(),
            }
        }
        _ => todo!(),
    };
    let variant_names = variants.iter().map(|x| &x.ident).collect::<Vec<_>>();
    let args = (0..inner_count)
        .map(|x| format_ident!("x{}", x.to_string()))
        .collect::<Vec<_>>();
    let args_tokens = quote! {
        #(#args),*
    };

    // Build the trait implementation
    let gen = quote! {
        impl EnumMethods<#inner> for #name {
            fn get_arg<S: Into<usize>>(&self, i: S) -> &#inner {
                let index: usize = i.into();
                match self {
                    #(Self::#variant_names(#args_tokens) => [#args_tokens][index]),*
                }
            }
            fn set_arg<S: Into<usize>>(&self, i: S, value: #inner) {
                todo!();
            }
        }
    };

    gen.into()
}

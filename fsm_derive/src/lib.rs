use proc_macro;
use proc_macro2::{self, TokenStream, Ident};
use quote::quote;
use syn::{self, DeriveInput, Data, DataEnum, Variant};

#[proc_macro_derive(Fsm)]
pub fn derive_fsm(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(
        main_fsm(
            TokenStream::from(input)
        )
    )
}

fn main_fsm(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();
    let name = &input.ident;
    let name_str = name.to_string();

    let imp_tok: TokenStream = "use std::string::String;".parse().unwrap();
    let e = gen(&input);

    let out: TokenStream = quote! {
        #imp_tok
        impl #name {
            #e

            fn name(&self) -> &std::primitive::str {
                &#name_str
            }
        }
    };

    out
}

fn gen(input: &DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    if let Data::Enum(DataEnum {ref variants, ..}) = input.data {
        let y: Vec<&Ident> = variants
            .iter()
            .map(|x| &x.ident)
            .collect();

        quote! {
            fn bla(&self) -> String {
                let a = vec![#(::std::stringify!(#y)),*];
                let b = match &self {
                    #(#enum_name::#y =>
                      ::std::stringify!(#y)),*
                };
                ::std::string::String::from(b)
            }
        }
    } else {
        panic!("not an enum");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_good_case() {
        let input = quote! {
            enum Test {
                A,
                B,
                C,
            }
        };
        let _ = main_fsm(input);
    }

    #[test]
    fn complex_enums() {
        let input = quote! {
            enum Test {
                A(u32),
                B,
                C {name: String, age: u32},
            }
        };
        let _ = main_fsm(input);
    }

    #[test]
    #[should_panic(expected = "not an enum")]
    fn test_panic_on_struct() {
        let input = quote! {
            struct Test {
                name: String,
                age: u32,
            }
        };
        let _ = main_fsm(input);
    }
}

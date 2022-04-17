use proc_macro;
use proc_macro2::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataEnum};

#[proc_macro_derive(Fsm)]
pub fn derive_fsm(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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

    proc_macro::TokenStream::from(out)
}

fn gen(input: &DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    if let Data::Enum(DataEnum {ref variants, ..}) = input.data {
        let l: Vec<String> = variants.iter().map(|x| x.ident.to_string()).collect();
        let text = l.join(", ");
        quote! {
            fn bla(&self) -> String {
                let a = vec![#(#l),*];
                a.join(", ")
                //"bla"
            }
        }
    } else {
        panic!("not an enum");
    }
}

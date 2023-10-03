use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{DeriveInput, Ident};

pub struct Prefix {
    value: Option<String>,
}

impl Prefix {
    pub(crate) fn parse(input: &DeriveInput) -> Self {
        let value = input
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("prefix"))
            .map(|prefix| {
                let literal: Literal = prefix
                    .parse_args()
                    .expect("prefix value must be a simple &str");
                let prefix = literal.to_string();
                if !prefix.starts_with('"') {
                    panic!("prefix value must be a simple &str");
                }
                if !prefix.starts_with("\"/") {
                    panic!("prefix value must be a simple &str starting with '/'");
                }
                prefix[1..prefix.len() - 1].to_string()
            });
        Self { value }
    }

    pub(crate) fn into_token_stream(
        self,
        remaining_path: &Ident,
        after_prefix: &Ident,
    ) -> TokenStream {
        match self.value {
            None => quote! {
                let #after_prefix = #remaining_path;
            },
            Some(prefix) => {
                quote! {
                    let prefix = #prefix;
                    if #remaining_path.len() < prefix.len() {
                        return None;
                    }
                    if &#remaining_path[..prefix.len()] != prefix {
                        return None;
                    }
                    let #after_prefix = &#remaining_path[prefix.len()..];
                }
            }
        }
    }
}

mod capture_fields;
mod method_tag;
mod path;
mod prefix;
mod variant_type;

use crate::prefix::Prefix;
use crate::variant_type::VariantType;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, GenericParam, Token};

fn extend_decoding_params(
    params: &Punctuated<GenericParam, Token![,]>,
) -> proc_macro2::TokenStream {
    let lifetimes: Vec<_> = params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Lifetime(l) => Some(&l.lifetime),
            GenericParam::Type(_) => None,
            GenericParam::Const(_) => None,
        })
        .collect();
    if lifetimes.is_empty() {
        quote! { 'de, #params }
    } else {
        quote! { 'de: #(#lifetimes)+*, #params }
    }
}

#[proc_macro_derive(
    FrontLine,
    attributes(
        get, post, put, delete, head, options, connect, trace, patch, flatten, prefix
    )
)]
pub fn front_line_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    match &derive_input.data {
        Data::Enum(data) => {
            let name = &derive_input.ident;
            let params = &derive_input.generics.params;
            let extended_params = extend_decoding_params(params);
            let prefix = Prefix::parse(&derive_input);
            let variants = VariantType::parse(data);
            let method = format_ident!("method");
            let remaining_path = format_ident!("remaining_path");
            let after_prefix = format_ident!("after_prefix");
            let prefix_matcher = prefix.into_token_stream(&remaining_path, &after_prefix);
            let variant_matchers: Vec<_> = variants
                .into_iter()
                .enumerate()
                .map(|(variant_offset, variant)| {
                    variant.into_token_stream(name, variant_offset, &method, &after_prefix)
                })
                .collect();
            let router = quote! {
                impl<#extended_params> front_line_router::Router<'de> for #name<#params> {
                      fn handle_parsed(
                        #method: front_line_router::Method,
                        #remaining_path: &'de str
                    ) -> Option<Self> {
                        #prefix_matcher
                        #(
                            #variant_matchers
                        )*
                        None
                    }
                }
            };
            router.into()
        }
        _ => panic!("FrontLine derive macro only works on enum types"),
    }
}

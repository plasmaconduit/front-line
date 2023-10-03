use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Ident, Lifetime, Type, Variant};

pub(crate) struct CaptureFields<'a> {
    fields: Vec<(&'a Ident, &'a Type)>,
}

impl<'a> CaptureFields<'a> {
    pub(crate) fn new(variant: &'a Variant) -> Self {
        let fields: Vec<_> = match &variant.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref().map(|i| (i, &f.ty)))
                .collect(),
            _ => vec![],
        };
        Self { fields }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub(crate) fn matches_all_idents(&self, idents: &[Ident]) -> bool {
        self.fields.iter().all(|(f, _)| idents.contains(f))
    }

    pub(crate) fn make_token_stream(
        &self,
        parent: &Ident,
        variant: &Ident,
        variant_offset: usize,
        path_offset: usize,
        path_block: &Lifetime,
    ) -> TokenStream {
        let mut conversions = Vec::new();
        let base_offset = format_ident!("_{variant_offset}_{path_offset}");
        for (ident, ty) in self.fields.iter() {
            let capture = format_ident!("capture{base_offset}_{ident}");
            let parsed = format_ident!("parsed{base_offset}_{ident}");
            let converted = format_ident!("converted{base_offset}_{ident}");
            let conversion = quote! {
                let #parsed: Option<#ty> = front_line_router::FromRoute::parse_path_variable(&#capture);
                if #parsed.is_none() {
                    break #path_block;
                }
                let #converted = #parsed.unwrap();
            };
            conversions.push(conversion);
        }
        let mut initializers = Vec::new();
        for (ident, _) in self.fields.iter() {
            let converted = format_ident!("converted{base_offset}_{ident}");
            let initializer = quote! {
                #ident: #converted,
            };
            initializers.push(initializer);
        }
        quote! {
            #(
                #conversions
            )*
            return Some(#parent::#variant {
                #(
                    #initializers
                )*
            });
        }
    }
}

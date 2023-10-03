use crate::capture_fields::CaptureFields;
use crate::method_tag::MethodTag;
use crate::path::Path;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::{DataEnum, Fields, Ident, Type};

pub(crate) enum VariantType<'a> {
    LeafVariant(&'a Ident, CaptureFields<'a>, Vec<(Path, MethodTag)>),
    FlattenedVariant(&'a Ident, &'a Type),
}

impl<'a> VariantType<'a> {
    pub(crate) fn parse(data: &DataEnum) -> Vec<VariantType> {
        let mut leaves = parse_leaf_variants(data);
        let flattened = parse_flattened_variants(data);
        leaves.extend(flattened);
        leaves
    }

    pub(crate) fn into_token_stream(
        self,
        parent: &Ident,
        variant_offset: usize,
        parsed_method: &Ident,
        after_prefix: &Ident,
    ) -> TokenStream {
        match self {
            VariantType::LeafVariant(variant, fields, routes) => {
                let paths_by_method = into_paths_by_method(routes);
                let mut by_method_matchers = Vec::with_capacity(paths_by_method.len());
                for (method, paths) in paths_by_method {
                    let path_blocks: Vec<_> = paths
                        .into_iter()
                        .enumerate()
                        .map(|(path_offset, path)| {
                            path.into_token_stream(
                                parent,
                                variant,
                                &fields,
                                variant_offset,
                                path_offset,
                                after_prefix,
                            )
                        })
                        .collect();
                    let method_ident = method.to_ident();
                    let by_method_matcher = quote! {
                        if #parsed_method == front_line_router::Method::#method_ident {
                            #(
                                #path_blocks
                            )*
                        }
                    };
                    by_method_matchers.push(by_method_matcher);
                }
                quote! {
                    #(
                      #by_method_matchers
                    )*
                }
            }
            VariantType::FlattenedVariant(variant, ty) => {
                let maybe_matched = format_ident!("maybe_{variant_offset}");
                let matched = format_ident!("matched_{variant_offset}");
                quote! {
                    let #maybe_matched = <#ty>::handle_parsed(#parsed_method, #after_prefix);
                    if let Some(#matched) = #maybe_matched {
                        return Some(#parent::#variant(#matched));
                    }
                }
            }
        }
    }
}

fn parse_leaf_variants(data: &DataEnum) -> Vec<VariantType> {
    data.variants
        .iter()
        .filter_map(|variant| {
            let fields = CaptureFields::new(variant);
            let paths_and_methods: Vec<_> = variant
                .attrs
                .iter()
                .filter_map(|attr| attr.path().get_ident().map(|ident| (attr, ident)))
                .filter_map(|(attr, ident)| {
                    MethodTag::try_from(ident).ok().map(|method| (attr, method))
                })
                .map(|(attr, method)| {
                    let literal: Literal = attr.parse_args().unwrap_or_else(|_| {
                        panic!("path argument for {} must be a simple &str", variant.ident);
                    });
                    let path_literal = literal.to_string();
                    if !path_literal.starts_with("\"") {
                        panic!("path argument for {} must be a simple &str", variant.ident);
                    }
                    if !path_literal.starts_with("\"/") {
                        panic!(
                            "path argument for {} must be a simple &str that starts with '/'",
                            variant.ident
                        );
                    }
                    let path = Path::parse(&path_literal[1..path_literal.len() - 1]);
                    let path_variables = path.variables();
                    if variant.fields.len() != path_variables.len() {
                        panic!(
                            "path variables for {} must match the named fields of the variant",
                            variant.ident
                        );
                    }
                    if path_variables.is_empty() {
                        if !matches!(variant.fields, Fields::Unit) {
                            panic!(
                                "{} doesn't define path vars, so it must be a unit variant",
                                variant.ident
                            );
                        }
                    } else {
                        if fields.is_empty() {
                            panic!(
                                "{} defines path variables, so it must have named fields",
                                variant.ident
                            );
                        }
                        let all_fields_match = fields.matches_all_idents(path_variables.as_slice());
                        if !all_fields_match {
                            panic!(
                                "variant {} named fields and path variables must match",
                                variant.ident
                            );
                        }
                    }
                    (path, method)
                })
                .collect();
            if paths_and_methods.is_empty() {
                None
            } else {
                Some(VariantType::LeafVariant(
                    &variant.ident,
                    fields,
                    paths_and_methods,
                ))
            }
        })
        .collect()
}

fn parse_flattened_variants(data: &DataEnum) -> Vec<VariantType> {
    data.variants
        .iter()
        .filter_map(|variant| {
            let flatten = Ident::new("flatten", Span::call_site());
            variant
                .attrs
                .iter()
                .filter_map(|attr| attr.path().get_ident())
                .find(|ident| *ident == &flatten)
                .map(|_| match &variant.fields {
                    Fields::Unnamed(fields) => {
                        if fields.unnamed.len() != 1 {
                            panic!(
                                "{} #[flattened], so it must have exactly 1 parameter",
                                variant.ident
                            );
                        }
                        let only_field = fields.unnamed.first().unwrap();
                        VariantType::FlattenedVariant(&variant.ident, &only_field.ty)
                    }
                    _ => {
                        panic!(
                            "{} is #[flattened], so it must have a single unnamed parameter",
                            variant.ident
                        );
                    }
                })
        })
        .collect()
}

fn into_paths_by_method(routes: Vec<(Path, MethodTag)>) -> BTreeMap<MethodTag, Vec<Path>> {
    let mut paths_by_method: BTreeMap<MethodTag, Vec<Path>> = BTreeMap::new();
    for (path, method) in routes.into_iter() {
        paths_by_method.entry(method).or_default().push(path);
    }
    paths_by_method
}

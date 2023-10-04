use crate::capture_fields::CaptureFields;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;
use syn::{Ident, Lifetime};

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Path {
    pub parts: Vec<PathParts>,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum PathParts {
    Segment(String),
    Variable(String),
}

impl Path {
    pub(crate) fn parse(path: &str) -> Path {
        let re = Regex::new(r"\{(?P<var>[^}]+)}|(?P<seg>/[^{]+/?)").unwrap();
        let mut parts = Vec::new();

        for cap in re.captures_iter(path) {
            if let Some(m) = cap.name("var") {
                parts.push(PathParts::Variable(m.as_str().to_string()));
            } else if let Some(m) = cap.name("seg") {
                parts.push(PathParts::Segment(m.as_str().to_string()));
            }
        }

        Path { parts }
    }

    pub(crate) fn variables(&self) -> Vec<Ident> {
        self.parts
            .iter()
            .filter_map(|part| match part {
                PathParts::Variable(variable) => {
                    Some(Ident::new(variable.as_str(), Span::call_site()))
                }
                PathParts::Segment(_) => None,
            })
            .collect()
    }

    pub(crate) fn into_token_stream(
        self,
        parent: &Ident,
        variant: &Ident,
        fields: &CaptureFields,
        variant_offset: usize,
        path_offset: usize,
        after_prefix: &Ident,
    ) -> TokenStream {
        let base_offset = format_ident!("_{variant_offset}_{path_offset}");
        let path_block_name = format!("'block{base_offset}");
        let path_block = Lifetime::new(path_block_name.as_str(), Span::call_site());
        let mut segment_matchers = Vec::new();
        let mut last_slice = after_prefix.clone();
        for (s_offset, part) in self.parts.into_iter().enumerate() {
            let segment_offset = format_ident!("{base_offset}_{s_offset}");
            let next_slice = format_ident!("after{segment_offset}");
            let segment_matcher = match part {
                PathParts::Segment(segment) => {
                    let segment_str = format_ident!("str{segment_offset}");
                    let segment_len = format_ident!("len{segment_offset}");
                    quote! {
                        let #segment_str = #segment;
                        let #segment_len = #segment_str.len();
                        if #last_slice.len() < #segment_len || &#last_slice[..#segment_len] != #segment_str {
                            break #path_block;
                        }
                        let #next_slice = &#last_slice[#segment_len..];
                    }
                }
                PathParts::Variable(variable) => {
                    let end = format_ident!("end{segment_offset}");
                    let capture = format_ident!("capture{base_offset}_{variable}");
                    quote! {
                        let #end = front_line::memchr::memchr(b'/', #last_slice.as_bytes()).unwrap_or(#last_slice.len());
                        let #capture = &#last_slice[..#end];
                        let #next_slice = &#last_slice[#end..];
                    }
                }
            };
            last_slice = next_slice;
            segment_matchers.push(segment_matcher);
        }
        let conversions =
            fields.make_token_stream(parent, variant, variant_offset, path_offset, &path_block);
        quote! {
            #path_block: {
                #(
                    #segment_matchers
                )*
                if !#last_slice.is_empty() && #last_slice != "/" {
                    break #path_block;
                }
                #conversions
            }
        }
    }
}

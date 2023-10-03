use proc_macro2::Span;
use syn::Ident;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub(crate) enum MethodTag {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
    Patch,
}

impl MethodTag {
    pub fn to_ident(&self) -> Ident {
        match self {
            MethodTag::Get => Ident::new("Get", Span::call_site()),
            MethodTag::Post => Ident::new("Post", Span::call_site()),
            MethodTag::Put => Ident::new("Put", Span::call_site()),
            MethodTag::Delete => Ident::new("Delete", Span::call_site()),
            MethodTag::Head => Ident::new("Head", Span::call_site()),
            MethodTag::Options => Ident::new("Options", Span::call_site()),
            MethodTag::Connect => Ident::new("Connect", Span::call_site()),
            MethodTag::Trace => Ident::new("Trace", Span::call_site()),
            MethodTag::Patch => Ident::new("Patch", Span::call_site()),
        }
    }
}

impl TryFrom<&Ident> for MethodTag {
    type Error = ();

    fn try_from(ident: &Ident) -> Result<Self, Self::Error> {
        if *ident == Ident::new("get", Span::call_site()) {
            return Ok(MethodTag::Get);
        }
        if *ident == Ident::new("post", Span::call_site()) {
            return Ok(MethodTag::Post);
        }
        if *ident == Ident::new("put", Span::call_site()) {
            return Ok(MethodTag::Put);
        }
        if *ident == Ident::new("delete", Span::call_site()) {
            return Ok(MethodTag::Delete);
        }
        if *ident == Ident::new("head", Span::call_site()) {
            return Ok(MethodTag::Head);
        }
        if *ident == Ident::new("options", Span::call_site()) {
            return Ok(MethodTag::Options);
        }
        if *ident == Ident::new("connect", Span::call_site()) {
            return Ok(MethodTag::Connect);
        }
        if *ident == Ident::new("trace", Span::call_site()) {
            return Ok(MethodTag::Trace);
        }
        if *ident == Ident::new("patch", Span::call_site()) {
            return Ok(MethodTag::Patch);
        }
        Err(())
    }
}

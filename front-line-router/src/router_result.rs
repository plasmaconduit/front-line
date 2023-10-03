use crate::HttpVersion;

/// Represents the result of routing an HTTP request.
///
/// This structure captures essential parts of an HTTP request like the route, query parameters,
/// the HTTP version, and the remaining head and body sections.
///
/// The generic type `T` allows flexibility in how routes are represented. It could be a simple
/// enum, a string, or any other type that best captures the essence of routes for a specific
/// application.
#[derive(PartialEq, Debug)]
pub struct RouterResult<'a, T> {
    /// The identified route from the HTTP request.
    ///
    /// This could be `None` if no matching route was found.
    pub route: Option<T>,

    /// The query string from the HTTP request.
    ///
    /// Represents the part after the `?` in the URL.
    pub query: &'a str,

    /// The version of the HTTP protocol used in the request.
    pub version: HttpVersion,

    /// The remaining parts of the HTTP request, typically the headers and the body.
    pub head_and_body: &'a [u8],
}

use crate::http_version::HttpVersion;
use crate::method::Method;
use crate::RouterResult;
use memchr::memmem;

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("the http request line was not valid")]
    InvalidRequestLine,
}

/// A trait that encapsulates routing logic for an HTTP request.
///
/// Implementers of this trait can specify custom logic to handle parsed method and path segments,
/// allowing for flexible routing mechanisms. However, for most common use cases, it's recommended
/// to use the `front_line::FrontLine` proc-macro to auto-generate `Router` instances for enums.
///
/// The provided `route` method processes an HTTP request byte slice, parsing its method, path, and
/// query components. If parsing is successful, it constructs a `RouterResult` that encapsulates
/// these parsed components.
pub trait Router<'de>: Sized {
    /// Handle the parsed method and path segment.
    ///
    /// Implementers can provide custom logic to identify routes based on the parsed method and
    /// path. This allows for the identification of specific application routes, endpoints, etc.
    ///
    /// # Arguments
    ///
    /// * `method` - The parsed HTTP method (e.g., GET, POST).
    /// * `remaining_path` - The parsed path segment from the HTTP request.
    ///
    /// # Returns
    ///
    /// Returns an instance of the implementing type if a route is identified. Otherwise,
    /// returns `None`.
    fn handle_parsed(method: Method, remaining_path: &'de str) -> Option<Self>;

    /// Parse and route an HTTP request.
    ///
    /// This method provides the core logic to process an HTTP request byte slice, extract its
    /// components, and identify a route if possible.
    ///
    /// # Arguments
    ///
    /// * `request` - The raw byte slice of the HTTP request.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `RouterResult` if routing is successful. If any parsing
    /// or validation errors occur, returns an `Error`.
    fn resolve(request: &'de [u8]) -> Result<RouterResult<'de, Self>, Error> {
        let end = memmem::find(request, b"\r\n\r\n").ok_or(Error::InvalidRequestLine)?;
        let request_line = &request[..end];
        let (method, after_method) =
            Method::parse(request_line).ok_or(Error::InvalidRequestLine)?;
        let full_path_end = memchr::memchr(b' ', after_method).ok_or(Error::InvalidRequestLine)?;
        let after_path = &after_method[full_path_end + 1..];
        let version = HttpVersion::parse(after_path).ok_or(Error::InvalidRequestLine)?;
        let full_path = &after_method[..full_path_end];
        let query_start = memchr::memchr(b'?', full_path).unwrap_or(full_path.len());
        let query_bytes = &full_path[full_path.len().min(query_start + 1)..];
        let query = std::str::from_utf8(query_bytes).map_err(|_| Error::InvalidRequestLine)?;
        let path_bytes = &full_path[..query_start];
        let path = std::str::from_utf8(path_bytes).map_err(|_| Error::InvalidRequestLine)?;
        let route = Self::handle_parsed(method, path);
        let head_and_body = &request[end + 4..];
        let result = RouterResult {
            route,
            query,
            version,
            head_and_body,
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[derive(PartialEq, Debug)]
    enum TestRoute {
        Test,
    }

    impl<'de> Router<'de> for TestRoute {
        fn handle_parsed(method: Method, remaining_path: &'de str) -> Option<Self> {
            match (method, remaining_path) {
                (Method::Get, "/test") => Some(TestRoute::Test),
                _ => None,
            }
        }
    }

    #[rstest]
    #[case(
        b"GET /test HTTP/1.1\r\n\r\nSome data",
        Ok(RouterResult {
            route: Some(TestRoute::Test),
            query: "",
            version: HttpVersion::OneOne,
            head_and_body: b"Some data",
        })
    )]
    #[case(
        b"GET /test?query=value HTTP/1.1\r\n\r\n",
        Ok(RouterResult {
            route: Some(TestRoute::Test),
            query: "query=value",
            version: HttpVersion::OneOne,
            head_and_body: b"",
        })
    )]
    #[case(
        b"GET /test HTTP/1.0\r\n\r\n",
        Ok(RouterResult {
            route: Some(TestRoute::Test),
            query: "",
            version: HttpVersion::OneZero,
            head_and_body: b"",
        })
    )]
    #[case(
        b"POST /test HTTP/1.1\r\n\r\n",
        Ok(RouterResult {
            route: None,
            query: "",
            version: HttpVersion::OneOne,
            head_and_body: b"",
        })
    )]
    #[case(
        b"GET /invalid HTTP/1.1\r\n\r\n",
        Ok(RouterResult {
            route: None,
            query: "",
            version: HttpVersion::OneOne,
            head_and_body: b"",
        })
    )]
    #[case(
        b"GET /invalid?key=value HTTP/1.1\r\n\r\n",
        Ok(RouterResult {
            route: None,
            query: "key=value",
            version: HttpVersion::OneOne,
            head_and_body: b"",
        })
    )]
    #[case(
        b"GET /invalid?key=value HTTP/1.1\r\n\r\nheader-section",
        Ok(RouterResult {
            route: None,
            query: "key=value",
            version: HttpVersion::OneOne,
            head_and_body: b"header-section",
        })
    )]
    #[case(b"GET /test HTT/1.1\r\n\r\n", Err(Error::InvalidRequestLine))]
    #[case(b"GET /test", Err(Error::InvalidRequestLine))]
    #[case(b"GET/test HTTP/1.1\r\n\r\n", Err(Error::InvalidRequestLine))]
    #[case(b"GET /test HTTP/1.1\r\nSome data", Err(Error::InvalidRequestLine))]
    fn test_route(
        #[case] input: &[u8],
        #[case] expected_result: Result<RouterResult<'_, TestRoute>, Error>,
    ) {
        let result = TestRoute::resolve(input);
        assert_eq!(result, expected_result);
    }
}

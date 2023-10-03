/// Represents HTTP methods.
///
/// These methods are tokens that indicate the desired action to be performed
/// on the identified resource.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Method {
    /// Represents the HTTP `GET` method.
    ///
    /// Used to retrieve data from a server.
    Get,

    /// Represents the HTTP `POST` method.
    ///
    /// Used to submit data to a server for processing.
    Post,

    /// Represents the HTTP `PUT` method.
    ///
    /// Used to update a resource or create a new resource if it does not exist.
    Put,

    /// Represents the HTTP `DELETE` method.
    ///
    /// Used to delete a resource.
    Delete,

    /// Represents the HTTP `HEAD` method.
    ///
    /// Used to retrieve metadata about a resource.
    Head,

    /// Represents the HTTP `OPTIONS` method.
    ///
    /// Used to describe the communication options for the target resource.
    Options,

    /// Represents the HTTP `CONNECT` method.
    ///
    /// Used to establish a network connection for a given URI.
    Connect,

    /// Represents the HTTP `TRACE` method.
    ///
    /// Used for diagnostic purposes.
    Trace,

    /// Represents the HTTP `PATCH` method.
    ///
    /// Used to apply partial modifications to a resource.
    Patch,
}

impl Method {
    /// Parse an HTTP request line to determine the method.
    ///
    /// This function will attempt to parse the provided request line slice and
    /// return the identified HTTP method and the remaining part of the request line.
    ///
    /// # Arguments
    ///
    /// * `request_line` - A byte slice containing the request line to parse.
    ///
    /// # Returns
    ///
    /// Returns `Some((Method, &[u8]))` if a valid HTTP method is found. Otherwise,
    /// returns `None`.
    pub fn parse(request_line: &[u8]) -> Option<(Self, &[u8])> {
        // method parsers are sorted by method length, and max length was calculated
        // from "[METHOD_NAME] / HTTP/1.1".len()
        if request_line.len() < 14 {
            return None;
        }
        if &request_line[..4] == b"GET " {
            return Some((Method::Get, &request_line[4..]));
        }
        if &request_line[..4] == b"PUT " {
            return Some((Method::Put, &request_line[4..]));
        }
        if request_line.len() < 15 {
            return None;
        }
        if &request_line[..5] == b"POST " {
            return Some((Method::Post, &request_line[5..]));
        }
        if &request_line[..5] == b"HEAD " {
            return Some((Method::Head, &request_line[5..]));
        }
        if request_line.len() < 16 {
            return None;
        }
        if &request_line[..6] == b"TRACE " {
            return Some((Method::Trace, &request_line[6..]));
        }
        if &request_line[..6] == b"PATCH " {
            return Some((Method::Patch, &request_line[6..]));
        }
        if request_line.len() < 17 {
            return None;
        }
        if &request_line[..7] == b"DELETE " {
            return Some((Method::Delete, &request_line[7..]));
        }
        if request_line.len() < 18 {
            return None;
        }
        if &request_line[..8] == b"OPTIONS " {
            return Some((Method::Options, &request_line[8..]));
        }
        if &request_line[..8] == b"CONNECT " {
            return Some((Method::Connect, &request_line[8..]));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Method;
    use rstest::rstest;

    #[rstest]
    #[case(b"GET / HTTP/1.1", Some((Method::Get, b"/ HTTP/1.1".as_slice())))]
    #[case(b"PUT / HTTP/1.1", Some((Method::Put, b"/ HTTP/1.1".as_slice())))]
    #[case(b"POST / HTTP/1.1", Some((Method::Post, b"/ HTTP/1.1".as_slice())))]
    #[case(b"HEAD / HTTP/1.1", Some((Method::Head, b"/ HTTP/1.1".as_slice())))]
    #[case(b"TRACE / HTTP/1.1", Some((Method::Trace, b"/ HTTP/1.1".as_slice())))]
    #[case(b"PATCH / HTTP/1.1", Some((Method::Patch, b"/ HTTP/1.1".as_slice())))]
    #[case(b"DELETE / HTTP/1.1", Some((Method::Delete, b"/ HTTP/1.1".as_slice())))]
    #[case(b"OPTIONS / HTTP/1.1", Some((Method::Options, b"/ HTTP/1.1".as_slice())))]
    #[case(b"CONNECT / HTTP/1.1", Some((Method::Connect, b"/ HTTP/1.1".as_slice())))]
    #[case(b"INVALIDMETHOD / HTTP/1.1", None)]
    fn test_parse_method(#[case] request: &[u8], #[case] expected: Option<(Method, &[u8])>) {
        assert_eq!(Method::parse(request), expected);
    }

    #[test]
    fn test_remaining_request_line() {
        let request = b"GET /foo/bar HTTP/1.1".as_slice();
        assert_eq!(
            Method::parse(request),
            Some((Method::Get, b"/foo/bar HTTP/1.1".as_slice()))
        );
    }

    #[rstest]
    #[case(b"GET/ HTTP/1.1")]
    #[case(b"PUT/ HTTP/1.1")]
    #[case(b"POST/ HTTP/1.1")]
    #[case(b"HEAD/ HTTP/1.1")]
    fn test_malformed_request(#[case] request: &[u8]) {
        assert_eq!(Method::parse(request), None);
    }

    #[rstest]
    #[case(b"GE")]
    #[case(b"POS")]
    #[case(b"TRAC")]
    #[case(b"DELET")]
    #[case(b"OPTION")]
    #[case(b"GET / HTTP/1.")]
    #[case(b"POST / HTTP/1.")]
    #[case(b"TRACE / HTTP/1.")]
    #[case(b"DELETE / HTTP/1.")]
    #[case(b"OPTIONS / HTTP/1.")]
    fn test_short_request(#[case] request: &[u8]) {
        assert_eq!(Method::parse(request), None);
    }
}

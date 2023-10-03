/// Represents versions of the HTTP protocol.
///
/// Currently supports only HTTP/1.0 and HTTP/1.1.
#[derive(Eq, PartialEq, Debug)]
pub enum HttpVersion {
    /// Represents the HTTP/1.0 version.
    OneZero,

    /// Represents the HTTP/1.1 version.
    ///
    /// This version includes features like persistent connections and chunked transfer-coding.
    OneOne,
}

impl HttpVersion {
    /// Parse an HTTP version from the given request line slice.
    ///
    /// This function will attempt to parse the provided slice and return the identified
    /// HTTP version if recognized.
    ///
    /// # Arguments
    ///
    /// * `remaining_request_line` - A byte slice containing the part of the request
    ///   line representing the HTTP version.
    ///
    /// # Returns
    ///
    /// Returns `Some(HttpVersion)` if a valid HTTP version is identified. Otherwise,
    /// returns `None`.
    pub fn parse(remaining_request_line: &[u8]) -> Option<Self> {
        if remaining_request_line == b"HTTP/1.1" {
            return Some(HttpVersion::OneOne);
        }
        if remaining_request_line == b"HTTP/1.0" {
            return Some(HttpVersion::OneZero);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::HttpVersion;
    use rstest::rstest;

    #[rstest]
    #[case(b"HTTP/1.1", Some(HttpVersion::OneOne))]
    #[case(b"HTTP/1.0", Some(HttpVersion::OneZero))]
    #[case(b"HTTP/0.9", None)]
    #[case(b"HTTP/2.0", None)]
    #[case(b"HTTPS/1.1", None)]
    #[case(b"HTTP/1.10", None)]
    #[case(b"HTTP/1.", None)]
    fn test_http_version_parsing(#[case] input: &[u8], #[case] expected: Option<HttpVersion>) {
        assert_eq!(HttpVersion::parse(input), expected);
    }
}

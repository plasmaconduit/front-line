/// A trait to enable zero-copy parsing from route paths.
///
/// This trait is designed as an alternative to `FromStr` to support both zero-copy and copy parsing.
/// It's especially useful for HTTP route parsing where parts of the route can be efficiently parsed
/// without the need to allocate memory for every segment of the route. This can lead to performance
/// benefits, especially in web applications where route parsing happens frequently.
///
/// Implement this trait for types that need to be parsed from route paths.
///
/// # Examples
///
/// ```
/// // Suppose you have a route segment "/user/42"
/// // and you want to directly parse "42" into a UserId type.
///
/// use front_line_router::FromRoute;
///
/// struct UserId(u32);
///
/// impl<'de> FromRoute<'de> for UserId {
///     fn parse_path_variable(slice: &'de str) -> Option<Self> {
///         slice.parse().map(UserId).ok()
///     }
/// }
/// ```
///
/// Zero-copy example:
///
/// ```
/// // For cases where the exact slice from the route path can be directly used,
/// // the `FromRoute` trait allows efficient zero-copy parsing.
///
/// use front_line_router::FromRoute;
///
/// struct UserName<'a>(&'a str);
///
/// impl<'de> FromRoute<'de> for UserName<'de> {
///     fn parse_path_variable(slice: &'de str) -> Option<Self> {
///         if slice.is_empty() {
///             None
///         } else {
///             Some(UserName(slice))
///         }
///     }
/// }
///
/// // Given a route segment "/user/alice", "alice" can be parsed directly as UserName
/// // without creating a new String.
/// ```
pub trait FromRoute<'de>: Sized {
    /// Parses a value from a route segment.
    ///
    /// # Arguments
    ///
    /// * `slice` - A segment of a route, typically a part between slashes in a URL.
    ///
    /// # Returns
    ///
    /// Returns `Some(T)` if the segment can be successfully parsed into type `T`. Otherwise,
    /// returns `None`.
    fn parse_path_variable(slice: &'de str) -> Option<Self>;
}

impl<'de> FromRoute<'de> for bool {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        match slice {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}

impl<'de> FromRoute<'de> for u8 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for u16 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for u32 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for u64 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for u128 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for usize {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for i8 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for i16 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for i32 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for i64 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for i128 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for isize {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for f32 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for f64 {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        slice.parse().ok()
    }
}

impl<'de> FromRoute<'de> for &'de [u8] {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        Some(slice.as_bytes())
    }
}

impl<'de> FromRoute<'de> for &'de str {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        Some(slice)
    }
}

impl<'de> FromRoute<'de> for String {
    fn parse_path_variable(slice: &'de str) -> Option<Self> {
        Some(slice.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::FromRoute;
    use rstest::rstest;

    #[rstest]
    #[case("true", Some(true))]
    #[case("false", Some(false))]
    #[case("other", None)]
    fn test_bool(#[case] input: &str, #[case] expected: Option<bool>) {
        assert_eq!(bool::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("255", Some(255))]
    #[case("-1", None)]
    #[case("256", None)]
    fn test_u8(#[case] input: &str, #[case] expected: Option<u8>) {
        assert_eq!(u8::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("65535", Some(65535))]
    #[case("-1", None)]
    #[case("65536", None)]
    fn test_u16(#[case] input: &str, #[case] expected: Option<u16>) {
        assert_eq!(u16::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("4294967295", Some(4294967295))]
    #[case("-1", None)]
    #[case("4294967296", None)]
    fn test_u32(#[case] input: &str, #[case] expected: Option<u32>) {
        assert_eq!(u32::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("18446744073709551615", Some(18446744073709551615))]
    #[case("-1", None)]
    #[case("18446744073709551616", None)]
    fn test_u64(#[case] input: &str, #[case] expected: Option<u64>) {
        assert_eq!(u64::parse_path_variable(input), expected);
    }

    #[test]
    fn test_usize() {
        assert_eq!(usize::parse_path_variable("42"), Some(42_usize));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case(
        "340282366920938463463374607431768211455",
        Some(340282366920938463463374607431768211455)
    )]
    #[case("-1", None)]
    #[case("340282366920938463463374607431768211456", None)]
    fn test_u128(#[case] input: &str, #[case] expected: Option<u128>) {
        assert_eq!(u128::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("127", Some(127))]
    #[case("-128", Some(-128))]
    #[case("128", None)]
    #[case("-129", None)]
    fn test_i8(#[case] input: &str, #[case] expected: Option<i8>) {
        assert_eq!(i8::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("32767", Some(32767))]
    #[case("-32768", Some(-32768))]
    #[case("32768", None)]
    #[case("-32769", None)]
    fn test_i16(#[case] input: &str, #[case] expected: Option<i16>) {
        assert_eq!(i16::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("2147483647", Some(2147483647))]
    #[case("-2147483648", Some(-2147483648))]
    #[case("2147483648", None)]
    #[case("-2147483649", None)]
    fn test_i32(#[case] input: &str, #[case] expected: Option<i32>) {
        assert_eq!(i32::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("9223372036854775807", Some(9223372036854775807))]
    #[case("-9223372036854775808", Some(-9223372036854775808))]
    #[case("9223372036854775808", None)]
    #[case("-9223372036854775809", None)]
    fn test_i64(#[case] input: &str, #[case] expected: Option<i64>) {
        assert_eq!(i64::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case(
        "170141183460469231731687303715884105727",
        Some(170141183460469231731687303715884105727)
    )]
    #[case("-170141183460469231731687303715884105728", Some(-170141183460469231731687303715884105728))]
    #[case("170141183460469231731687303715884105728", None)]
    #[case("-170141183460469231731687303715884105729", None)]
    fn test_i128(#[case] input: &str, #[case] expected: Option<i128>) {
        assert_eq!(i128::parse_path_variable(input), expected);
    }

    #[test]
    fn test_isize() {
        assert_eq!(isize::parse_path_variable("42"), Some(42_isize));
        assert_eq!(isize::parse_path_variable("-42"), Some(-42_isize));
    }

    #[rstest]
    #[case("5.5", Some(5.5f32))]
    #[case("-5.5", Some(-5.5f32))]
    #[case("not_a_float", None)]
    fn test_f32(#[case] input: &str, #[case] expected: Option<f32>) {
        assert_eq!(f32::parse_path_variable(input), expected);
    }

    #[rstest]
    #[case("5.5", Some(5.5))]
    #[case("-5.5", Some(-5.5))]
    #[case("not_a_float", None)]
    fn test_f64(#[case] input: &str, #[case] expected: Option<f64>) {
        assert_eq!(f64::parse_path_variable(input), expected);
    }

    #[test]
    fn test_bytes() {
        assert_eq!(
            <&[u8]>::parse_path_variable("test"),
            Some(b"test".as_slice())
        );
    }

    #[test]
    fn test_str() {
        assert_eq!(<&str>::parse_path_variable("test"), Some("test"));
    }

    #[test]
    fn test_string() {
        assert_eq!(String::parse_path_variable("test"), Some("test".to_owned()));
    }
}

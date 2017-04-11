use url::percent_encoding::{EncodeSet, utf8_percent_encode};

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
struct URI_ENCODE_SET;

impl EncodeSet for URI_ENCODE_SET {
    fn contains(&self, byte: u8) -> bool {
        match byte {
            b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'-' | b'.' | b'_' | b'~' => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
struct URI_ENCODE_SET_WITHOUT_SLASH;

impl EncodeSet for URI_ENCODE_SET_WITHOUT_SLASH {
    fn contains(&self, byte: u8) -> bool {
        match byte {
            b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => false,
            _ => true,
        }
    }
}

/// Percent-encoding according to S3 specification
///
/// # From the Specification
///
/// * URI encode every byte except the unreserved characters: 'A'-'Z', 'a'-'z', '0'-'9', '-', '.',
///   '_', and '~'.
/// * The space character is a reserved character and must be encoded as "%20" (and not as "+").
/// * Each URI encoded byte is formed by a '%' and the two-digit hexadecimal value of the byte.
/// * Letters in the hexadecimal value must be uppercase, for example "%1A".
///
/// source: [https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html]()
#[inline]
pub fn encode_uri(uri: &str) -> String {
    utf8_percent_encode(uri, URI_ENCODE_SET).collect()
}

/// Percent-encoding for bucket keys according to S3 specification
///
/// This is identical to `encode_uri` with the exception that '/' is not encoded.
///
/// # From the Specification
///
/// * Encode the forward slash character, '/', everywhere except in the object key name. For
/// example, if the object key name is photos/Jan/sample.jpg, the forward slash in the key name
/// is not encoded.
/// * all the rules from `encode_uri`
///
/// source: [https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html]()
#[inline]
pub fn encode_uri_object_key(uri: &str) -> String {
    utf8_percent_encode(uri, URI_ENCODE_SET_WITHOUT_SLASH).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_uri() {
        assert_eq!("a%20%2Bbb%2Fc", encode_uri("a +bb/c"));
    }

    #[test]
    fn test_encode_uri_object_key() {
        assert_eq!("a%20%2Bbb/c", encode_uri_object_key("a +bb/c"));
    }
}

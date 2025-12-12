/// Computes the CRC64 checksum of a string and returns it as an uppercase hexadecimal string.
/// 
/// # Arguments
/// 
/// * `data` - The input string to compute the CRC64 checksum for.
/// * `seperator` - An optional separator to insert every 4 characters in the result.
/// 
/// # Returns
/// 
/// An uppercase hexadecimal string representing the CRC64 checksum.
/// If a separator is provided, it will be inserted every 4 characters.
/// 
/// # Examples
/// 
/// ```
/// use acovo::crc64_str;
/// 
/// let result = crc64_str("hello", None);
/// assert_eq!(result, "D6B0DD10CB6676A2");
/// 
/// let result = crc64_str("hello", Some("-"));
/// assert_eq!(result, "D6B0-DD10-CB66-76A2");
/// ```
#[cfg(feature = "hash")]
pub fn crc64_str(data: &str, seperator: Option<&str>) -> String {
    // Compute the CRC64 checksum of the input data
    let cksum = crc64::crc64(0, data.as_bytes());
    
    // If a separator is provided, format with separator efficiently
    if let Some(sep) = seperator {
        // Pre-allocate string with exact capacity needed
        let mut result = String::with_capacity(16 + sep.len() * 3);
        
        // Format each byte as uppercase hex and insert separators
        let bytes = cksum.to_be_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            if i == 2 || i == 4 || i == 6 {
                result.push_str(sep);
            }
            // Format byte as uppercase hex using hex crate
            let mut buf = [0u8; 2];
            hex::encode_to_slice([byte], &mut buf).unwrap();
            result.push_str(&std::str::from_utf8(&buf).unwrap().to_uppercase());
        }
        result
    } else {
        // Format without separator using hex crate directly
        let mut buffer = [0u8; 16];
        hex::encode_to_slice(cksum.to_be_bytes(), &mut buffer).unwrap();
        std::str::from_utf8(&buffer).unwrap().to_uppercase()
    }
}

#[cfg(test)]
#[cfg(feature = "hash")]
mod tests {
    use super::*;

    #[test]
    fn test_crc64_str() {
        let result = crc64_str("1234567890", Some("-"));
        println!("test_crc64_str: {}", result);
        assert_eq!(result.len() > 0, true);
    }
    
    #[test]
    fn test_crc64_str_without_separator() {
        let result = crc64_str("hello", None);
        // Actual CRC64 value for "hello"
        assert_eq!(result, "D6B0DD10CB6676A2");
    }
    
    #[test]
    fn test_crc64_str_with_separator() {
        let result = crc64_str("hello", Some("-"));
        // Actual CRC64 value for "hello" with separator
        assert_eq!(result, "D6B0-DD10-CB66-76A2");
    }
    
    #[test]
    fn test_crc64_str_empty_string() {
        let result = crc64_str("", Some("-"));
        // CRC64 of empty string
        assert_eq!(result, "0000-0000-0000-0000");
    }
    
    #[test]
    fn test_crc64_str_different_separator() {
        let result = crc64_str("world", Some(":"));
        // Should use colon as separator
        assert_eq!(result, "D139:2981:B85E:EC4A");
    }
    
    #[test]
    fn test_crc64_str_long_string() {
        let long_string = "This is a longer string to test the CRC64 implementation with more data.";
        let result = crc64_str(long_string, None);
        assert_eq!(result.len(), 16); // CRC64 should always produce 16 hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit())); // All characters should be hex digits
    }
    
    #[test]
    fn test_crc64_str_unicode_characters() {
        let result = crc64_str("🦀🚀", None);
        assert_eq!(result.len(), 16); // Should still produce 16 hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_crc64_str_only_separator_provided() {
        // Test with only separator and no data
        let result = crc64_str("", Some("--"));
        assert_eq!(result, "0000--0000--0000--0000");
    }
    
    #[test]
    fn test_crc64_str_special_characters() {
        let result = crc64_str("!@#$%^&*()", None);
        assert_eq!(result.len(), 16);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
        // Actual value: 93BE8325535C0007
    }
    
    #[test]
    fn test_crc64_str_numeric_string() {
        let result = crc64_str("1234567890", None);
        assert_eq!(result, "93BE8325535C0007");
    }
    
    #[test]
    fn test_crc64_str_mixed_case() {
        let result = crc64_str("HeLLo WoRLd", None);
        assert_eq!(result, "D42FBB7E0D6994A0");
    }
    
    #[test]
    fn test_crc64_str_single_character() {
        let result = crc64_str("a", None);
        assert_eq!(result, "5FB354025B277B14");
    }
    
    #[test]
    fn test_crc64_str_repeated_characters() {
        let result = crc64_str("AAAAAAAAAA", None);
        assert_eq!(result, "6E5020227049B06F");
    }
    
    #[test]
    fn test_crc64_str_whitespace_only() {
        let result = crc64_str("     ", None);
        assert_eq!(result, "EB69A4B8F14AFEB7");
    }
    
    #[test]
    fn test_crc64_str_newlines_and_tabs() {
        let result = crc64_str("\n\t\n\t", None);
        assert_eq!(result, "77A9E4696E2AE9F5");
    }
}

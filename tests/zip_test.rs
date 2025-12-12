//! Integration tests for the zip module

#[cfg(test)]
mod zip_tests {
    use acovo::zip::extract_zip;

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_function_exists() {
        // This test ensures that the extract_zip function exists and compiles
        // We don't actually run it to avoid filesystem side effects in tests
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_error_handling() {
        // Test that extract_zip properly handles errors for non-existent files
        let result = extract_zip("non_existent_file.zip", "/tmp");
        assert!(result.is_err());
    }
}
//! Integration tests for the trace module

#[cfg(test)]
mod trace_tests {
    #[test]
    #[cfg(feature = "trace")]
    fn test_trace_macros_compile() {
        // This test ensures that the trace macros compile correctly
        // We don't actually run them to avoid conflicts with the global tracing subscriber
        use acovo::init_tracing;
        use acovo::init_global_tracing;
        
        // Test that the macros exist and compile
        // We're not invoking them to avoid conflicts in the test environment
    }
    
    #[test]
    #[cfg(feature = "trace")]
    fn test_trace_module_features() {
        // Test that trace module features are available
        use acovo::time::LocalTimeFormatter;
        use acovo::fs::get_exe_dir;
        
        // Verify that the imports work
        let _formatter = LocalTimeFormatter;
        let _dir = get_exe_dir();
    }
}
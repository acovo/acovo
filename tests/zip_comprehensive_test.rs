//! Comprehensive integration tests for the zip module

#[cfg(test)]
mod zip_tests {
    use acovo::zip::extract_zip;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;
    use std::path::Path;

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_function_exists() {
        // This test ensures that the extract_zip function exists and compiles
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_error_handling_nonexistent_file() {
        // Test that extract_zip properly handles errors for non-existent files
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let dest_dir = temp_dir.path().to_str().unwrap();
        
        let result = extract_zip("non_existent_file.zip", dest_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to open ZIP file"));
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_with_valid_zip_file() {
        // Create a temporary directory for our test files
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_path = temp_dir.path();
        
        // Create a simple ZIP file in memory
        let zip_data: Vec<u8> = create_simple_zip();
        
        // Write the ZIP data to a file
        let zip_file_path = temp_path.join("test.zip");
        fs::write(&zip_file_path, &zip_data).expect("Failed to write ZIP file");
        
        // Create destination directory
        let dest_dir = temp_path.join("extracted");
        fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
        
        // Extract the ZIP file
        let result = extract_zip(
            zip_file_path.to_str().unwrap(), 
            dest_dir.to_str().unwrap()
        );
        
        // Verify extraction succeeded
        assert!(result.is_ok(), "Extract should succeed: {:?}", result.err());
        
        // Verify extracted files exist
        let extracted_file = dest_dir.join("test.txt");
        assert!(extracted_file.exists(), "Expected extracted file to exist");
        
        // Verify file content
        let content = fs::read_to_string(&extracted_file).expect("Failed to read extracted file");
        assert_eq!(content, "Hello, World!", "File content should match");
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_creates_directories() {
        // Create a temporary directory for our test files
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_path = temp_dir.path();
        
        // Create a ZIP file with directory structure
        let zip_data: Vec<u8> = create_zip_with_directories();
        
        // Write the ZIP data to a file
        let zip_file_path = temp_path.join("test_nested.zip");
        fs::write(&zip_file_path, &zip_data).expect("Failed to write ZIP file");
        
        // Create destination directory
        let dest_dir = temp_path.join("extracted_nested");
        fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
        
        // Extract the ZIP file
        let result = extract_zip(
            zip_file_path.to_str().unwrap(), 
            dest_dir.to_str().unwrap()
        );
        
        // Verify extraction succeeded
        assert!(result.is_ok(), "Extract should succeed: {:?}", result.err());
        
        // Verify directory structure was created
        let nested_dir = dest_dir.join("nested");
        assert!(nested_dir.exists(), "Nested directory should exist");
        assert!(nested_dir.is_dir(), "Should be a directory");
        
        // Verify nested file exists
        let nested_file = nested_dir.join("inner.txt");
        assert!(nested_file.exists(), "Nested file should exist");
        
        // Verify nested file content
        let content = fs::read_to_string(&nested_file).expect("Failed to read nested file");
        assert_eq!(content, "Nested content", "Nested file content should match");
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_empty_filename() {
        // Test that extract_zip properly handles empty filename
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let dest_dir = temp_dir.path().to_str().unwrap();
        
        let result = extract_zip("", dest_dir);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_empty_dest_dir() {
        // Test that extract_zip properly handles empty destination directory
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_path = temp_dir.path();
        
        // Create a simple ZIP file
        let zip_data: Vec<u8> = create_simple_zip();
        let zip_file_path = temp_path.join("test.zip");
        fs::write(&zip_file_path, &zip_data).expect("Failed to write ZIP file");
        
        // Use current directory as destination when empty string is provided
        let result = extract_zip(zip_file_path.to_str().unwrap(), "");
        // The function should succeed as it creates the directory
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_multiple_files() {
        // Create a temporary directory for our test files
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_path = temp_dir.path();
        
        // Create a ZIP file with multiple files
        let zip_data: Vec<u8> = create_zip_with_multiple_files();
        
        // Write the ZIP data to a file
        let zip_file_path = temp_path.join("test_multiple.zip");
        fs::write(&zip_file_path, &zip_data).expect("Failed to write ZIP file");
        
        // Create destination directory
        let dest_dir = temp_path.join("extracted_multiple");
        fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
        
        // Extract the ZIP file
        let result = extract_zip(
            zip_file_path.to_str().unwrap(), 
            dest_dir.to_str().unwrap()
        );
        
        // Verify extraction succeeded
        assert!(result.is_ok(), "Extract should succeed: {:?}", result.err());
        
        // Verify first file exists and content
        let file1 = dest_dir.join("file1.txt");
        assert!(file1.exists(), "First file should exist");
        let content1 = fs::read_to_string(&file1).expect("Failed to read first file");
        assert_eq!(content1, "Content of file 1", "First file content should match");
        
        // Verify second file exists and content
        let file2 = dest_dir.join("file2.txt");
        assert!(file2.exists(), "Second file should exist");
        let content2 = fs::read_to_string(&file2).expect("Failed to read second file");
        assert_eq!(content2, "Content of file 2", "Second file content should match");
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_extract_zip_with_special_characters() {
        // Create a temporary directory for our test files
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_path = temp_dir.path();
        
        // Create a ZIP file with special characters in filenames
        let zip_data: Vec<u8> = create_zip_with_special_characters();
        
        // Write the ZIP data to a file
        let zip_file_path = temp_path.join("test_special.zip");
        fs::write(&zip_file_path, &zip_data).expect("Failed to write ZIP file");
        
        // Create destination directory
        let dest_dir = temp_path.join("extracted_special");
        fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
        
        // Extract the ZIP file
        let result = extract_zip(
            zip_file_path.to_str().unwrap(), 
            dest_dir.to_str().unwrap()
        );
        
        // Verify extraction succeeded
        assert!(result.is_ok(), "Extract should succeed: {:?}", result.err());
        
        // Verify file with special characters exists
        let special_file = dest_dir.join("special_文件.txt");
        assert!(special_file.exists(), "File with special characters should exist");
        
        // Verify file content
        let content = fs::read_to_string(&special_file).expect("Failed to read special file");
        assert_eq!(content, "Special content", "Special file content should match");
    }

    // Helper function to create a simple ZIP file in memory
    #[cfg(feature = "compress")]
    fn create_simple_zip() -> Vec<u8> {
        use std::io::Cursor;
        let mut zip_data = Vec::new();
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_data));
        
        // Start writing the first file
        zip.start_file::<_, ()>("test.txt", zip::write::FileOptions::default())
            .expect("Failed to start file in ZIP");
        zip.write_all(b"Hello, World!").expect("Failed to write to ZIP file");
        
        // Finish the ZIP
        zip.finish().expect("Failed to finish ZIP");
        
        zip_data
    }

    // Helper function to create a ZIP file with directory structure
    #[cfg(feature = "compress")]
    fn create_zip_with_directories() -> Vec<u8> {
        use std::io::Cursor;
        let mut zip_data = Vec::new();
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_data));
        
        // Create a directory entry
        zip.add_directory::<_, ()>("nested/", zip::write::FileOptions::default())
            .expect("Failed to add directory to ZIP");
        
        // Add a file in the directory
        zip.start_file::<_, ()>("nested/inner.txt", zip::write::FileOptions::default())
            .expect("Failed to start file in ZIP");
        zip.write_all(b"Nested content").expect("Failed to write to ZIP file");
        
        // Finish the ZIP
        zip.finish().expect("Failed to finish ZIP");
        
        zip_data
    }

    // Helper function to create a ZIP file with multiple files
    #[cfg(feature = "compress")]
    fn create_zip_with_multiple_files() -> Vec<u8> {
        use std::io::Cursor;
        let mut zip_data = Vec::new();
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_data));
        
        // Add first file
        zip.start_file::<_, ()>("file1.txt", zip::write::FileOptions::default())
            .expect("Failed to start first file in ZIP");
        zip.write_all(b"Content of file 1").expect("Failed to write to first ZIP file");
        
        // Add second file
        zip.start_file::<_, ()>("file2.txt", zip::write::FileOptions::default())
            .expect("Failed to start second file in ZIP");
        zip.write_all(b"Content of file 2").expect("Failed to write to second ZIP file");
        
        // Finish the ZIP
        zip.finish().expect("Failed to finish ZIP");
        
        zip_data
    }

    // Helper function to create a ZIP file with special characters in filenames
    #[cfg(feature = "compress")]
    fn create_zip_with_special_characters() -> Vec<u8> {
        use std::io::Cursor;
        let mut zip_data = Vec::new();
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_data));
        
        // Add file with special characters
        zip.start_file::<_, ()>("special_文件.txt", zip::write::FileOptions::default())
            .expect("Failed to start file with special characters in ZIP");
        zip.write_all(b"Special content").expect("Failed to write to special ZIP file");
        
        // Finish the ZIP
        zip.finish().expect("Failed to finish ZIP");
        
        zip_data
    }
}
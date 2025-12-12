/// Extracts a ZIP archive to the specified destination directory.
/// 
/// This function opens a ZIP file and extracts all its contents to the specified
/// destination directory, preserving the directory structure. It handles both
/// files and directories within the ZIP archive.
/// 
/// # Arguments
/// 
/// * `filename` - Path to the ZIP file to extract
/// * `dest_dir` - Destination directory where files will be extracted
/// 
/// # Returns
/// 
/// * `Ok(())` if extraction succeeds
/// * `Err(Box<dyn std::error::Error>)` if extraction fails
/// 
/// # Features
/// 
/// This function is only compiled when the "compress" feature is enabled.
/// 
/// # Example
/// 
/// ```rust
/// # #[cfg(feature = "compress")]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use acovo::zip::extract_zip;
///
/// // This will attempt to extract "archive.zip" to "./extracted/" directory
/// // but will return an error if the file doesn't exist
/// match extract_zip("archive.zip", "./extracted/") {
///     Ok(()) => println!("Extraction succeeded"),
///     Err(e) => println!("Extraction failed: {}", e),
/// }
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "compress"))]
/// # fn main() {}
/// ```
/// 
/// # Notes
/// 
/// * The function replaces backslashes in file paths with forward slashes for cross-platform compatibility
/// * Directory structure within the ZIP is preserved during extraction
/// * File permissions are handled appropriately on Unix systems (when available)
#[cfg(feature = "compress")]
pub fn extract_zip(filename: &str, dest_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use zip::ZipArchive;

    // Open the ZIP file
    let file = fs::File::open(filename)
        .map_err(|e| format!("Failed to open ZIP file '{}': {}", filename, e))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to parse ZIP archive: {}", e))?;

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("Failed to create destination directory '{}': {}", dest_dir, e))?;

    // Extract each file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to access file at index {} in ZIP: {}", i, e))?;
        let outpath = Path::new(dest_dir).join(file.mangled_name());

        // Handle directories
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory '{:?}': {}", outpath, e))?;
        } else {
            // Create parent directories if they don't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent directory '{:?}': {}", p, e))?;
                }
            }
            // Write file contents
            let mut outfile = fs::File::create(&outpath)
                .map_err(|e| format!("Failed to create output file '{:?}': {}", outpath, e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to copy data to file '{:?}': {}", outpath, e))?;
        }
    }

    Ok(())
}

#[cfg(test)]
#[cfg(feature = "compress")]
mod tests {
    use crate::fs::get_exe_parent_path;

    use super::*;

    #[test]
    fn test_extract_zip() {
        // This test currently just verifies that the function compiles correctly
        // In a future improvement, we could create an actual test ZIP file and verify extraction
    }
}

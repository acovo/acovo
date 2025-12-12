use anyhow::{anyhow, Result as AnyResult};
#[cfg(feature = "fs")]
use std::{fs::File, io, path::Path, path::PathBuf};

/// Gets the directory path of the current executable file
///
/// This function retrieves the path of the currently running executable,
/// then extracts its parent directory. It handles potential errors in
/// retrieving the executable path or converting it to a string.
///
/// # Returns
/// * `Ok(String)` - The absolute path to the directory containing the executable
/// * `Err(anyhow::Error)` - If there was an error getting the current executable path,
///                          if the executable has no parent directory, or if the path
///                          cannot be converted to a string
///
/// # Examples
/// ```
/// use acovo::get_exe_dir;
///
/// match get_exe_dir() {
///     Ok(dir) => println!("Executable is in directory: {}", dir),
///     Err(e) => eprintln!("Failed to get executable directory: {}", e),
/// }
/// ```
#[cfg(feature = "fs")]
pub fn get_exe_dir() -> AnyResult<String> {
    match std::env::current_exe()?
        .parent()
        .ok_or(anyhow!("ParentNotFound"))?
        .to_path_buf()
        .into_os_string()
        .into_string()
    {
        Ok(data) => Ok(data),
        Err(e) => Err(anyhow!("{:?}", e)),
    }
}

/// Creates a directory and all of its parent directories if they don't exist
///
/// # Arguments
/// * `path` - A string slice that holds the path to the directory to create
///
/// # Returns
/// * `Ok(())` - If the directory was created successfully
/// * `Err(io::Error)` - If there was an error creating the directory
///
/// # Examples
/// ```
/// use acovo::mkdir;
///
/// mkdir("./path/to/new/directory").unwrap();
/// ```
#[cfg(feature = "fs")]
pub fn mkdir(path: &str) -> io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Reads lines from a file and returns an iterator over the lines
///
/// # Arguments
/// * `filename` - A generic parameter that can be converted to a Path reference
///
/// # Returns
/// * `Ok(io::Lines<io::BufReader<File>>)` - An iterator over the lines in the file
/// * `Err(io::Error)` - If there was an error opening the file
///
/// # Examples
/// ```
/// use acovo::read_lines;
///
/// if let Ok(lines) = read_lines("path/to/file.txt") {
///     for line in lines {
///         if let Ok(content) = line {
///             println!("{}", content);
///         }
///     }
/// }
/// ```
#[cfg(feature = "fs")]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    use std::io::BufRead;

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Writes a vector of strings to a file, either creating a new file or appending to an existing one
///
/// # Arguments
/// * `file` - The path to the file to write to
/// * `lines` - A vector of strings to write to the file
/// * `create` - If true, creates a new file (truncating if it exists); if false, appends to the file
///
/// # Returns
/// * `Ok(())` - If the lines were written successfully
/// * `Err(anyhow::Error)` - If there was an error writing to the file
///
/// # Examples
/// ```
/// use acovo::write_lines;
///
/// let lines = vec!["First line".to_string(), "Second line".to_string()];
/// write_lines("path/to/file.txt".to_string(), lines, true).unwrap();
/// ```
#[cfg(feature = "fs")]
pub fn write_lines(file: String, lines: Vec<String>, create: bool) -> AnyResult<()> {
    use std::io::{BufWriter, Write};
    
    let file_writer = if create {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file)?
    } else {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file)?
    };

    let mut buf_writer = BufWriter::new(file_writer);
    
    for line in lines {
        buf_writer.write_all(line.as_bytes())?;
        buf_writer.write_all(b"\n")?;
    }
    
    buf_writer.flush()?;
    Ok(())
}

/// Gets the parent directory of the current executable file
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the parent directory of the executable
/// * `Err(anyhow::Error)` - If there was an error getting the executable path or if the path has no parent
///
/// # Examples
/// ```
/// use acovo::get_exe_parent_path;
///
/// let parent_path = get_exe_parent_path().unwrap();
/// println!("Executable parent directory: {:?}", parent_path);
/// ```
#[cfg(feature = "fs")]
pub fn get_exe_parent_path() -> AnyResult<PathBuf> {
    let exe_dir = get_exe_dir()?;
    let path = PathBuf::from(exe_dir);
    let ret_option = path.parent().map(PathBuf::from);
    ret_option.ok_or_else(|| anyhow!("PathNotFound"))
}

/// Gets the parent directory of the current working directory
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the parent directory of the current working directory
/// * `Err(anyhow::Error)` - If there was an error getting the current directory or if the path has no parent
///
/// # Examples
/// ```
/// use acovo::get_current_parent_path;
///
/// let parent_path = get_current_parent_path().unwrap();
/// println!("Current working directory parent: {:?}", parent_path);
/// ```
#[cfg(feature = "fs")]
pub fn get_current_parent_path() -> AnyResult<PathBuf> {
    use std::env;
    let binding = env::current_dir()?;
    let current_dir = Path::new(&binding);
    println!("current_dir {:?}", &current_dir);
    let ret_option = current_dir.parent().map(PathBuf::from);
    ret_option.ok_or_else(|| anyhow!("PathNotFound"))
}

/// Gets the parent directory of a given path
///
/// # Arguments
/// * `path` - A reference to a Path object
///
/// # Returns
/// * `Some(PathBuf)` - The parent directory path if it exists
/// * `None` - If the path has no parent (e.g., root directory)
///
/// # Examples
/// ```
/// use acovo::get_parent_path;
/// use std::path::Path;
///
/// let path = Path::new("/home/user/documents/file.txt");
/// if let Some(parent) = get_parent_path(path) {
///     println!("Parent directory: {:?}", parent);
/// }
/// ```
#[cfg(feature = "fs")]
pub fn get_parent_path(path: &Path) -> Option<PathBuf> {
    path.parent().map(PathBuf::from)
}

/// Lists all files in a directory (and its subdirectories) with a specific extension
///
/// # Arguments
/// * `dir` - A reference to a Path object representing the directory to search
/// * `ext` - A string slice representing the file extension to filter by (without the dot)
///
/// # Returns
/// A vector of PathBuf objects representing the paths to files with the specified extension
///
/// # Examples
/// ```
/// use acovo::list_files;
/// use std::path::Path;
///
/// let files = list_files(Path::new("./src"), "rs");
/// for file in files {
///     println!("Found Rust file: {:?}", file);
/// }
/// ```
pub fn list_files(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs_to_visit = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        if current_dir.is_dir() {
            match std::fs::read_dir(&current_dir) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(extension) = path.extension() {
                                    if extension == ext {
                                        files.push(path);
                                    }
                                }
                            } else if path.is_dir() {
                                dirs_to_visit.push(path);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Failed to read directory {}: {}", current_dir.display(), e),
            }
        }
    }
    files
}

/// Extracts the file name from a given path
///
/// # Arguments
/// * `path` - A PathBuf object representing the path to extract the file name from
///
/// # Returns
/// * `Some(String)` - The file name as a String if it exists
/// * `None` - If the path has no file name (e.g., root directory)
///
/// # Examples
/// ```
/// use acovo::file_name;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("/home/user/documents/file.txt");
/// if let Some(name) = file_name(path) {
///     println!("File name: {}", name); // Outputs: File name: file.txt
/// }
/// ```
pub fn file_name(path: PathBuf) -> Option<String> {
    if let Some(file_name_os_str) = path.file_name() {
        return Some(file_name_os_str.to_string_lossy().into_owned());
    }
    None
}

#[cfg(test)]
#[cfg(feature = "fs")]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_get_exe_dir() {
        let result = get_exe_dir();
        println!("got_exe_dir: {:?}", result);
        assert_eq!(result.unwrap().len() > 0, true);
    }

    #[test]
    fn test_mkdir() {
        let result = mkdir("/tmp/123456");
        println!("test_mkdir: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_write_read_lines() {
        let out_dir = get_exe_dir().unwrap();
        println!("got_exe_dir: {:?}", out_dir);
        let mut test_data: Vec<String> = vec![];
        test_data.push("1".into());
        test_data.push("2".into());

        let file_name = format!("{}/test.txt", out_dir);
        write_lines(file_name.clone(), test_data, true);

        println!("ToReadLines");
        if let Ok(lines) = read_lines(file_name) {
            for line in lines {
                if let Ok(text) = line {
                    println!("{}", text);
                }
            }
        }
    }

    #[test]
    fn test_write_lines_create() {
        let test_dir = "/tmp/acovo_test";
        mkdir(test_dir).expect("Failed to create test directory");
        
        let file_path = format!("{}/test_create.txt", test_dir);
        let lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        
        let result = write_lines(file_path.clone(), lines.clone(), true);
        assert!(result.is_ok());
        
        // Verify content
        if let Ok(read_lines) = read_lines(&file_path) {
            let mut i = 0;
            for line in read_lines {
                if let Ok(content) = line {
                    assert_eq!(content, lines[i]);
                    i += 1;
                }
            }
            assert_eq!(i, lines.len());
        }
        
        // Clean up
        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_write_lines_append() {
        let test_dir = "/tmp/acovo_test_append";
        mkdir(test_dir).expect("Failed to create test directory");
        
        let file_path = format!("{}/test_append.txt", test_dir);
        
        // Write initial content
        let initial_lines = vec!["Initial Line 1".to_string(), "Initial Line 2".to_string()];
        let result = write_lines(file_path.clone(), initial_lines.clone(), true);
        assert!(result.is_ok());
        
        // Append more content
        let appended_lines = vec!["Appended Line 1".to_string(), "Appended Line 2".to_string()];
        let result = write_lines(file_path.clone(), appended_lines.clone(), false);
        assert!(result.is_ok());
        
        // Verify content
        let expected_lines = vec![
            "Initial Line 1",
            "Initial Line 2",
            "Appended Line 1",
            "Appended Line 2"
        ];
        
        if let Ok(read_lines) = read_lines(&file_path) {
            let mut i = 0;
            for line in read_lines {
                if let Ok(content) = line {
                    assert_eq!(content, expected_lines[i]);
                    i += 1;
                }
            }
            assert_eq!(i, expected_lines.len());
        }
        
        // Clean up
        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_list_files() {
        let test_dir = "/tmp/acovo_list_files_test";
        mkdir(test_dir).expect("Failed to create test directory");
        
        // Create test files
        let file1_path = format!("{}/file1.rs", test_dir);
        let file2_path = format!("{}/file2.rs", test_dir);
        let file3_path = format!("{}/file3.txt", test_dir);
        
        // Create subdirectory
        let subdir_path = format!("{}/subdir", test_dir);
        mkdir(&subdir_path).expect("Failed to create subdirectory");
        let file4_path = format!("{}/file4.rs", subdir_path);
        
        // Write content to files
        let mut file1 = fs::File::create(&file1_path).expect("Failed to create file1");
        file1.write_all(b"// Test file 1").expect("Failed to write to file1");
        
        let mut file2 = fs::File::create(&file2_path).expect("Failed to create file2");
        file2.write_all(b"// Test file 2").expect("Failed to write to file2");
        
        let mut file3 = fs::File::create(&file3_path).expect("Failed to create file3");
        file3.write_all(b"# Test file 3").expect("Failed to write to file3");
        
        let mut file4 = fs::File::create(&file4_path).expect("Failed to create file4");
        file4.write_all(b"// Test file 4").expect("Failed to write to file4");
        
        // Test listing .rs files
        let rs_files = list_files(Path::new(test_dir), "rs");
        assert_eq!(rs_files.len(), 3); // Should find 3 .rs files
        
        // Check that all returned paths are .rs files
        for file_path in rs_files {
            assert_eq!(file_path.extension().unwrap(), "rs");
        }
        
        // Clean up
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_file_name() {
        // Test with a file path
        let path = PathBuf::from("/home/user/documents/file.txt");
        let name = file_name(path);
        assert_eq!(name, Some("file.txt".to_string()));
        
        // Test with a directory path
        let path = PathBuf::from("/home/user/documents");
        let name = file_name(path);
        assert_eq!(name, Some("documents".to_string()));
        
        // Test with root path (no file name)
        let path = PathBuf::from("/");
        let name = file_name(path);
        assert_eq!(name, None);
    }

    #[test]
    fn test_get_parent_path() {
        // Test with a file path
        let path = Path::new("/home/user/documents/file.txt");
        let parent = get_parent_path(path);
        assert_eq!(parent, Some(PathBuf::from("/home/user/documents")));
        
        // Test with a directory path
        let path = Path::new("/home/user/documents");
        let parent = get_parent_path(path);
        assert_eq!(parent, Some(PathBuf::from("/home/user")));
        
        // Test with root path (no parent)
        let path = Path::new("/");
        let parent = get_parent_path(path);
        assert_eq!(parent, None);
    }
}

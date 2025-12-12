use anyhow::{anyhow, Result as AnyResult};
#[cfg(feature = "fs")]
use std::{fs::File, io, path::Path, path::PathBuf};

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

#[cfg(feature = "fs")]
pub fn mkdir(path: &str) -> io::Result<()> {
    std::fs::create_dir_all(path)
}

#[cfg(feature = "fs")]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    use std::io::BufRead;

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(feature = "fs")]
pub fn write_lines(file: String, lines: Vec<String>, create: bool) -> AnyResult<()> {
    println!("ToWriteLines {}", lines.len());

    let mut file_writer = if create {
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

    use std::io::Write;
    for line in lines {
        file_writer.write_all(line.as_bytes())?;
        file_writer.write(b"\n")?;
    }

    Ok(())
}

#[cfg(feature = "fs")]
pub fn get_exe_parent_path() -> AnyResult<PathBuf> {
    let exe_dir = get_exe_dir()?;
    let path = PathBuf::from(exe_dir);
    let ret_option = path.parent().map(PathBuf::from);
    ret_option.ok_or_else(|| anyhow!("PathNotFound"))
}

#[cfg(feature = "fs")]
pub fn get_current_parent_path() -> AnyResult<PathBuf> {
    use std::env;
    let binding = env::current_dir()?;
    let current_dir = Path::new(&binding);
    println!("current_dir {:?}", &current_dir);
    let ret_option = current_dir.parent().map(PathBuf::from);
    ret_option.ok_or_else(|| anyhow!("PathNotFound"))
}

#[cfg(feature = "fs")]
pub fn get_parent_path(path: &Path) -> Option<PathBuf> {
    path.parent().map(PathBuf::from)
}

pub fn list_files(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if dir.is_dir() {
        match std::fs::read_dir(dir) {
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
                            files.extend(list_files(&path, ext));
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory {}: {}", dir.display(), e),
        }
    }
    files
}

pub fn file_name(path: PathBuf) -> Option<String> {
    path.file_name()
        .map(|file_name_os_str| file_name_os_str.to_string_lossy().into_owned())
}

#[cfg(test)]
#[cfg(feature = "fs")]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_get_exe_dir() {
        let result = get_exe_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(Path::new(&path).exists());
    }

    #[test]
    fn test_mkdir() {
        let test_dir = "./test_dir";
        let result = mkdir(test_dir);
        assert!(result.is_ok());
        assert!(Path::new(test_dir).exists());
        // Clean up
        let _ = fs::remove_dir(test_dir);
    }

    #[test]
    fn test_write_read_lines() {
        let test_file = "./test_lines.txt".to_string();
        let lines = vec!["line1".to_string(), "line2".to_string(), "line3".to_string()];
        
        // Test writing
        let write_result = write_lines(test_file.clone(), lines.clone(), true);
        assert!(write_result.is_ok());
        
        // Test reading
        let read_result = read_lines(&test_file);
        assert!(read_result.is_ok());
        let read_lines: Result<Vec<_>, _> = read_result.unwrap().collect();
        assert!(read_lines.is_ok());
        let read_lines = read_lines.unwrap();
        assert_eq!(read_lines.len(), 3);
        assert_eq!(read_lines[0], "line1");
        assert_eq!(read_lines[1], "line2");
        assert_eq!(read_lines[2], "line3");
        
        // Clean up
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_get_exe_parent_path() {
        let result = get_exe_parent_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
    }

    #[test]
    fn test_get_current_parent_path() {
        let result = get_current_parent_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
    }

    #[test]
    fn test_list_files() {
        // Create a temporary directory structure for testing
        let test_dir = "./test_list_files";
        let _ = fs::create_dir(test_dir);
        let _ = fs::create_dir(format!("{}/subdir", test_dir));
        
        // Create test files
        let mut file1 = fs::File::create(format!("{}/file1.txt", test_dir)).unwrap();
        let mut file2 = fs::File::create(format!("{}/file2.txt", test_dir)).unwrap();
        let mut file3 = fs::File::create(format!("{}/subdir/file3.txt", test_dir)).unwrap();
        let mut file4 = fs::File::create(format!("{}/file4.log", test_dir)).unwrap();
        
        writeln!(file1, "test").unwrap();
        writeln!(file2, "test").unwrap();
        writeln!(file3, "test").unwrap();
        writeln!(file4, "test").unwrap();
        
        // Test listing files with .txt extension
        let txt_files = list_files(Path::new(test_dir), "txt");
        assert_eq!(txt_files.len(), 3); // Should find file1.txt, file2.txt, and subdir/file3.txt
        
        // Test listing files with .log extension
        let log_files = list_files(Path::new(test_dir), "log");
        assert_eq!(log_files.len(), 1); // Should find file4.log
        
        // Clean up
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_file_name() {
        // Test with a file path
        let path = PathBuf::from("/home/user/document.txt");
        let name = file_name(path);
        assert_eq!(name, Some("document.txt".to_string()));
        
        // Test with a directory path
        let path = PathBuf::from("/home/user/documents/");
        let name = file_name(path);
        assert_eq!(name, Some("documents".to_string()));
        
        // Test with root path
        let path = PathBuf::from("/");
        let name = file_name(path);
        assert_eq!(name, None);
    }

    #[test]
    fn test_get_parent_path() {
        let path = PathBuf::from("/home/user/documents/file.txt");
        let parent = get_parent_path(&path);
        assert_eq!(parent, Some(PathBuf::from("/home/user/documents")));
        
        // Test with root path
        let path = PathBuf::from("/");
        let parent = get_parent_path(&path);
        assert_eq!(parent, None);
    }
}

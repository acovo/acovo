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

    if dir.is_dir() {
        match std::fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            let extension = path.extension();
                            if extension.is_some() {
                                if extension.unwrap() == ext {
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
}

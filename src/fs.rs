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

    if create == true {
        let mut file_writer = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{}", &file))?;
        for line in lines {
            use std::io::Write;

            file_writer.write_all(line.as_bytes())?;
            file_writer.write(b"\n")?;
        }
    } else {
        let mut file_writer = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}", &file))?;
        for line in lines {
            use std::io::Write;

            file_writer.write_all(line.as_bytes())?;
            file_writer.write(b"\n")?;
        }
    }

    Ok(())
}

#[cfg(feature = "fs")]
pub fn get_exe_parent_path() -> AnyResult<PathBuf> {
    let exe_dir = get_exe_dir()?;
    let path = PathBuf::from(exe_dir);
    let ret_option = path.parent().map(PathBuf::from);
    if ret_option.is_some() {
        Ok(ret_option.unwrap())
    } else {
        Err(anyhow!("PathNotFound"))
    }
}

#[cfg(feature = "fs")]
pub fn get_current_parent_path() -> AnyResult<PathBuf> {
    use std::env;
    let binding = env::current_dir().unwrap();
    let current_dir = Path::new(&binding);
    println!("current_dir {:?}", &current_dir);
    let ret_option = current_dir.parent().map(PathBuf::from);
    if ret_option.is_some() {
        Ok(ret_option.unwrap())
    } else {
        Err(anyhow!("PathNotFound"))
    }
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

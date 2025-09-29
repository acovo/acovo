#[cfg(feature = "fs")]
use std::{fs::File, io, path::Path,path::PathBuf};

use anyhow::{anyhow, Result as AnyResult};

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
where P: AsRef<Path>, {
    use std::io::BufRead;

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(feature = "fs")]
pub fn get_exe_parent_path() -> AnyResult<PathBuf> {
    let exe_dir = get_exe_dir()?;
    let path = PathBuf::from(exe_dir);
    let ret_option = path.parent().map(PathBuf::from);
    if ret_option.is_some() {
        Ok(ret_option.unwrap())
    }else {
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
    }else {
        Err(anyhow!("PathNotFound"))
    }
}

#[cfg(feature = "fs")]
pub fn get_parent_path(path: &Path) -> Option<PathBuf> {
    path.parent().map(PathBuf::from)
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
}

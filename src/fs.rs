#[cfg(feature = "fs")]
use std::io;

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

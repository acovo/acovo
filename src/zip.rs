#[cfg(feature = "compress")]
fn extract_zip(filename: &str, dest_dir: &str) -> i32 {
    use std::fs;

    let fname = std::path::Path::new(filename);
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // check if contains “\”

            use std::io;
            let zip_item_name = file.name().replace("\\", "/");

            if zip_item_name.contains("\\") {
                // create directory by filepath.
                use crate::fs::mkdir;

                let last_fs_pos = zip_item_name.rfind("\\").unwrap_or(0usize);
                let mut zip_item_dir: String = "".into();
                if last_fs_pos > 0 {
                    zip_item_dir = zip_item_name[0..last_fs_pos].to_string();
                }

                fs::create_dir_all(&zip_item_dir.replace("\\", "/"));
            }

            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&zip_item_name).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                //fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}

#[cfg(test)]
#[cfg(feature = "compress")]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result as AnyResult};

    #[test]
    fn test_extract_zip() {
        extract_zip("../test.zip", "");
    }
}

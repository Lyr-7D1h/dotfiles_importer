use std::ffi;
use std::fs;
use std::io;
use std::path;

pub fn link(src_dir: &path::PathBuf, dest_dir: &path::PathBuf) -> io::Result<()> {
    link_recurse(src_dir, dest_dir, src_dir)
}

fn link_recurse(
    src_dir: &path::PathBuf,
    dest_dir: &path::PathBuf,
    cur_dir: &path::PathBuf,
) -> io::Result<()> {
    for entry in cur_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let found_path = entry.path();

            if found_path.file_name() == Some(ffi::OsStr::new(".git"))
                || found_path.file_name() == Some(ffi::OsStr::new("dotfiles_importer"))
            {
                println!("Ignoring {:?}", found_path);
                continue;
            }

            let dest_path = dest_dir.join(found_path.strip_prefix(src_dir).unwrap());

            // If file already exists remove
            if dest_path.is_file() {
                fs::remove_file(&dest_path)?;
            }

            if found_path.is_file() {
                if dest_path.is_dir() {
                    fs::remove_dir_all(&dest_path)?;
                }

                println!("{:?} to {:?}", found_path, dest_path);
                if let Err(err) = fs::hard_link(found_path, dest_path) {
                    return Err(err);
                }
            } else if found_path.is_dir() {
                if !dest_path.exists() {
                    fs::create_dir(&dest_path)?;
                }

                link_recurse(src_dir, dest_dir, &found_path)?;
            }
        }
    }

    Ok(())
}

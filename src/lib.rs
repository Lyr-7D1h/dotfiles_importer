use std::fs;
use std::path;
use std::io;

pub mod args;

mod backup;
mod link;

pub struct Importer {
    destpath: path::PathBuf,
    srcpath: path::PathBuf
}

impl Importer {
    pub fn from(args: &args::ImporterArgs) -> Importer {
        Importer {
            destpath: args.destpath.clone(),
            srcpath: args.srcpath.clone()
        }
    }

    pub fn backup(&self) -> io::Result<()> {
        let mut backup_path = self.destpath.clone();

        let home_folder = self.destpath.file_name().unwrap().to_str().unwrap();

        backup_path.push(
            format!("{}-backup", home_folder)
        );

        let mut file_extension = 1;
        loop {
            let error = fs::create_dir(&backup_path).err();

            match error {
                None => break,
                Some(err) => {
                    if err.kind() == io::ErrorKind::AlreadyExists {
                        backup_path.set_file_name(
                format!("{}-backup{}", home_folder, file_extension)
                        );
                        file_extension += 1;
                    } else {
                        return Err(
                            io::Error::new(err.kind(), "could not create backup directory")
                        );
                    }
                }
            }
        }

        println!("Copying from {} to {}", &self.destpath.display(), backup_path.display());

        backup::backup_existing_files(&self.srcpath, &self.destpath, &backup_path)
    }

    pub fn link(&self) -> std::io::Result<()> {

        // Hard link
        link::link(&self.srcpath, &self.destpath)
    }
}
use std::env;
use std::path;

pub struct ImporterArgs {
    pub srcpath: path::PathBuf,
    pub destpath: path::PathBuf,
    pub backup: bool,
}

fn get_path_buf(path: &str) -> Result<path::PathBuf, String> {
    let path = path::PathBuf::from(path);
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    Ok(path)
}

impl ImporterArgs {
    fn default() -> ImporterArgs {
        let dot_files_path = match env::current_dir() {
            Err(_) => panic!("Could not fetch current working directory"),
            Ok(value) => value.canonicalize().unwrap(),
        };

        let home_path = match env::var("HOME") {
            Err(_) => panic!("Could not find home directory"),
            Ok(value) => value,
        };

        let destpath = path::PathBuf::from(home_path);

        ImporterArgs {
            srcpath: dot_files_path,
            destpath: destpath,
            backup: true,
        }
    }

    pub fn new(args: Vec<String>) -> Result<ImporterArgs, String> {
        let mut importer_args = ImporterArgs::default();

        let mut args = args.iter();

        args.next();

        loop {
            let arg = args.next();
            match arg {
                Some(arg) => match arg.as_str() {
                    "-h" | "--help" => {
                        let help = r#"dotfiles_importer
Lyr-7d1h <Lyr-7d1h@pm.me>

Usage:
  dotfiles_importer [OPTIONS]

Backup existing dotfiles and then hardlink them from source directory to destination directory

OPTIONS:
  -s <path>, --source <path>        path to source directory [default: wd/cwd]
  -d <path>, --destination <path>   path to destination directory [default: $HOME]
  -n, --no-backup                   don't make a backup of the existing dotfiles
              "#;
                        return Err(help.to_string());
                    }
                    "-d" | "--destination" => {
                        let path_option = args.next();

                        match path_option {
                            Some(path) => importer_args.srcpath = get_path_buf(path)?,
                            None => return Err("No argument given for --destination".to_string()),
                        }
                    }
                    "-s" | "--source" => {
                        let path_option = args.next();

                        match path_option {
                            Some(path) => {
                                importer_args.srcpath = get_path_buf(path)?;
                            }
                            None => return Err("No argument given for --source".to_string()),
                        }
                    }
                    "-n" | "--no-backup" => importer_args.backup = false,
                    "-t" | "--test" => {
                        println!("Using test paths..");
                        importer_args.srcpath = path::PathBuf::from("test-config/new_config");
                        importer_args.destpath = path::PathBuf::from("test-config/config");
                    }
                    _ => {
                        return Err(format!("Invalid option: {}", arg));
                    }
                },
                None => break,
            }
        }

        Ok(importer_args)
    }
}

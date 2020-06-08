use std::env;
use std::path;

pub struct ImporterArgs {
  pub srcpath: path::PathBuf,
  pub destpath: path::PathBuf,
  pub backup: bool,
}

// TODO: Improve argument parsing
impl ImporterArgs {
  fn default() -> ImporterArgs {
    let dot_files_path  = match env::current_dir() {
      Err(_) => panic!("Could not fetch current working directory"),
      Ok(value) => value.canonicalize().unwrap() 
    };

    let home_path = match env::var("HOME") {
      Err(_) => panic!("Could not find home directory"),
      Ok(value) => value
    };

    let destpath = path::PathBuf::from(home_path);

    ImporterArgs {
      srcpath: dot_files_path,
      destpath: destpath,
      backup: true
    }
  }

  pub fn new() -> Result<ImporterArgs, &'static str> {
    let mut args: Vec<String> = env::args().collect();

    let mut importer_args = ImporterArgs::default();

    args.remove(0);

    for (i, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "-h" | "--help" => {
          let help = r#"dotfiles_importer
Lyr-7d1h <Lyr-7d1h@pm.me>

Usage:
  importer [OPTIONS]

Backup existing dotfiles and then hardlink them from source directory to destination directory

OPTIONS:
  -s <path>, --source <path>        path to source directory [default: wd/cwd]
  -d <path>, --destination <path>   path to destination directory [default: $HOME]
  -n, --no-backup                   don't make a backup of the existing dotfiles
  -t, --test                        use the test directory (test-config)
          "#;
          return Err(help);
        },
        "-s" | "--source" => {
          let source_path = args.get(i + 1);
          
          match source_path {
            Some(path) => importer_args.srcpath = path::PathBuf::from(path),
            None => return Err("No argument given for -s")
          }
        },
        "-n" | "--no-backup" => importer_args.backup = false,
        "-t" | "--test" => {
          println!("Using test paths..");
          importer_args.srcpath = path::PathBuf::from("test-config/new_config");
          importer_args.destpath = path::PathBuf::from("test-config/config");
        },
        _ => {
          let error = &format!("Invalid option: {}", arg);

          return Err(error.as_str());
        }
      }
    }

    Ok(importer_args)
  }
}
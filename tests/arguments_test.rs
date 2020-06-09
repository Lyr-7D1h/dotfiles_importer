use dotfiles_importer::args::ImporterArgs;
use std::path;

fn create_arguments(args: Vec<&str>) -> Vec<String> {
  args.iter().map(|arg| arg.to_string()).collect()
}

#[test] 
fn validation_works() {
  let no_src = ImporterArgs::new(
    create_arguments(vec!["asdf", "-s"])
  );
  let no_dest = ImporterArgs::new(
    create_arguments(vec!["asdf", "--no-backup", "--destination"])
  );
  let invalid_path = ImporterArgs::new(
    create_arguments(vec!["asdf", "--source", "asdf"])
  );
  let invalid_option = ImporterArgs::new(
    create_arguments(vec!["asdf", "asdf"])
  );
 
  assert_eq!(no_src.err(), Some("No argument given for --source".to_string()));
  assert_eq!(no_dest.err(), Some("No argument given for --destination".to_string()));
  assert_eq!(invalid_path.err(), Some("Path is not a directory".to_string()));
  assert_eq!(invalid_option.err(), Some("Invalid option: asdf".to_string()));
}

#[test]
fn parsing_works() {
  let multiple_args = ImporterArgs::new(
    create_arguments(vec!["asdf", "-n", "-t"])
  ).unwrap();

  assert_eq!(multiple_args.backup, false);
  assert_eq!(multiple_args.srcpath, path::PathBuf::from("test-config/new_config"));
  assert_eq!(multiple_args.destpath, path::PathBuf::from("test-config/config"));
}
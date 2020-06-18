use dotfiles_importer::Importer;
use dotfiles_importer::args;

use std::io;
use std::process;
use std::env;

///
/// Make sure src_dir does not contain "dest_dir.filename()-backup" as file or directory
///
fn main() {
    let args = env::args().collect();
    let options = args::ImporterArgs::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    println!("\nSource: {:?}\nDestination: {:?}", options.srcpath, options.destpath);
    
    let importer = Importer::from(&options);
        
    if options.backup {
        println!("\nBacking up...");
        match importer.backup() {
            Ok(_) => println!("Backup complete"),
            Err(err) => {
            match err.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("{}", err);
                        panic!("There is no source file")
                } 
                    _ => {
                        eprintln!("Backup failed: {}", err);
                        eprintln!("Continue with linking files? (y/n)");

                        let mut input= String::new();
                        loop {
                            io::stdin().read_line(&mut input).unwrap();

                            let attempt = input.trim().to_lowercase();

                            if attempt == "y" || attempt == "yes" {
                                break;
                            } else if attempt == "n" || attempt == "no" {
                                process::exit(1)
                            } else {
                                eprintln!("Invalid option: '{}'. Type either y or n", attempt);
                                input.clear();
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\nLinking files");

    match importer.link() {
        Ok(_) => println!("Linking complete"),
        Err(err) => eprintln!("Linking failed: {}", err)
    }
}
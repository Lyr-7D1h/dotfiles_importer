# Dotfiles Importer
Very simple cli executable that will backup existing files in the dest folder and then 
[hard links](http://www.linfo.org/hard_link.html) 
the src files to dest folder.

## HowToUse
Simply build the executable using `cargo build --release`.

You can find the executable in `target/release`.

Once you have the executable you should drop it in your dotfiles repo.
By default it will use the current folder as source and your home directory as destination.

From `-h` argument:
```
Usage:
  dotfiles_importer [OPTIONS]

Backup existing dotfiles and then hardlink them from source directory to destination directory

OPTIONS:
  -s <path>, --source <path>        path to source directory [default: wd/cwd]
  -d <path>, --destination <path>   path to destination directory [default: $HOME]
  -n, --no-backup                   don't make a backup of the existing dotfiles
  ```

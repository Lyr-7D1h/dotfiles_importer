# Dotfiles Importer
Very simple cli executable that will backup existing files in the dest folder and then 
[hard links](http://www.linfo.org/hard_link.html) 
the src files to dest folder.

## HowToUse
Simply build the executable using `cargo build --release`.

You can find the executable in `target/release`.

Once you have the executable you should drop it in your dotfiles repo.
By default it will use the current folder as source and your home directory as destination.

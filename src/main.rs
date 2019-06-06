use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

#[derive(Debug)]
struct Dotfile {
    name: String,
    path: PathBuf,
}

impl Dotfile {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn dotname(&self) -> String {
        format!(".{}", self.name)
    }
}

fn symlink_dotfile<P: AsRef<Path>>(home: P, dotfile: &Dotfile) -> Result<(), std::io::Error> {
    let source = {
        let mut path = PathBuf::new();
        path.push(dotfile.path.clone());
        path.push(dotfile.name());
        path
    };
    let destination = {
        let mut path = PathBuf::new();
        path.push(home);
        path.push(dotfile.dotname());
        path
    };

    if cfg!(windows) {
        unimplemented!()
    } else {
        std::os::unix::fs::symlink(source, destination)?;
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "housekeeper")]
struct Args {
    /// The source directory for the dotfiles.
    #[structopt(required = true, parse(from_os_str))]
    dotfiles_directory: PathBuf,

    /// The path to the "home" directory.
    /// Defaults to the user's home directory.
    #[structopt(long = "home", parse(from_os_str))]
    home_directory: Option<PathBuf>,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let home_directory = args
        .home_directory
        .or_else(dirs::home_dir)
        .expect("No home directory set.");

    create_dir_all(&home_directory)?;

    let dotfiles = vec![Dotfile {
        name: "vimrc".into(),
        path: Path::new("./examples/dotfiles/vimrc").to_path_buf(),
    }];

    for dotfile in dotfiles {
        symlink_dotfile(&home_directory, &dotfile)?;
    }

    Ok(())
}

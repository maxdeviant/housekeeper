use std::fs::{canonicalize, create_dir_all, read_dir};
use std::path::{Path, PathBuf};

use structopt::StructOpt;

#[derive(Debug)]
struct Dotfile {
    path: PathBuf,
}

impl Dotfile {
    fn from_path(path: PathBuf) -> Self {
        Self { path }
    }

    fn name(&self) -> String {
        self.path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    }

    fn dotname(&self) -> String {
        format!(".{}", self.name())
    }
}

fn symlink_dotfile<P: AsRef<Path>>(home: P, dotfile: &Dotfile) -> Result<(), std::io::Error> {
    let source = canonicalize(&dotfile.path)?;
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

    let dotfiles = {
        let mut dotfiles: Vec<Dotfile> = Vec::new();

        for entry in read_dir(args.dotfiles_directory)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                dotfiles.push(Dotfile::from_path(path));
            }
        }

        dotfiles
    };

    for dotfile in dotfiles {
        symlink_dotfile(&home_directory, &dotfile)?;
    }

    Ok(())
}

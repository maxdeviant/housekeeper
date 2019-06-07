#[macro_use]
extern crate log;

use std::fs::{canonicalize, create_dir_all, read_dir};
use std::path::PathBuf;

use structopt::StructOpt;

struct Installer {
    home_directory: PathBuf,
    force: bool,
}

trait Install {
    fn install(&self, installer: &Installer) -> Result<(), std::io::Error>;
}

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

impl Install for Dotfile {
    fn install(&self, installer: &Installer) -> Result<(), std::io::Error> {
        let source = canonicalize(&self.path)?;
        let destination = {
            let mut path = PathBuf::new();
            path.push(&installer.home_directory);
            path.push(self.dotname());
            path
        };

        if destination.exists() {
            if destination.is_dir() {
                warn!("{} already exists as a directory!", &self.dotname());
                return Ok(());
            }

            let metadata = std::fs::symlink_metadata(&destination)?;
            if !metadata.file_type().is_symlink() {
                warn!("{} already exists as a file!", &self.dotname());

                if !installer.force {
                    return Ok(());
                }
            }

            std::fs::remove_file(&destination)?;
        }

        info!("Linking {:?} to {:?}", source, destination);

        if cfg!(windows) {
            unimplemented!()
        } else {
            std::os::unix::fs::symlink(source, destination)?;
        }

        Ok(())
    }
}

fn configure_logger() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors = ColoredLevelConfig::new().info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!("{} {}", colors.color(record.level()), message))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
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

    /// Forces the creation of dotfiles, obliterating any files or
    /// directories that are in the way.
    #[structopt(short = "f", long = "force")]
    force: bool,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    configure_logger().expect("Failed to configure logger.");

    let home_directory = args
        .home_directory
        .or_else(dirs::home_dir)
        .expect("No home directory set.");

    create_dir_all(&home_directory)?;

    let dotfiles = {
        let mut dotfiles: Vec<Dotfile> = Vec::new();

        for entry in read_dir(args.dotfiles_directory)? {
            let path = entry?.path();

            if !path.is_dir() {
                dotfiles.push(Dotfile::from_path(path));
            }
        }

        dotfiles
    };

    let installer = Installer {
        home_directory,
        force: args.force,
    };

    for dotfile in dotfiles {
        dotfile.install(&installer)?;
    }

    Ok(())
}

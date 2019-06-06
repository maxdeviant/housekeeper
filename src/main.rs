use std::fs::create_dir_all;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "housekeeper")]
struct Args {
    #[structopt(long = "home", parse(from_os_str))]
    home_directory: Option<PathBuf>,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let home_directory = args
        .home_directory
        .or_else(dirs::home_dir)
        .expect("No home directory set.");

    create_dir_all(home_directory)?;

    Ok(())
}

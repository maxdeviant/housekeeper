use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "housekeeper")]
struct Args {}

#[paw::main]
fn main(_args: Args) -> Result<(), std::io::Error> {
    Ok(())
}

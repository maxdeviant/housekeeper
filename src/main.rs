#[derive(structopt::StructOpt)]
struct Args {}

#[paw::main]
fn main(_args: Args) -> Result<(), std::io::Error> {
    println!("Hello, world!");
    Ok(())
}

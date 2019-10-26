extern crate structopt;
extern crate failure;

use structopt::{StructOpt};
use failure::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "installr")]
struct CliOpt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Git repository to install
    #[structopt()]
    target: String
}

fn main() -> Result<(), Error> {
    let opt = CliOpt::from_args();
    println!("{:#?}", opt);
    Ok(())
}
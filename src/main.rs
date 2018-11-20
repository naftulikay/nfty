/// nfty: Naturally a Foundation of Theology for You
#[macro_use]
extern crate nfty_derive;

pub mod cli;
pub mod logging;
pub mod project;
pub mod util;

use structopt::StructOpt;

fn main() {
    // kick off execution
    cli::Opt::from_args().execute();
}

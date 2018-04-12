extern crate chrono;
extern crate log;
extern crate log4rs;
extern crate rayon;
#[macro_use]
extern crate structopt;

pub mod cli;

use cli::{Opt, Subcommand};
use structopt::StructOpt;

fn main() {
    match Opt::from_args().command {
        Subcommand::Echo { messages } => println!("{}", messages.join(" ")),
    }
}

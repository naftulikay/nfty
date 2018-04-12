extern crate chrono;
extern crate log;
extern crate log4rs;
extern crate rayon;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug,StructOpt)]
struct Opt {
    #[structopt(short="d", long="debug")]
    debug: bool,
    #[structopt(subcommand)]
    command: SubCommand
}

#[derive(Debug,StructOpt)]
enum SubCommand {
    #[structopt(name="echo")]
    Echo {
        messages: Vec<String>,
    },
}

fn main() {
    match Opt::from_args().command {
        SubCommand::Echo { messages } => println!("{}", messages.join(" ")),
    }
}

pub mod project;

#[derive(Debug,StructOpt)]
pub struct Opt {
    #[structopt(short="d", long="debug")]
    pub debug: bool,
    #[structopt(subcommand)]
    pub command: Subcommand
}

#[derive(Debug,StructOpt)]
pub enum Subcommand {
    #[structopt(name="echo")]
    Echo {
        messages: Vec<String>,
    },
}



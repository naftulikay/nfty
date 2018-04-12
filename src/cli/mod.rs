pub mod project;

use structopt::clap::AppSettings;

#[structopt(raw(global_settings="&[AppSettings::ColoredHelp]"))]
#[derive(Debug,StructOpt)]
pub struct Opt {
    /// Emit debug-level log statements.
    #[structopt(short="d", long="debug")]
    pub debug: bool,
    /// Emit info-level log statements. If neither --verbose nor --debug are specified, the default
    /// log level is set to warnings and errors only.
    #[structopt(short="v", long="verbose")]
    pub verbose: bool,
    /// Write logs to syslog using syslog(3).
    #[structopt(long="syslog")]
    pub syslog: bool,
    /// Format logs as JSON lines. Default is plaintext.
    #[structopt(short="j", long="json")]
    pub json: bool,
    #[structopt(subcommand)]
    pub command: Subcommand
}

#[derive(Debug,StructOpt)]
pub enum Subcommand {
    /// Echo text to standard output.
    #[structopt(name="echo")]
    Echo {
        messages: Vec<String>,
    },
}



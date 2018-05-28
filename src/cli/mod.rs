pub mod bring;
pub mod engage;

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

impl Opt {

    pub fn execute(&self) {
        match self.command {
            Subcommand::Bring(ref c) => c.execute(),
            Subcommand::Engage(ref c) => c.execute(),
        }
    }
}

#[derive(Debug,StructOpt)]
pub enum Subcommand {
    /// Bring down a remote project.
    #[structopt(name="bring")]
    Bring(bring::Bring),
    /// Manage software projects.
    #[structopt(name="engage")]
    Engage(engage::Engage),
}

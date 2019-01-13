pub mod project;

use crate::logging;

use structopt::StructOpt;
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

    fn configure_logging(&self) {
        let output = match self.syslog {
            true  => logging::LoggingOutput::Syslog,
            false => logging::LoggingOutput::Console(logging::ConsoleStream::Stderr)
        };

        let format = if self.json {
            logging::LoggingFormat::Json
        } else if self.verbose {
            logging::LoggingFormat::Verbose
        } else {
            logging::LoggingFormat::Plain
        };

        let level = if self.debug {
            logging::LoggingLevel::Verbose
        } else {
            logging::LoggingLevel::Normal
        };

        logging::init(&output, &format, &level);
    }

    pub fn execute(&self) {
        self.configure_logging();

        match self.command {
            Subcommand::Project(ref c) => c.execute(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Manage software projects.
    #[structopt(name="project")]
    Project(crate::cli::project::Project),
}

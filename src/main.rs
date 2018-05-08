extern crate chrono;
extern crate git2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate log4rs_syslog;
extern crate parking_lot;
extern crate pbr;
extern crate rayon;
extern crate regex;
#[macro_use]
extern crate structopt;

pub mod cli;

use cli::data::{Opt, Subcommand};

use log::LevelFilter;

use log4rs::append::Append;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::encode::Encode;
use log4rs::encode::json::JsonEncoder;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs_syslog::{Facility, LogOption, SyslogAppender};

use structopt::StructOpt;

static BINARY_NAME: &'static str = env!("CARGO_PKG_NAME");

static LOGGING_FORMAT: &'static str = "{d(%Y-%m-%dT%H:%M:%S%.3f%z)} {l:5.5} [{T}] {M}: {m}{n}";


fn configure_logging(verbose: bool, debug: bool, json: bool, syslog: bool) {
    let encoder: Box<Encode> = if json {
        Box::new(JsonEncoder::new())
    } else {
        Box::new(PatternEncoder::new(LOGGING_FORMAT))
    };

    let appender: Box<Append> = if syslog {
        Box::new(SyslogAppender::builder().encoder(encoder).openlog(BINARY_NAME, LogOption::LOG_PID, Facility::User).build())
    } else {
        Box::new(ConsoleAppender::builder().encoder(encoder).target(Target::Stderr).build())
    };

    let appender = Appender::builder().build("output", appender);
    let root_level = if debug {
        LevelFilter::Debug
    } else if verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };

    let root = Root::builder().appender("output").build(root_level);

    let config = Config::builder()
        .appender(appender)
        .build(root)
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn main() {
    let options = Opt::from_args();

    // configure logging for console/syslog, json/text, verbose/debug
    configure_logging(options.verbose, options.debug, options.json, options.syslog);

    match options.command {
        Subcommand::Project(e) => cli::project::execute(e),
    }
}

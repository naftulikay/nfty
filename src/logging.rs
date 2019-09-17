use std::path::PathBuf;

use log::LevelFilter;

use log4rs::append::Append;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::encode::Encode;
use log4rs::encode::json::JsonEncoder;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs_syslog::{Facility, LogOption, SyslogAppender};

use parking_lot::{Once, ONCE_INIT};

static INITIALIZE: Once = ONCE_INIT;

static BINARY_NAME: &'static str = env!("CARGO_PKG_NAME");

static PLAIN_LOGGING_FORMAT: &'static str = "{m}{n}";
static VERBOSE_LOGGING_FORMAT: &'static str = "{d(%Y-%m-%dT%H:%M:%S%.3f%z)} {l:5.5} [{T}] {M}: {m}{n}";

pub enum ConsoleStream {
    Stderr,
    Stdout,
}

pub enum LoggingOutput {
    Console(ConsoleStream),
    Disk(PathBuf),
    Syslog,
}

pub enum LoggingFormat {
    Json,
    Plain,
    Verbose,
}

pub enum LoggingLevel {
    Verbose,
    Normal,
}

pub fn init(output: &LoggingOutput, format: &LoggingFormat, level: &LoggingLevel) {
    // subsequent calls to this method should not do anything
    INITIALIZE.call_once(move || {
        init_once(output, format, level);
    });
}

fn init_once(output: &LoggingOutput, format: &LoggingFormat, level: &LoggingLevel) {
    // TODO there has to be a way to minify this
    let encoder: Box<dyn Encode> = match format {
        LoggingFormat::Json    => Box::new(JsonEncoder::new()),
        LoggingFormat::Plain   => Box::new(PatternEncoder::new(PLAIN_LOGGING_FORMAT)),
        LoggingFormat::Verbose => Box::new(PatternEncoder::new(VERBOSE_LOGGING_FORMAT)),
    };

    let appender: Box<dyn Append> = match output {
        LoggingOutput::Console(dest) => Box::new(ConsoleAppender::builder().encoder(encoder).target(match dest {
            ConsoleStream::Stderr => Target::Stderr,
            ConsoleStream::Stdout => Target::Stdout,
        }).build()),
        LoggingOutput::Syslog => Box::new(SyslogAppender::builder().encoder(encoder).openlog(
            BINARY_NAME, LogOption::LOG_PID, Facility::User
        ).build()),
        LoggingOutput::Disk(_) => unreachable!("Not supported yet."),
    };

    let appender_name = match output {
        LoggingOutput::Console(dest) => match dest {
            ConsoleStream::Stderr => "stderr",
            ConsoleStream::Stdout => "stdout",
        },
        LoggingOutput::Syslog => "syslog",
        LoggingOutput::Disk(_) => "disk",
    };

    log4rs::init_config(
        Config::builder()
            .appender(Appender::builder().build(appender_name, appender))
            .build(Root::builder().appender(appender_name).build(match level {
                LoggingLevel::Verbose => LevelFilter::Debug,
                LoggingLevel::Normal  => LevelFilter::Info,
            }))
            .unwrap()
    ).unwrap();
}

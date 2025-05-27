//This is a configuration file for the Library crate.

pub enum LogLevel
{
    Error,
    Warning,
    Info,
    Debug,
}

pub enum LogOutput
{
    Stdout,
    Stderr,
    File(String), // File path as a string
}

///this struct contains the configuration for the Library crate.
/// #Examples:
/// ```
/// use Library::config::{Config, LogLevel, LogOutput};
/// let config = Config(enabled: false, log_level: LogLevel::Debug, log_output: LogOutput::Stderr);
/// ```


pub struct Config
{
    pub enabled : bool,
    pub log_level: LogLevel,
    pub log_output: LogOutput,
}

impl Config {
    pub fn new() -> Self{
        Self{
            enabled: true,
            log_level : LogLevel::Info,
            log_output: LogOutput::Stdout,
        }
    }
}
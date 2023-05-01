use solana_program::msg;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum Splog {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

/// Solana program logs.
pub fn splog_log(splog: Splog, message: &str) {
    SploggerTerminal::print(splog, message);
}

/// General-use macro for Splogging.
#[macro_export]
macro_rules! splog {
    ($log:expr, $msg:expr) => {$crate::splog_log($log, $msg)};
    ($log:expr, $($arg:tt)*) => ($crate::splog_log($log, &format!($($arg)*)));
}

/// Log a debug message.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::splog!(Splog::DEBUG, &format!($($arg)*)));
}

/// Log an info message.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::splog!(Splog::INFO, &format!($($arg)*)));
}

/// Log a warn message.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::splog!(Splog::WARN, &format!($($arg)*)));
}

/// Log an error message.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::splog!(Splog::ERROR, &format!($($arg)*)));
}

/// Log a critical message.
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => ($crate::splog!(Splog::CRITICAL, &format!($($arg)*)));
}

/// Terminal color controls.
pub struct SploggerTerminal {}

impl SploggerTerminal {
    /// Initalize control over the terminal color & output.
    pub fn print(splog: Splog, message: &str) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut colspec = ColorSpec::new();
        match splog {
            Splog::DEBUG => {
                msg!("[DEBUG]: {}", message);
            }
            Splog::INFO => {
                colspec.set_fg(Some(Color::Cyan)).set_bold(true);
                stdout.set_color(&colspec).unwrap();
                msg!("[INFO]: {}", message);
            }
            Splog::WARN => {
                colspec.set_fg(Some(Color::Yellow)).set_bold(true);
                stdout.set_color(&colspec).unwrap();
                msg!("[WARN]: {}", message);
            }
            Splog::ERROR => {
                colspec.set_fg(Some(Color::Red)).set_bold(true);
                stdout.set_color(&colspec).unwrap();
                msg!("[ERROR]: {}", message);
            }
            Splog::CRITICAL => {
                colspec.set_fg(Some(Color::Red)).set_bold(true);
                stdout.set_color(&colspec).unwrap();
                msg!("[CRITICAL]: {}", message);
            }
        };
        stdout.reset().unwrap();
    }
}

use solana_program::msg;

pub enum Splog {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

pub fn splog_log(splog: Splog, message: &str) {
    match splog {
        Splog::DEBUG => msg!("[DEBUG]: {}", message),
        Splog::INFO => msg!("[INFO]: {}", message),
        Splog::WARN => msg!("[WARN]: {}", message),
        Splog::ERROR => msg!("[ERROR]: {}", message),
        Splog::CRITICAL => msg!("[CRITICAL]: {}", message),
    }
}

#[macro_export]
macro_rules! splog {
    ($log:expr, $msg:expr) => {$crate::splog_log($log, $msg)};
    ($log:expr, $($arg:tt)*) => ($crate::splog_log($log, &format!($($arg)*)));
}

#[macro_export]
macro_rules! splog_debug {
    ($($arg:tt)*) => ($crate::splog!(Splog::DEBUG, &format!($($arg)*)));
}

#[macro_export]
macro_rules! splog_info {
    ($($arg:tt)*) => ($crate::splog!(Splog::INFO, &format!($($arg)*)));
}

#[macro_export]
macro_rules! splog_warn {
    ($($arg:tt)*) => ($crate::splog!(Splog::WARN, &format!($($arg)*)));
}

#[macro_export]
macro_rules! splog_error {
    ($($arg:tt)*) => ($crate::splog!(Splog::ERROR, &format!($($arg)*)));
}

#[macro_export]
macro_rules! splog_critical {
    ($($arg:tt)*) => ($crate::splog!(Splog::CRITICAL, &format!($($arg)*)));
}
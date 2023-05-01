use splogger::{critical, debug, error, info, splog, warn, Splog};

pub fn impl_test() {
    splog!(Splog::DEBUG, "This is a debug message.");
    splog!(Splog::INFO, "This is an info message.");
    splog!(Splog::WARN, "This is a warning message.");
    splog!(Splog::ERROR, "This is an error message.");
    splog!(Splog::CRITICAL, "This is a critical message.");

    debug!("This is a debug message.");
    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
    critical!("This is a critical message.");

    const JOE: &'static str = "Joe";

    splog!(Splog::INFO, "This is an info message about {}.", JOE);

    debug!("This is a debug message about {}.", JOE);
    info!("This is an info message about {}.", JOE);
    warn!("This is a warning message about {}.", JOE);
    error!("This is an error message about {}.", JOE);
    critical!("This is a critical message about {}.", JOE);
}

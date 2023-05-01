use splogger::*;

pub fn impl_test() {
    splog!(Splog::DEBUG, "This is a debug message.");
    splog!(Splog::INFO, "This is an info message.");
    splog!(Splog::WARN, "This is a warning message.");
    splog!(Splog::ERROR, "This is an error message.");
    splog!(Splog::CRITICAL, "This is a critical message.");

    splog_debug!("This is a debug message.");
    splog_info!("This is an info message.");
    splog_warn!("This is a warning message.");
    splog_error!("This is an error message.");
    splog_critical!("This is a critical message.");

    const JOE: &'static str = "Joe";

    splog!(Splog::INFO, "This is an info message about {}.", JOE);

    splog_debug!("This is a debug message about {}.", JOE);
    splog_info!("This is an info message about {}.", JOE);
    splog_warn!("This is a warning message about {}.", JOE);
    splog_error!("This is an error message about {}.", JOE);
    splog_critical!("This is a critical message about {}.", JOE);
}

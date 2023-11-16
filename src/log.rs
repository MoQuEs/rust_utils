use log::{log, Level};
use std::fmt::Debug;

pub trait Log {
    fn log(self, level: Level, target: &str, message: impl AsRef<str>) -> Self;
    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self;
}

pub trait LogUnwrap<T> {
    fn log_u(self, level: Level, target: &str, message: impl AsRef<str>) -> T;
    fn log_error_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_warn_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_info_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_debug_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_trace_u(self, target: &str, message: impl AsRef<str>) -> T;
}

pub trait LogDebug {
    fn log_d(self, level: Level, target: &str, message: impl AsRef<str>) -> Self;
    fn log_error_d(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_warn_d(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_info_d(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_debug_d(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_trace_d(self, target: &str, message: impl AsRef<str>) -> Self;
}

impl<T> Log for Option<T> {
    fn log(self, level: Level, target: &str, message: impl AsRef<str>) -> Self {
        if self.is_none() {
            log!(
                target: target,
                level,
                "Empty option; Message: {}",
                message.as_ref()
            );
        }

        self
    }

    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Error, target, message)
    }

    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Warn, target, message)
    }

    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Info, target, message)
    }

    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Debug, target, message)
    }

    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Trace, target, message)
    }
}

impl<T> LogUnwrap<T> for Option<T> {
    fn log_u(self, level: Level, target: &str, message: impl AsRef<str>) -> T {
        self.log(level, target, message.as_ref())
            .unwrap_or_else(|| panic!("{}", message.as_ref().to_string()))
    }

    fn log_error_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Error, target, message)
    }

    fn log_warn_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Warn, target, message)
    }

    fn log_info_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Info, target, message)
    }

    fn log_debug_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Debug, target, message)
    }

    fn log_trace_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Trace, target, message)
    }
}

impl<T: Debug> LogDebug for Option<T> {
    fn log_d(self, level: Level, target: &str, message: impl AsRef<str>) -> Self {
        if self.is_some() {
            log!(target: target, level, "Value: {:?}", message.as_ref());
        }

        self
    }

    fn log_error_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Error, target, message)
    }

    fn log_warn_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Warn, target, message)
    }

    fn log_info_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Info, target, message)
    }

    fn log_debug_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Debug, target, message)
    }

    fn log_trace_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Trace, target, message)
    }
}

impl<T, E: Debug> Log for Result<T, E> {
    fn log(self, level: Level, target: &str, message: impl AsRef<str>) -> Self {
        self.map_err(|err| {
            log!(
                target: target,
                level,
                "Message: {}; Error: {:?};",
                message.as_ref(),
                err
            );

            err
        })
    }

    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Error, target, message)
    }

    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Warn, target, message)
    }

    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Info, target, message)
    }

    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Debug, target, message)
    }

    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(Level::Trace, target, message)
    }
}

impl<T, E: Debug> LogUnwrap<T> for Result<T, E> {
    fn log_u(self, level: Level, target: &str, message: impl AsRef<str>) -> T {
        self.log(level, target, message.as_ref())
            .unwrap_or_else(|_| panic!("{}", message.as_ref().to_string()))
    }

    fn log_error_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Error, target, message)
    }

    fn log_warn_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Warn, target, message)
    }

    fn log_info_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Info, target, message)
    }

    fn log_debug_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Debug, target, message)
    }

    fn log_trace_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(Level::Trace, target, message)
    }
}

impl<T: Debug, E> LogDebug for Result<T, E> {
    fn log_d(self, level: Level, target: &str, message: impl AsRef<str>) -> Self {
        if self.is_ok() {
            log!(target: target, level, "Value: {:?}", message.as_ref());
        }

        self
    }

    fn log_error_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Error, target, message)
    }

    fn log_warn_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Warn, target, message)
    }

    fn log_info_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Info, target, message)
    }

    fn log_debug_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Debug, target, message)
    }

    fn log_trace_d(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log_d(Level::Trace, target, message)
    }
}

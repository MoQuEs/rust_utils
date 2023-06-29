use std::fmt::Debug;

pub trait Log {
    fn log(self, level: log::Level, target: &str, message: impl AsRef<str>) -> Self;
    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self;
}

pub trait LogU<T> {
    fn log_u(self, level: log::Level, target: &str, message: impl AsRef<str>) -> T;
    fn log_error_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_warn_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_info_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_debug_u(self, target: &str, message: impl AsRef<str>) -> T;
    fn log_trace_u(self, target: &str, message: impl AsRef<str>) -> T;
}

impl<T> Log for Option<T> {
    fn log(self, level: log::Level, target: &str, message: impl AsRef<str>) -> Self {
        if self.is_none() {
            log::log!(
                target: target,
                level,
                "Empty option; Message: {}",
                message.as_ref()
            );
        }

        self
    }

    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Error, target, message)
    }

    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Warn, target, message)
    }

    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Info, target, message)
    }

    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Debug, target, message)
    }

    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Trace, target, message)
    }
}

impl<T> LogU<T> for Option<T> {
    fn log_u(self, level: log::Level, target: &str, message: impl AsRef<str>) -> T {
        self.log(level, target, message.as_ref())
            .unwrap_or_else(|| panic!("{}", message.as_ref().to_string()))
    }

    fn log_error_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Error, target, message)
    }

    fn log_warn_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Warn, target, message)
    }

    fn log_info_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Info, target, message)
    }

    fn log_debug_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Debug, target, message)
    }

    fn log_trace_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Trace, target, message)
    }
}

impl<T, E: Debug> Log for Result<T, E> {
    fn log(self, level: log::Level, target: &str, message: impl AsRef<str>) -> Self {
        self.map_err(|err| {
            log::log!(
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
        self.log(log::Level::Error, target, message)
    }

    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Warn, target, message)
    }

    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Info, target, message)
    }

    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Debug, target, message)
    }

    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self {
        self.log(log::Level::Trace, target, message)
    }
}

impl<T, E: Debug> LogU<T> for Result<T, E> {
    fn log_u(self, level: log::Level, target: &str, message: impl AsRef<str>) -> T {
        self.log(level, target, message.as_ref())
            .unwrap_or_else(|_| panic!("{}", message.as_ref().to_string()))
    }

    fn log_error_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Error, target, message)
    }

    fn log_warn_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Warn, target, message)
    }

    fn log_info_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Info, target, message)
    }

    fn log_debug_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Debug, target, message)
    }

    fn log_trace_u(self, target: &str, message: impl AsRef<str>) -> T {
        self.log_u(log::Level::Trace, target, message)
    }
}

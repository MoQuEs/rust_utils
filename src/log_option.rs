use crate::log::{Log, LogU};

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
        if self.is_none() {
            log::log!(
                target: target,
                level,
                "Empty option; Message: {}",
                message.as_ref()
            );
        }

        self.unwrap()
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

use std::fmt::Debug;

pub trait LogErrorFromResult {
    fn log(self, level: log::Level, target: &str, message: impl AsRef<str>) -> Self;
    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self;
}

impl<T, E: Debug> LogErrorFromResult for Result<T, E> {
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

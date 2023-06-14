pub trait LogErrorFromOption {
    fn log(self, level: log::Level, target: &str, message: impl AsRef<str>) -> Self;
    fn log_error(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_warn(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_info(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_debug(self, target: &str, message: impl AsRef<str>) -> Self;
    fn log_trace(self, target: &str, message: impl AsRef<str>) -> Self;
}

impl<T> LogErrorFromOption for Option<T> {
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

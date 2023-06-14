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

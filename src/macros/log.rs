#[macro_export]
macro_rules! res_log {
    ($code:expr; $e:ident; target: $target:expr, $lvl:expr, $($arg:tt),+) => {
        match $code {
            Ok(val) => Ok(val),
            Err($e) => {
                log::log!(target: $target, $lvl, $($arg),+);
                Err($e)
            }
        }
    };

    ($code:expr; target: $target:expr, $lvl:expr, $($arg:tt),+) => {
        match $code {
            Ok(val) => Ok(val),
            Err(err) => {
                log::log!(target: $target, $lvl, $($arg),+);
                Err(err)
            }
        }
    };

    ($code:expr; target: $target:expr, $lvl:expr) => {
        match $code {
            Ok(val) => Ok(val),
            Err(err) => {
                log::log!(target: $target, $lvl, "Error: {err:?}");
                Err(err)
            }
        }
    };

    ($code:expr; $e:ident; $lvl:expr, $($arg:tt),+) => {
        match $code {
            Ok(val) => Ok(val),
            Err($e) => {
                log::log!($lvl, $($arg),+);
                Err($e)
            }
        }
    };

    ($code:expr; $lvl:expr, $($arg:tt),+) => {
        match $code {
            Ok(val) => Ok(val),
            Err(err) => {
                log::log!($lvl, $($arg),+);
                Err(err)
            }
        }
    };

    ($code:expr; $lvl:expr) => {
        match $code {
            Ok(val) => Ok(val),
            Err(err) => {
                log::log!($lvl, "Error: {err:?}");
                Err(err)
            }
        }
    };
}

#[macro_export]
macro_rules! opt_log {
    ($code:expr; target: $target:expr, $lvl:expr, $($arg:tt),+) => {
        match $code {
            Some(val) => Some(val),
            None => {
                log::log!(target: $target, $lvl, $($arg),+);
                None
            }
        }
    };

    ($code:expr; $lvl:expr, $($arg:tt),+) => {
        match $code {
            Some(val) => Some(val),
            None => {
                log::log!($lvl, $($arg),+);
                None
            }
        }
    };
}

#[macro_export]
macro_rules! res_error {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; $e; target: $target, log::Level::Error, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; target: $target, log::Level::Error, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        res_log!($code; target: $target, log::Level::Error)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        res_log!($code; $e; log::Level::Error, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        res_log!($code; log::Level::Error, $($arg),+)
    };

    ($code:expr;) => {
        res_log!($code; log::Level::Error)
    };
}

#[macro_export]
macro_rules! opt_error {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        opt_log!($code; target: $target, log::Level::Error, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        opt_log!($code; log::Level::Error, $($arg),+)
    };
}

#[macro_export]
macro_rules! res_warn {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; $e; target: $target, log::Level::Warn, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; target: $target, log::Level::Warn, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        res_log!($code; target: $target, log::Level::Warn)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        res_log!($code; $e; log::Level::Warn, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        res_log!($code; log::Level::Warn, $($arg),+)
    };

    ($code:expr;) => {
        res_log!($code; log::Level::Warn)
    };
}

#[macro_export]
macro_rules! opt_warn {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        opt_log!($code; target: $target, log::Level::Warn, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        opt_log!($code; log::Level::Warn, $($arg),+)
    };
}

#[macro_export]
macro_rules! res_info {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; $e; target: $target, log::Level::Info, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; target: $target, log::Level::Info, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        res_log!($code; target: $target, log::Level::Info)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        res_log!($code; $e; log::Level::Info, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        res_log!($code; log::Level::Info, $($arg),+)
    };

    ($code:expr;) => {
        res_log!($code; log::Level::Info)
    };
}

#[macro_export]
macro_rules! opt_info {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        opt_log!($code; target: $target, log::Level::Info, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        opt_log!($code; log::Level::Info, $($arg),+)
    };
}

#[macro_export]
macro_rules! res_debug {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; $e; target: $target, log::Level::Debug, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; target: $target, log::Level::Debug, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        res_log!($code; target: $target, log::Level::Debug)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        res_log!($code; $e; log::Level::Debug, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        res_log!($code; log::Level::Debug, $($arg),+)
    };

    ($code:expr;) => {
        res_log!($code; log::Level::Debug)
    };
}

#[macro_export]
macro_rules! opt_debug {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        opt_log!($code; target: $target, log::Level::Debug, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        opt_log!($code; log::Level::Debug, $($arg),+)
    };
}

#[macro_export]
macro_rules! res_trace {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; $e; target: $target, log::Level::Trace, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        res_log!($code; target: $target, log::Level::Trace, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        res_log!($code; target: $target, log::Level::Trace)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        res_log!($code; $e; log::Level::Trace, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        res_log!($code; log::Level::Trace, $($arg),+)
    };

    ($code:expr;) => {
        res_log!($code; log::Level::Trace)
    };
}

#[macro_export]
macro_rules! opt_trace {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        opt_log!($code; target: $target, log::Level::Trace, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        opt_log!($code; log::Level::Trace, $($arg),+)
    };
}

#[cfg(test)]
mod test {
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Clone)]
    struct Error(String);

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "err {}", self.0)
        }
    }

    impl std::error::Error for Error {}

    fn ok(inner: &str) -> Result<&str, &str> {
        Ok(inner)
    }

    fn err_s(inner: &str) -> Result<&str, &str> {
        Err(inner)
    }

    fn err_impl(inner: &str) -> Result<&str, Box<dyn std::error::Error>> {
        Err(Error(inner.to_string()).into())
    }

    fn some(inner: &str) -> Option<&str> {
        Some(inner)
    }

    fn none(_inner: &str) -> Option<&str> {
        None
    }

    macro_rules! test_maker_result {
        ($f:ident) => {
            let _ = res_error!(
                $f("a"); err;
                target: "target",
                "0 {err}"
            );

            let _ = res_warn!(
                $f("b");
                target: "target",
                "1"
            );

            let _ = res_info!(
                $f("c");
                target: "target"
            );

            let _ = res_debug!(
                $f("d"); err;
                "3 {err}"
            );

            let _ = res_trace!(
                $f("e");
                "4"
            );

            let _ = res_error!(
                $f("f");
            );
        };
    }

    macro_rules! test_maker_option {
        ($f:ident) => {
            let _ = opt_warn!(
                $f("b");
                target: "target",
                "1"
            );

            let _ = opt_trace!(
                $f("e");
                "4"
            );
        };
    }

    #[test]
    fn test_res_ok() {
        testing_logger::setup();

        test_maker_result!(ok);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0);
        });
    }

    #[test]
    fn test_res_err_s() {
        testing_logger::setup();

        test_maker_result!(err_s);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 6);

            assert_eq!(captured_logs[0].body, "0 a");
            assert_eq!(captured_logs[0].level, log::Level::Error);

            assert_eq!(captured_logs[1].body, "1");
            assert_eq!(captured_logs[1].level, log::Level::Warn);

            assert_eq!(captured_logs[2].body, "Error: \"c\"");
            assert_eq!(captured_logs[2].level, log::Level::Info);

            assert_eq!(captured_logs[3].body, "3 d");
            assert_eq!(captured_logs[3].level, log::Level::Debug);

            assert_eq!(captured_logs[4].body, "4");
            assert_eq!(captured_logs[4].level, log::Level::Trace);

            assert_eq!(captured_logs[5].body, "Error: \"f\"");
            assert_eq!(captured_logs[5].level, log::Level::Error);
        });
    }

    #[test]
    fn test_res_err_impl() {
        testing_logger::setup();

        test_maker_result!(err_impl);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 6);

            assert_eq!(captured_logs[0].body, "0 err a");
            assert_eq!(captured_logs[0].level, log::Level::Error);

            assert_eq!(captured_logs[1].body, "1");
            assert_eq!(captured_logs[1].level, log::Level::Warn);

            assert_eq!(captured_logs[2].body, "Error: Error(\"c\")");
            assert_eq!(captured_logs[2].level, log::Level::Info);

            assert_eq!(captured_logs[3].body, "3 err d");
            assert_eq!(captured_logs[3].level, log::Level::Debug);

            assert_eq!(captured_logs[4].body, "4");
            assert_eq!(captured_logs[4].level, log::Level::Trace);

            assert_eq!(captured_logs[5].body, "Error: Error(\"f\")");
            assert_eq!(captured_logs[5].level, log::Level::Error);
        });
    }

    #[test]
    fn test_opt_some() {
        testing_logger::setup();

        test_maker_option!(some);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0);
        });
    }

    #[test]
    fn test_res_error_err_s() {
        testing_logger::setup();

        test_maker_option!(none);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);

            assert_eq!(captured_logs[0].body, "1");
            assert_eq!(captured_logs[0].level, log::Level::Warn);

            assert_eq!(captured_logs[1].body, "4");
            assert_eq!(captured_logs[1].level, log::Level::Trace);
        });
    }
}

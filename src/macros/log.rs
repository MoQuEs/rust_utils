pub use log;

#[macro_export]
macro_rules! result_log {
    ($code:expr; $e:ident; target: $target:expr, $lvl:ident, $($arg:tt),+) => {
        $code.inspect_err(|$e| $crate::macros::log::log::log!(target: $target, $crate::macros::log::log::Level::$lvl, $($arg),+))
    };

    ($code:expr; target: $target:expr, $lvl:ident, $($arg:tt),+) => {
        $code.inspect_err(|_| $crate::macros::log::log::log!(target: $target, $crate::macros::log::log::Level::$lvl, $($arg),+))
    };

    ($code:expr; target: $target:expr, $lvl:ident) => {
        $code.inspect_err(|err| $crate::macros::log::log::log!(target: $target, $crate::macros::log::log::Level::$lvl, "Error: {err:?}"))
    };

    ($code:expr; $e:ident; $lvl:ident, $($arg:tt),+) => {
        $code.inspect_err(|$e| $crate::macros::log::log::log!($crate::macros::log::log::Level::$lvl, $($arg),+))
    };

    ($code:expr; $lvl:ident, $($arg:tt),+) => {
        $code.inspect_err(|_| $crate::macros::log::log::log!($crate::macros::log::log::Level::$lvl, $($arg),+))
    };

    ($code:expr; $lvl:ident) => {
        $code.inspect_err(|err| $crate::macros::log::log::log!($crate::macros::log::log::Level::$lvl, "Error: {err:?}"))
    };
}

#[macro_export]
macro_rules! option_log {
    ($code:expr; target: $target:expr, $lvl:ident, $($arg:tt),+) => {
        if $code.is_none() {
            $crate::macros::log::log::log!(target: $target, $crate::macros::log::log::Level::$lvl, $($arg),+);
        }
    };

    ($code:expr; $lvl:ident, $($arg:tt),+) => {
        if $code.is_none() {
            $crate::macros::log::log::log!($crate::macros::log::log::Level::$lvl, $($arg),+);
        }
    };
}

#[macro_export]
macro_rules! result_error {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; $e; target: $target, Error, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; target: $target, Error, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        $crate::result_log!($code; target: $target, Error)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        $crate::result_log!($code; $e; Error, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::result_log!($code; Error, $($arg),+)
    };

    ($code:expr;) => {
        $crate::result_log!($code; Error)
    };
}

#[macro_export]
macro_rules! option_error {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::option_log!($code; target: $target, Error, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::option_log!($code; Error, $($arg),+)
    };
}

#[macro_export]
macro_rules! result_warn {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; $e; target: $target, Warn, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; target: $target, Warn, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        $crate::result_log!($code; target: $target, Warn)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        $crate::result_log!($code; $e; Warn, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::result_log!($code; Warn, $($arg),+)
    };

    ($code:expr;) => {
        $crate::result_log!($code; Warn)
    };
}

#[macro_export]
macro_rules! option_warn {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::option_log!($code; target: $target, Warn, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::option_log!($code; Warn, $($arg),+)
    };
}

#[macro_export]
macro_rules! result_info {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; $e; target: $target, Info, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; target: $target, Info, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        $crate::result_log!($code; target: $target, Info)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        $crate::result_log!($code; $e; Info, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::result_log!($code; Info, $($arg),+)
    };

    ($code:expr;) => {
        $crate::result_log!($code; Info)
    };
}

#[macro_export]
macro_rules! option_info {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::option_log!($code; target: $target, Info, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::option_log!($code; Info, $($arg),+)
    };
}

#[macro_export]
macro_rules! result_debug {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; $e; target: $target, Debug, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; target: $target, Debug, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        $crate::result_log!($code; target: $target, Debug)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        $crate::result_log!($code; $e; Debug, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::result_log!($code; Debug, $($arg),+)
    };

    ($code:expr;) => {
        $crate::result_log!($code; Debug)
    };
}

#[macro_export]
macro_rules! option_debug {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::option_log!($code; target: $target, Debug, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::option_log!($code; Debug, $($arg),+)
    };
}

#[macro_export]
macro_rules! result_trace {
    ($code:expr; $e:ident; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; $e; target: $target, Trace, $($arg),+)
    };

    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::result_log!($code; target: $target, Trace, $($arg),+)
    };

    ($code:expr; target: $target:expr) => {
        $crate::result_log!($code; target: $target, Trace)
    };

    ($code:expr; $e:ident; $($arg:tt),+) => {
        $crate::result_log!($code; $e; Trace, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::result_log!($code; Trace, $($arg),+)
    };

    ($code:expr;) => {
        $crate::result_log!($code; Trace)
    };
}

#[macro_export]
macro_rules! option_trace {
    ($code:expr; target: $target:expr, $($arg:tt),+) => {
        $crate::option_log!($code; target: $target, Trace, $($arg),+)
    };

    ($code:expr; $($arg:tt),+) => {
        $crate::option_log!($code; Trace, $($arg),+)
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
            let _ = result_error!(
                $f("a"); err;
                target: "target",
                "0 {err}"
            );

            let _ = result_warn!(
                $f("b");
                target: "target",
                "1"
            );

            let _ = result_info!(
                $f("c");
                target: "target"
            );

            let _ = result_debug!(
                $f("d"); err;
                "3 {err}"
            );

            let _ = result_trace!(
                $f("e");
                "4"
            );

            let _ = result_error!(
                $f("f");
            );

            let _ = result_log!(
                $f("x"); err;
                target: "target",
                Error,
                "99 {err}"
            );
        };
    }

    macro_rules! test_maker_option {
        ($f:ident) => {
            let _ = option_warn!(
                $f("b");
                target: "target",
                "1"
            );

            let _ = option_trace!(
                $f("e");
                "4"
            );

            let _ = option_log!(
                $f("x");
                target: "target",
                Error,
                "99"
            );
        };
    }

    #[test]
    fn test_result_ok() {
        testing_logger::setup();

        test_maker_result!(ok);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0);
        });
    }

    #[test]
    fn test_result_err_s() {
        testing_logger::setup();

        test_maker_result!(err_s);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 7);

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

            assert_eq!(captured_logs[6].body, "99 x");
            assert_eq!(captured_logs[6].level, log::Level::Error);
        });
    }

    #[test]
    fn test_result_err_impl() {
        testing_logger::setup();

        test_maker_result!(err_impl);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 7);

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

            assert_eq!(captured_logs[6].body, "99 err x");
            assert_eq!(captured_logs[6].level, log::Level::Error);
        });
    }

    #[test]
    fn test_option_some() {
        testing_logger::setup();

        test_maker_option!(some);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0);
        });
    }

    #[test]
    fn test_result_error_err_s() {
        testing_logger::setup();

        test_maker_option!(none);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 3);

            assert_eq!(captured_logs[0].body, "1");
            assert_eq!(captured_logs[0].level, log::Level::Warn);

            assert_eq!(captured_logs[1].body, "4");
            assert_eq!(captured_logs[1].level, log::Level::Trace);

            assert_eq!(captured_logs[2].body, "99");
            assert_eq!(captured_logs[2].level, log::Level::Error);
        });
    }
}

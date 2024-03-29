use std::fmt::Display;

pub trait Ignore: Sized {
    fn ignore(self) {}
}

pub trait PrintAndIgnore {
    fn ignore_and_print_if_error(self);
}

#[cfg(feature = "log")]
pub trait PrintAndLog: Ignore {
    fn ignore_and_log_if_error(self);
}

impl<T, E> Ignore for Result<T, E> {}

impl<T, E: Display> PrintAndIgnore for Result<T, E> {
    fn ignore_and_print_if_error(self) {
        self.map_err(|err| {
            println!("Error: {}", &err);
            err
        })
        .ignore();
    }
}

#[cfg(feature = "log")]
impl<T, E: Display> PrintAndLog for Result<T, E> {
    fn ignore_and_log_if_error(self) {
        self.map_err(|err| {
            log::error!("Error: {}", &err);
            err
        })
        .ignore();
    }
}

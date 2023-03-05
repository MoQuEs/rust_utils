pub trait UnwrapRef<T> {
    fn unwrap_ref(&self) -> &T;
}

impl<T> UnwrapRef<T> for Option<T> {
    fn unwrap_ref(&self) -> &T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap_ref()` on a `None` value"),
        }
    }
}

impl<T, E> UnwrapRef<T> for Result<T, E> {
    fn unwrap_ref(&self) -> &T {
        match self {
            Ok(val) => val,
            Err(_) => panic!("called `Result::unwrap_ref()` on a `None` value"),
        }
    }
}


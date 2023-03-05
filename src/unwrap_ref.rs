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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_unwrap_ref_option() {
        let value = Some("asd");
        assert_eq!(value.unwrap_ref(), &"asd");
    }

    #[test]
    fn call_unwrap_ref_result() {
        let value: Result<&str, ()> = Ok("asd");
        assert_eq!(value.unwrap_ref(), &"asd");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap_ref()` on a `None` value")]
    fn call_unwrap_ref_option_panic() {
        let value: Option<usize> = None;
        value.unwrap_ref();
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap_ref()` on a `None` value")]
    fn call_unwrap_ref_result_panic() {
        let value: Result<usize, ()> = Err(());
        value.unwrap_ref();
    }
}

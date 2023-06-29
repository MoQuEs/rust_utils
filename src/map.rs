pub trait MapRefOption<U> {
    fn map_ref<N>(&self, f: impl FnOnce(&U) -> N) -> Option<N>;
}

pub trait MapRefResult<U, E> {
    fn map_ref<N>(&self, f: impl FnOnce(&U) -> N) -> Result<N, E>;
}

impl<U> MapRefOption<U> for Option<U> {
    fn map_ref<N>(&self, f: impl FnOnce(&U) -> N) -> Option<N> {
        self.as_ref().map(f)
    }
}

impl<U, E: Clone> MapRefResult<U, E> for Result<U, E> {
    fn map_ref<N>(&self, f: impl FnOnce(&U) -> N) -> Result<N, E> {
        match self {
            Ok(u) => Ok(f(u)),
            Err(e) => Err(e.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ref_some() {
        let x = Some(1);
        let y = x.map_ref(|x| x + 1);
        assert_eq!(y, Some(2));
    }

    #[test]
    fn test_map_ref_ok() {
        let x: Result<i32, ()> = Ok(1);
        let y = x.map_ref(|x| x + 1);
        assert_eq!(y, Ok(2));
    }

    #[test]
    fn test_map_ref_none() {
        let x: Option<String> = None;
        let y = x.map_ref(|x| x.parse::<i32>().unwrap());
        assert_eq!(y, None);
    }

    #[test]
    fn test_map_ref_error() {
        let x: Result<i32, &'static str> = Err("");
        let y = x.map_ref(|x| x + 1);
        assert_eq!(y, Err(""));
    }

    #[test]
    fn test_map_ref_none_change_type() {
        let x: Option<String> = None;
        let y = x.map_ref(|x| x.parse::<i32>().unwrap());
        assert_eq!(y, None);
    }

    #[test]
    fn test_map_ref_some_change_type() {
        let x: Option<String> = Some("123".to_string());
        let y = x.map_ref(|x| x.parse::<i32>().unwrap());
        assert_eq!(y, Some(123));
    }
}

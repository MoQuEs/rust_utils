use num::FromPrimitive;

pub fn positive_number<T: Ord + std::str::FromStr + FromPrimitive>(
    s: impl AsRef<str>,
) -> Result<T, String> {
    let number: T = s.as_ref().parse().map_err(|_| format!("`{}` isn't a number", s.as_ref()))?;

    if number > FromPrimitive::from_usize(0).unwrap() {
        Ok(number)
    } else {
        Err("number is not positive (>0)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_positive_number_ok() {
        assert_eq!(positive_number("3"), Ok(3));
        assert_eq!(positive_number("1"), Ok(1));
        assert_eq!(positive_number(i32::MAX.to_string().as_str()), Ok(i32::MAX));
    }

    #[test]
    fn call_positive_number_err() {
        assert!(positive_number::<i32>("0").is_err());
        assert!(positive_number::<i32>("-20").is_err());
        assert!(positive_number::<i32>(i32::MIN.to_string().as_str()).is_err());
    }
}
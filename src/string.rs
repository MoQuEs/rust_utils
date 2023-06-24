use regex::Regex;

pub fn mask_if_email(potential_email: impl AsRef<str>) -> String {
    let re = Regex::new(r"^(.)(.*)(@.*)$").unwrap();
    if re.is_match(potential_email.as_ref()) {
        return re.replace(potential_email.as_ref(), "$1*****$3").to_string();
    }

    potential_email.as_ref().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_email() {
        let value = mask_if_email("asd@a.tsa");
        assert_eq!(value, "a*****@a.tsa");
    }

    #[test]
    fn is_not_email() {
        let value = mask_if_email("asda.tsa");
        assert_eq!(value, "asda.tsa");
    }
}

pub fn mask_if_email(potential_email: impl AsRef<str>) -> String {
    if potential_email.as_ref().contains('@') {
        let mut parts = potential_email.as_ref().split('@');

        let mut new_string = String::new();
        new_string.push_str(&parts.next().unwrap()[0..1]);
        new_string.push_str("***OMITTED***");
        new_string.push('@');
        new_string.push_str(parts.next().unwrap());

        return new_string;
    }

    potential_email.as_ref().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_email() {
        let value = mask_if_email("asd@a.tsa");
        assert_eq!(value, "a***OMITTED***@a.tsa");
    }

    #[test]
    fn is_not_email() {
        let value = mask_if_email("asda.tsa");
        assert_eq!(value, "asda.tsa");
    }
}

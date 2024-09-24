pub fn remove_non_az_09_characters_and_to_lowercase<T: std::fmt::Display>(text: T) -> String {
    text.to_string()
        .to_lowercase()
        .chars()
        .map(|x| match x {
            'a'..='z' => x,
            '0'..='9' => x,
            _ => '_',
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_remove_non_az_09_characters_and_to_lowercase() {
        let value = remove_non_az_09_characters_and_to_lowercase("a b");
        assert_eq!(value, String::from("a_b"));
    }

    #[test]
    fn call_remove_non_az_09_characters_and_to_lowercase_with_special_chars() {
        let value = remove_non_az_09_characters_and_to_lowercase("a b<>:\"/\\|?*");
        assert_eq!(value, String::from("a_b_________"));
    }

    #[test]
    fn call_remove_non_az_09_characters_and_to_lowercase_with_numbers() {
        let value = remove_non_az_09_characters_and_to_lowercase("a b 123");
        assert_eq!(value, String::from("a_b_123"));
    }
}

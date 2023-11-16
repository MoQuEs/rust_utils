pub fn remove_non_az_09_characters_and_to_lowercase<T: std::fmt::Display>(
    text: T,
) -> askama::Result<String> {
    Ok(text
        .to_string()
        .to_lowercase()
        .chars()
        .map(|x| match x {
            'a'..='z' => x,
            '0'..='9' => x,
            _ => '_',
        })
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_remove_non_az_09_characters_and_to_lowercase() {
        let value = remove_non_az_09_characters_and_to_lowercase("a b");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), String::from("a_b"));
    }

    #[test]
    fn call_remove_non_az_09_characters_and_to_lowercase_with_special_chars() {
        let value = remove_non_az_09_characters_and_to_lowercase("a b<>:\"/\\|?*");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), String::from("a_b__________"));
    }

    #[test]
    fn call_remove_non_az_09_characters_and_to_lowercase_with_numbers() {
        let value = remove_non_az_09_characters_and_to_lowercase("a b 123");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), String::from("a_b_123"));
    }
}

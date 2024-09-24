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

pub fn slugify_for_filename(text: impl AsRef<str>) -> String {
    // '<>:"/\|?*' are not allowed in Windows filenames
    text.as_ref()
        .replace(['<', '>', ':', '"', '/', '\\', '?', '*', ' '], "_")
}

pub trait MaskIfEmail {
    fn mask_if_email(&self) -> String;
}

pub trait SlugifyForFilename {
    fn slugify_for_filename(&self) -> String;
}

impl MaskIfEmail for String {
    fn mask_if_email(&self) -> String {
        mask_if_email(self)
    }
}

impl MaskIfEmail for str {
    fn mask_if_email(&self) -> String {
        mask_if_email(self)
    }
}

impl SlugifyForFilename for String {
    fn slugify_for_filename(&self) -> String {
        slugify_for_filename(self)
    }
}

impl SlugifyForFilename for str {
    fn slugify_for_filename(&self) -> String {
        slugify_for_filename(self)
    }
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

    #[test]
    fn is_email_str() {
        let value = "asd@a.tsa".mask_if_email();
        assert_eq!(value, "a***OMITTED***@a.tsa".to_string());
    }

    #[test]
    fn is_not_email_str() {
        let value = "asda.tsa".mask_if_email();
        assert_eq!(value, "asda.tsa".to_string());
    }

    #[test]
    fn is_email_string() {
        let value = "asd@a.tsa".to_string().mask_if_email();
        assert_eq!(value, "a***OMITTED***@a.tsa".to_string());
    }

    #[test]
    fn is_not_email_string() {
        let value = "asda.tsa".to_string().mask_if_email();
        assert_eq!(value, "asda.tsa".to_string());
    }

    #[test]
    fn slugify_for_filename_string() {
        let value = "a b".to_string().slugify_for_filename();
        assert_eq!(value, "a_b".to_string());
    }

    #[test]
    fn slugify_for_filename_str() {
        let value = "a b".slugify_for_filename();
        assert_eq!(value, "a_b".to_string());
    }

    #[test]
    fn slugify_for_filename_string_with_special_chars() {
        let value = "a b<>:\"/\\|?*".to_string().slugify_for_filename();
        assert_eq!(value, "a_b______|__".to_string());
    }

    #[test]
    fn slugify_for_filename_str_with_special_chars() {
        let value = "a b<>:\"/\\|?*".slugify_for_filename();
        assert_eq!(value, "a_b______|__".to_string());
    }
}

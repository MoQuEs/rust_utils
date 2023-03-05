pub trait Normalize {
    fn normalize(&self) -> String;
}

impl<T: AsRef<str>> Normalize for T {
    fn normalize(&self) -> String {
        self.slugify("_")
    }
}

pub trait Slugify {
    fn slugify(&self, separator: impl AsRef<str>) -> String;
}

impl<T: AsRef<str>> Slugify for T {
    fn slugify(&self, separator: impl AsRef<str>) -> String {
        slugify::slugify(self.as_ref(), "", separator.as_ref(), None)
    }
}

pub trait Escape {
    fn escape(&self) -> String;
}

impl<T: AsRef<str>> Escape for T {
    fn escape(&self) -> String {
        self.as_ref().replace('\'', "\\\'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_normalize() {
        let value = "a b";
        assert_eq!(value.normalize(), String::from("a_b"));
    }

    #[test]
    fn call_slugify() {
        let value = "a b";
        assert_eq!(value.slugify("."), String::from("a.b"));
    }

    #[test]
    fn call_escape() {
        let value = "a'b";
        assert_eq!(value.escape(), String::from("a\\\'b"));
    }
}

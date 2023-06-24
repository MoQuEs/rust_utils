use serde::{Deserialize, Deserializer};

pub fn option_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>
{
    Deserialize::deserialize(deserializer)
        .map(|x: T| {
            Some(x)
        })
}


#[cfg(test)]
mod tests {
    use serde::Serialize;
    use super::*;

    #[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
    pub struct Test {
        pub a: String,

        #[serde(default)]
        #[serde(deserialize_with = "option_option")]
        pub b: Option<Option<String>>,

        #[serde(default)]
        #[serde(deserialize_with = "option_option")]
        pub c: Option<Option<usize>>,
    }

    #[test]
    fn call_option_option_none() {
        let test = serde_json::from_str::<Test>(r#"{"a": "a"}"#).unwrap();
        assert_eq!(test, Test { a: String::from("a"), b: None, c: None });
    }

    #[test]
    fn call_option_option_1() {
        let test = serde_json::from_str::<Test>(r#"{"a": "a", "b": null, "c": null}"#).unwrap();
        assert_eq!(test, Test { a: String::from("a"), b: Some(None), c: Some(None) });
    }

    #[test]
    fn call_option_option_2() {
        let test = serde_json::from_str::<Test>(r#"{"a": "a", "b": "b", "c": 1}"#).unwrap();
        assert_eq!(test, Test { a: String::from("a"), b: Some(Some(String::from("b"))), c: Some(Some(1)) });
    }
}

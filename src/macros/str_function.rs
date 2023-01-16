#[macro_export]
macro_rules! str_function {
    ($name:ident) => {
        #[allow(non_snake_case, missing_docs)]
        pub fn $name() -> &'static str {
            stringify!($name)
        }
    };
}

#[cfg(test)]
mod tests {
    str_function!(Foo);

    #[test]
    fn call_function() {
        assert_eq!(self::Foo(), "Foo");
    }
}

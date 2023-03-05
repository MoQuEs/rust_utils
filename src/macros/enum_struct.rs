#[macro_export]
macro_rules! enum_struct {
    ($name:ident { $($variant:ident ),* }) => {
        #[derive(Debug)]
        pub enum $name {
            $( $variant, )*
        }

        impl $name {
            pub fn to_string(&self) -> &'static str {
                match self {
                    $( Self::$variant => stringify!($name.$variant), )*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    enum_struct!(Foo { Bar, Bas });

    #[test]
    fn create() {
        assert_eq!(Foo::Bar.to_string(), "Foo.Bar");
        assert_eq!(Foo::Bas.to_string(), "Foo.Bas");
    }
}

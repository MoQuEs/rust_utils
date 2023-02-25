#![allow(clippy::from_over_into)]

#[macro_export]
macro_rules! transparent_struct {
    (
        $(#[$derive:meta])*
        $name:ident($variant:ty)
    ) => {
        $(#[$derive])*
        pub struct $name($variant);

        impl std::ops::Deref for $name {
            type Target = $variant;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$variant> for $name {
            fn from(other: $variant) -> Self {
                Self(other)
            }
        }

        impl Into<$variant> for $name {
            fn into(self) -> $variant {
                self.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    transparent_struct!(FooString(String));
    transparent_struct!(FooVec(Vec<String>));
    transparent_struct!(#[derive(Clone, Debug, PartialEq)] FooDerive(String));

    #[test]
    fn create_single() {
        let foo_1 = FooString(String::from("test"));
        assert_eq!(foo_1.deref(), &String::from("test"));
    }

    #[test]
    fn create_from_single() {
        let foo_1 = FooString::from(String::from("test"));
        assert_eq!(foo_1.deref(), &String::from("test"));
    }

    #[test]
    fn into_single() {
        let foo_1: String = FooString::from(String::from("test")).into();
        assert_eq!(foo_1.deref(), String::from("test"));
    }

    #[test]
    fn create_vec() {
        let foo_1 = FooVec(vec![String::from("test")]);
        assert_eq!(foo_1.deref(), &vec![String::from("test")]);
    }

    #[test]
    fn create_from_vec() {
        let foo_1 = FooVec::from(vec![String::from("test")]);
        assert_eq!(foo_1.deref(), &vec![String::from("test")]);
    }

    #[test]
    fn into_vec() {
        let foo_1: Vec<String> = FooVec::from(vec![String::from("test")]).into();
        assert_eq!(foo_1.deref(), vec![String::from("test")]);
    }

    #[test]
    fn derive() {
        let foo_1 = FooDerive(String::from("test"));
        assert_eq!(foo_1.clone(), foo_1); // Clone, Debug, PartialEq
    }
}

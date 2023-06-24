use std::cmp::min;


macro_rules! diff_type {
    ($name:ident, $type:ty) => {
        pub fn $name<
            'a,
            T1: AsRef<$type> + 'a + PartialEq,
            T2: AsRef<$type> + 'a + PartialEq,
            I1: AsRef<[T1]>,
            I2: AsRef<[T2]>,
        >(
            v1: &'a I1,
            v2: &'a I2,
        ) -> Vec<&'a $type> {
            let mut diff = Vec::with_capacity(min(v1.as_ref().len(), v2.as_ref().len()));

            for ele1 in v1.as_ref() {
                let mut contains = false;

                for ele2 in v2.as_ref() {
                    if ele2.as_ref().eq(ele1.as_ref()) {
                        contains = true;
                        break;
                    }
                }

                if !contains {
                    diff.push(ele1.as_ref());
                }
            }

            for ele2 in v2.as_ref() {
                let mut contains = false;

                for ele1 in v1.as_ref() {
                    if ele1.as_ref().eq(ele2.as_ref()) {
                        contains = true;
                        break;
                    }
                }

                if !contains {
                    diff.push(ele2.as_ref());
                }
            }

            diff
        }
    }
}

diff_type!(diff_str, str);
diff_type!(diff_u8, u8);



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        let v1 = vec!["1", "2"];
        let v2 = vec!["1", "2"];

        assert_eq!(Vec::<&str>::new(), diff_str(&v1, &v2));
    }

    #[test]
    fn v1() {
        let v1 = vec!["1", "2", "3"];
        let v2 = vec!["1", "2"];

        assert_eq!(vec!["3"], diff_str(&v1, &v2));
    }

    #[test]
    fn v2() {
        let v1 = vec!["1", "2"];
        let v2 = vec!["1", "2", "3"];

        assert_eq!(vec!["3"], diff_str(&v1, &v2));
    }

    #[test]
    fn same_different_types() {
        let v1 = vec![String::from("1"), String::from("2")];
        let v2 = vec!["1", "2"];

        assert_eq!(Vec::<&str>::new(), diff_str(&v1, &v2));
    }

    #[test]
    fn v1_different_types() {
        let v1 = vec![String::from("1"), String::from("2"), String::from("3")];
        let v2 = vec!["1", "2"];

        assert_eq!(vec!["3"], diff_str(&v1, &v2));
    }

    #[test]
    fn v2_different_types() {
        let v1 = vec!["1", "2"];
        let v2 = vec![String::from("1"), String::from("2"), String::from("3")];

        assert_eq!(vec!["3"], diff_str(&v1, &v2));
    }
}

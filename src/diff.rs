use std::cmp::min;

pub fn diff<
    'a,
    T1: AsRef<str> + 'a + PartialEq,
    T2: AsRef<str> + 'a + PartialEq,
    I1: AsRef<[T1]>,
    I2: AsRef<[T2]>,
>(
    v1: &'a I1,
    v2: &'a I2,
) -> Vec<&'a str> {
    let mut diff = Vec::with_capacity(min(v1.as_ref().len(), v2.as_ref().len()));

    for ele1 in v1.as_ref() {
        for ele2 in v2.as_ref() {
            if ele1.as_ref() != ele2.as_ref() {
                diff.push(ele1.as_ref());
            }
        }
    }

    for ele2 in v2.as_ref() {
        for ele1 in v1.as_ref() {
            if ele1.as_ref() != ele2.as_ref() {
                diff.push(ele1.as_ref());
            }
        }
    }

    diff
}

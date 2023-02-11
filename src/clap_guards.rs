use num::FromPrimitive;

pub fn positive_number<T: Ord + std::str::FromStr + FromPrimitive>(
    s: &str,
) -> Result<T, String> {
    let number: T = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;

    if number > FromPrimitive::from_usize(0).unwrap() {
        Ok(number)
    } else {
        Err("number is not positive (>0)".to_string())
    }
}
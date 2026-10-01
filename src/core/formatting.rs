//! String and numeric formatting utilities.

/// Formats a double precision floating point number with specified decimal precision.
pub fn double_to_string(value: f64, decimal_places: usize) -> String {
    format!("{:.*}", decimal_places, value)
}

/// Formats an integer to string.
pub fn int_to_string(value: i32) -> String {
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting() {
        assert_eq!(double_to_string(3.14159, 2), "3.14");
        assert_eq!(double_to_string(3.14159, 4), "3.1416");
        assert_eq!(int_to_string(42), "42");
    }
}

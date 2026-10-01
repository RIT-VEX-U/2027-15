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
        assert_eq!(double_to_string(1.23456, 2), "1.23");
        assert_eq!(double_to_string(1.23456, 4), "1.2346");
        assert_eq!(int_to_string(42), "42");
    }
}

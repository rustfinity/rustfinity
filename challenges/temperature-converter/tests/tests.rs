#[cfg(test)]
mod tests {
    use temperature_converter::*;

    #[test]
    fn test_convert_celsius_to_fahrenheit() {
        assert_eq!(convert_temperature(100.0, "C", "F"), Ok(212.0));
    }

    #[test]
    fn test_convert_fahrenheit_to_celsius() {
        assert_eq!(convert_temperature(32.0, "F", "C"), Ok(0.0));
    }

    #[test]
    fn test_convert_celsius_to_kelvin() {
        assert_eq!(convert_temperature(0.0, "C", "K"), Ok(273.15));
    }

    #[test]
    fn test_convert_kelvin_to_celsius() {
        let result = convert_temperature(300.0, "K", "C").unwrap();
        let rounded_result = (result * 100.0).round() / 100.0;
        assert_eq!(rounded_result, 26.85);
    }

    #[test]
    fn test_invalid_unit() {
        assert_eq!(
            convert_temperature(100.0, "C", "X"),
            Err("Invalid unit".to_string())
        );
    }

    #[test]
    fn test_convert_fahrenheit_to_kelvin() {
        assert_eq!(convert_temperature(32.0, "F", "K"), Ok(273.15));
    }

    #[test]
    fn test_convert_kelvin_to_fahrenheit() {
        assert_eq!(convert_temperature(273.15, "K", "F"), Ok(32.0));
    }

    #[test]
    fn test_convert_celsius_to_celsius() {
        assert_eq!(convert_temperature(37.5, "C", "C"), Ok(37.5));
    }
}

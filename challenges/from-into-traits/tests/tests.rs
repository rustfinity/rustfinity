use from_into_traits::*;

// ============================================
// Temperature Conversion Tests
// ============================================

mod celsius_to_fahrenheit {
    use super::*;

    #[test]
    fn freezing_point() {
        let c = Celsius(0.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 32.0).abs() < 0.001);
    }

    #[test]
    fn boiling_point() {
        let c = Celsius(100.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 212.0).abs() < 0.001);
    }

    #[test]
    fn body_temperature() {
        let c = Celsius(37.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 98.6).abs() < 0.1);
    }

    #[test]
    fn negative_temperature() {
        let c = Celsius(-40.0);
        let f: Fahrenheit = c.into();
        // -40 is the same in both scales!
        assert!((f.0 - (-40.0)).abs() < 0.001);
    }

    #[test]
    fn using_from_syntax() {
        let c = Celsius(25.0);
        let f = Fahrenheit::from(c);
        assert!((f.0 - 77.0).abs() < 0.001);
    }
}

mod fahrenheit_to_celsius {
    use super::*;

    #[test]
    fn freezing_point() {
        let f = Fahrenheit(32.0);
        let c: Celsius = f.into();
        assert!((c.0 - 0.0).abs() < 0.001);
    }

    #[test]
    fn boiling_point() {
        let f = Fahrenheit(212.0);
        let c: Celsius = f.into();
        assert!((c.0 - 100.0).abs() < 0.001);
    }

    #[test]
    fn room_temperature() {
        let f = Fahrenheit(68.0);
        let c: Celsius = f.into();
        assert!((c.0 - 20.0).abs() < 0.001);
    }

    #[test]
    fn negative_temperature() {
        let f = Fahrenheit(-40.0);
        let c: Celsius = f.into();
        assert!((c.0 - (-40.0)).abs() < 0.001);
    }

    #[test]
    fn using_from_syntax() {
        let f = Fahrenheit(77.0);
        let c = Celsius::from(f);
        assert!((c.0 - 25.0).abs() < 0.001);
    }
}

mod temperature_round_trip {
    use super::*;

    #[test]
    fn celsius_round_trip() {
        let original = Celsius(42.5);
        let f: Fahrenheit = original.into();
        let back: Celsius = f.into();
        assert!((back.0 - original.0).abs() < 0.001);
    }

    #[test]
    fn fahrenheit_round_trip() {
        let original = Fahrenheit(100.0);
        let c: Celsius = original.into();
        let back: Fahrenheit = c.into();
        assert!((back.0 - original.0).abs() < 0.001);
    }
}

// ============================================
// RGB to HexColor Tests
// ============================================

mod rgb_to_hex {
    use super::*;

    #[test]
    fn white() {
        let rgb = Rgb { r: 255, g: 255, b: 255 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#FFFFFF");
    }

    #[test]
    fn black() {
        let rgb = Rgb { r: 0, g: 0, b: 0 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#000000");
    }

    #[test]
    fn red() {
        let rgb = Rgb { r: 255, g: 0, b: 0 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#FF0000");
    }

    #[test]
    fn green() {
        let rgb = Rgb { r: 0, g: 255, b: 0 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#00FF00");
    }

    #[test]
    fn blue() {
        let rgb = Rgb { r: 0, g: 0, b: 255 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#0000FF");
    }

    #[test]
    fn coral() {
        let rgb = Rgb { r: 255, g: 87, b: 51 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#FF5733");
    }

    #[test]
    fn leading_zeros() {
        let rgb = Rgb { r: 1, g: 2, b: 3 };
        let hex: HexColor = rgb.into();
        assert_eq!(hex.0, "#010203");
    }

    #[test]
    fn using_from_syntax() {
        let rgb = Rgb { r: 128, g: 128, b: 128 };
        let hex = HexColor::from(rgb);
        assert_eq!(hex.0, "#808080");
    }
}

// ============================================
// Email Tests
// ============================================

mod email_from_str {
    use super::*;

    #[test]
    fn from_str_slice() {
        let email: Email = "user@example.com".into();
        assert_eq!(email.0, "user@example.com");
    }

    #[test]
    fn from_string() {
        let s = String::from("admin@test.org");
        let email: Email = s.into();
        assert_eq!(email.0, "admin@test.org");
    }

    #[test]
    fn using_from_syntax_str() {
        let email = Email::from("hello@world.net");
        assert_eq!(email.0, "hello@world.net");
    }

    #[test]
    fn using_from_syntax_string() {
        let s = String::from("test@domain.com");
        let email = Email::from(s);
        assert_eq!(email.0, "test@domain.com");
    }

    #[test]
    fn empty_string() {
        let email: Email = "".into();
        assert_eq!(email.0, "");
    }

    #[test]
    fn unicode_email() {
        let email: Email = "user@例え.jp".into();
        assert_eq!(email.0, "user@例え.jp");
    }
}

// ============================================
// Point2D to Point3D Tests
// ============================================

mod point_conversion {
    use super::*;

    #[test]
    fn origin() {
        let p2d = Point2D { x: 0.0, y: 0.0 };
        let p3d: Point3D = p2d.into();
        assert_eq!(p3d.x, 0.0);
        assert_eq!(p3d.y, 0.0);
        assert_eq!(p3d.z, 0.0);
    }

    #[test]
    fn positive_coordinates() {
        let p2d = Point2D { x: 1.0, y: 2.0 };
        let p3d: Point3D = p2d.into();
        assert_eq!(p3d.x, 1.0);
        assert_eq!(p3d.y, 2.0);
        assert_eq!(p3d.z, 0.0);
    }

    #[test]
    fn negative_coordinates() {
        let p2d = Point2D { x: -5.5, y: -10.25 };
        let p3d: Point3D = p2d.into();
        assert_eq!(p3d.x, -5.5);
        assert_eq!(p3d.y, -10.25);
        assert_eq!(p3d.z, 0.0);
    }

    #[test]
    fn using_from_syntax() {
        let p2d = Point2D { x: 3.14, y: 2.71 };
        let p3d = Point3D::from(p2d);
        assert!((p3d.x - 3.14).abs() < 0.001);
        assert!((p3d.y - 2.71).abs() < 0.001);
        assert_eq!(p3d.z, 0.0);
    }

    #[test]
    fn large_values() {
        let p2d = Point2D { x: 1e10, y: -1e10 };
        let p3d: Point3D = p2d.into();
        assert_eq!(p3d.x, 1e10);
        assert_eq!(p3d.y, -1e10);
        assert_eq!(p3d.z, 0.0);
    }
}

// ============================================
// Generic Wrapper Tests
// ============================================

mod wrapper {
    use super::*;

    #[test]
    fn wrap_integer() {
        let wrapped: Wrapper<i32> = 42.into();
        assert_eq!(wrapped.into_inner(), 42);
    }

    #[test]
    fn wrap_string() {
        let wrapped: Wrapper<String> = String::from("hello").into();
        assert_eq!(wrapped.into_inner(), "hello");
    }

    #[test]
    fn wrap_float() {
        let wrapped: Wrapper<f64> = 3.14.into();
        assert!((wrapped.into_inner() - 3.14).abs() < 0.001);
    }

    #[test]
    fn wrap_bool() {
        let wrapped: Wrapper<bool> = true.into();
        assert!(wrapped.into_inner());
    }

    #[test]
    fn wrap_vec() {
        let wrapped: Wrapper<Vec<i32>> = vec![1, 2, 3].into();
        assert_eq!(wrapped.into_inner(), vec![1, 2, 3]);
    }

    #[test]
    fn wrap_tuple() {
        let wrapped: Wrapper<(i32, &str)> = (1, "hello").into();
        assert_eq!(wrapped.into_inner(), (1, "hello"));
    }

    #[test]
    fn using_from_syntax() {
        let wrapped = Wrapper::from(100u8);
        assert_eq!(wrapped.into_inner(), 100u8);
    }

    #[test]
    fn wrap_custom_struct() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 1, y: 2 };
        let wrapped: Wrapper<Point> = p.into();
        assert_eq!(wrapped.into_inner(), Point { x: 1, y: 2 });
    }

    #[test]
    fn wrap_option() {
        let wrapped: Wrapper<Option<i32>> = Some(42).into();
        assert_eq!(wrapped.into_inner(), Some(42));
    }

    #[test]
    fn wrap_result() {
        let wrapped: Wrapper<Result<i32, &str>> = Ok(42).into();
        assert_eq!(wrapped.into_inner(), Ok(42));
    }
}

// ============================================
// Integration Tests
// ============================================

mod integration {
    use super::*;

    #[test]
    fn temperature_chain() {
        // Start with Celsius, convert through Fahrenheit, back to Celsius
        let temps: Vec<Celsius> = vec![Celsius(0.0), Celsius(25.0), Celsius(100.0)];
        let converted: Vec<Celsius> = temps
            .into_iter()
            .map(|c| -> Fahrenheit { c.into() })
            .map(|f| -> Celsius { f.into() })
            .collect();

        assert!((converted[0].0 - 0.0).abs() < 0.001);
        assert!((converted[1].0 - 25.0).abs() < 0.001);
        assert!((converted[2].0 - 100.0).abs() < 0.001);
    }

    #[test]
    fn rgb_collection_to_hex() {
        let colors = vec![
            Rgb { r: 255, g: 0, b: 0 },
            Rgb { r: 0, g: 255, b: 0 },
            Rgb { r: 0, g: 0, b: 255 },
        ];

        let hex_colors: Vec<HexColor> = colors.into_iter().map(|rgb| rgb.into()).collect();

        assert_eq!(hex_colors[0].0, "#FF0000");
        assert_eq!(hex_colors[1].0, "#00FF00");
        assert_eq!(hex_colors[2].0, "#0000FF");
    }

    #[test]
    fn points_to_3d() {
        let points_2d = vec![
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 2.0, y: 2.0 },
        ];

        let points_3d: Vec<Point3D> = points_2d.into_iter().map(|p| p.into()).collect();

        for p in &points_3d {
            assert_eq!(p.z, 0.0);
        }
        assert_eq!(points_3d[0].x, 0.0);
        assert_eq!(points_3d[2].x, 2.0);
    }

    #[test]
    fn generic_function_with_into() {
        fn process_wrapped<T, U>(value: T) -> U
        where
            T: Into<Wrapper<U>>,
            Wrapper<U>: Into<Wrapper<U>>,
        {
            let wrapped: Wrapper<U> = value.into();
            wrapped.into_inner()
        }

        let result: i32 = process_wrapped(42);
        assert_eq!(result, 42);
    }

    #[test]
    fn email_from_various_sources() {
        let sources: Vec<Email> = vec![
            "static@example.com".into(),
            String::from("dynamic@example.com").into(),
        ];

        assert_eq!(sources[0].0, "static@example.com");
        assert_eq!(sources[1].0, "dynamic@example.com");
    }

    #[test]
    fn nested_wrapper() {
        let inner: Wrapper<i32> = 42.into();
        let outer: Wrapper<Wrapper<i32>> = inner.into();
        assert_eq!(outer.into_inner().into_inner(), 42);
    }
}

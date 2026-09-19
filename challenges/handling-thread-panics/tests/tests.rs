use handling_thread_panics::*;

mod describe_panic_tests {
    use super::*;

    #[test]
    fn clean_job_reports_nothing() {
        assert_eq!(describe_panic(Box::new(|| {})), None);
    }

    #[test]
    fn job_that_does_work_reports_nothing() {
        assert_eq!(
            describe_panic(Box::new(|| {
                let _total: u64 = (0..1000).sum();
            })),
            None
        );
    }

    #[test]
    fn captures_a_literal_panic_message() {
        assert_eq!(
            describe_panic(Box::new(|| panic!("boom"))),
            Some("boom".to_string())
        );
    }

    #[test]
    fn captures_a_formatted_panic_message() {
        let code = 42;
        assert_eq!(
            describe_panic(Box::new(move || panic!("failed with {code}"))),
            Some("failed with 42".to_string())
        );
    }

    #[test]
    fn captures_an_unwrap_failure() {
        let message = describe_panic(Box::new(|| {
            let none: Option<i32> = None;
            none.unwrap();
        }));

        assert!(message.is_some(), "expected the panic to be reported");
    }

    #[test]
    fn captures_an_index_out_of_bounds() {
        let message = describe_panic(Box::new(|| {
            let v = vec![1, 2, 3];
            let _ = v[10];
        }));

        assert!(message.is_some(), "expected the panic to be reported");
    }

    #[test]
    fn a_panicking_job_does_not_kill_the_caller() {
        let _ = describe_panic(Box::new(|| panic!("boom")));

        // Still here, so the caller survived the worker's panic.
        assert_eq!(describe_panic(Box::new(|| {})), None);
    }
}

mod divide_in_thread_tests {
    use super::*;

    #[test]
    fn divides_evenly() {
        assert_eq!(divide_in_thread(10, 2), Ok(5));
    }

    #[test]
    fn truncates_toward_zero() {
        assert_eq!(divide_in_thread(7, 2), Ok(3));
        assert_eq!(divide_in_thread(-7, 2), Ok(-3));
    }

    #[test]
    fn dividing_by_zero_is_an_error() {
        assert_eq!(divide_in_thread(1, 0), Err("division by zero".to_string()));
    }

    #[test]
    fn zero_numerator_is_fine() {
        assert_eq!(divide_in_thread(0, 5), Ok(0));
    }

    #[test]
    fn zero_over_zero_is_still_an_error() {
        assert_eq!(divide_in_thread(0, 0), Err("division by zero".to_string()));
    }

    #[test]
    fn negative_divisor() {
        assert_eq!(divide_in_thread(9, -3), Ok(-3));
    }

    #[test]
    fn the_caller_keeps_running_after_an_error() {
        assert!(divide_in_thread(1, 0).is_err());
        assert_eq!(divide_in_thread(8, 4), Ok(2));
    }
}

use default_trait_patterns::*;

// ============================================================================
// AppConfig tests
// ============================================================================

#[test]
fn appconfig_default_theme() {
    let config = AppConfig::default();
    assert_eq!(config.theme, "light");
}

#[test]
fn appconfig_default_dark_mode() {
    let config = AppConfig::default();
    assert!(!config.dark_mode);
}

#[test]
fn appconfig_default_font_size() {
    let config = AppConfig::default();
    assert_eq!(config.font_size, 14);
}

#[test]
fn appconfig_default_max_connections() {
    let config = AppConfig::default();
    assert_eq!(config.max_connections, 100);
}

#[test]
fn appconfig_struct_update_syntax() {
    let config = AppConfig {
        dark_mode: true,
        ..Default::default()
    };
    assert!(config.dark_mode);
    assert_eq!(config.theme, "light");
    assert_eq!(config.font_size, 14);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn appconfig_partial_update() {
    let config = AppConfig {
        theme: String::from("monokai"),
        font_size: 18,
        ..Default::default()
    };
    assert_eq!(config.theme, "monokai");
    assert_eq!(config.font_size, 18);
    assert!(!config.dark_mode);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn appconfig_all_custom() {
    let config = AppConfig {
        theme: String::from("solarized"),
        dark_mode: true,
        font_size: 20,
        max_connections: 500,
    };
    assert_eq!(config.theme, "solarized");
    assert!(config.dark_mode);
    assert_eq!(config.font_size, 20);
    assert_eq!(config.max_connections, 500);
}

#[test]
fn appconfig_clone() {
    let config1 = AppConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1, config2);
}

// ============================================================================
// Counter tests
// ============================================================================

#[test]
fn counter_default() {
    let counter = Counter::default();
    assert_eq!(counter.count, 0);
}

#[test]
fn counter_new() {
    let counter = Counter::new(10);
    assert_eq!(counter.count, 10);
}

#[test]
fn counter_increment() {
    let mut counter = Counter::default();
    counter.increment();
    assert_eq!(counter.count, 1);
}

#[test]
fn counter_decrement() {
    let mut counter = Counter::default();
    counter.decrement();
    assert_eq!(counter.count, -1);
}

#[test]
fn counter_multiple_operations() {
    let mut counter = Counter::default();
    counter.increment();
    counter.increment();
    counter.decrement();
    assert_eq!(counter.count, 1);
}

#[test]
fn counter_new_negative() {
    let counter = Counter::new(-5);
    assert_eq!(counter.count, -5);
}

#[test]
fn counter_clone() {
    let counter1 = Counter::new(42);
    let counter2 = counter1.clone();
    assert_eq!(counter1, counter2);
}

// ============================================================================
// BoundedValue tests
// ============================================================================

#[test]
fn bounded_value_default_i32() {
    let bounded: BoundedValue<i32> = BoundedValue::default();
    assert_eq!(bounded.value, 0);
    assert_eq!(bounded.min, 0);
    assert_eq!(bounded.max, 0);
}

#[test]
fn bounded_value_default_f64() {
    let bounded: BoundedValue<f64> = BoundedValue::default();
    assert_eq!(bounded.value, 0.0);
    assert_eq!(bounded.min, 0.0);
    assert_eq!(bounded.max, 0.0);
}

#[test]
fn bounded_value_default_string() {
    let bounded: BoundedValue<String> = BoundedValue::default();
    assert_eq!(bounded.value, "");
    assert_eq!(bounded.min, "");
    assert_eq!(bounded.max, "");
}

#[test]
fn bounded_value_default_bool() {
    let bounded: BoundedValue<bool> = BoundedValue::default();
    assert!(!bounded.value);
    assert!(!bounded.min);
    assert!(!bounded.max);
}

#[test]
fn bounded_value_new() {
    let bounded = BoundedValue::new(50, 0, 100);
    assert_eq!(bounded.value, 50);
    assert_eq!(bounded.min, 0);
    assert_eq!(bounded.max, 100);
}

#[test]
fn bounded_value_clamp_within() {
    let mut bounded = BoundedValue::new(50, 0, 100);
    bounded.clamp();
    assert_eq!(bounded.value, 50);
}

#[test]
fn bounded_value_clamp_below_min() {
    let mut bounded = BoundedValue::new(-10, 0, 100);
    bounded.clamp();
    assert_eq!(bounded.value, 0);
}

#[test]
fn bounded_value_clamp_above_max() {
    let mut bounded = BoundedValue::new(200, 0, 100);
    bounded.clamp();
    assert_eq!(bounded.value, 100);
}

#[test]
fn bounded_value_clamp_at_min() {
    let mut bounded = BoundedValue::new(0, 0, 100);
    bounded.clamp();
    assert_eq!(bounded.value, 0);
}

#[test]
fn bounded_value_clamp_at_max() {
    let mut bounded = BoundedValue::new(100, 0, 100);
    bounded.clamp();
    assert_eq!(bounded.value, 100);
}

#[test]
fn bounded_value_clone() {
    let bounded1 = BoundedValue::new(50, 0, 100);
    let bounded2 = bounded1.clone();
    assert_eq!(bounded1, bounded2);
}

// ============================================================================
// Status tests
// ============================================================================

#[test]
fn status_default() {
    let status = Status::default();
    assert!(matches!(status, Status::Pending));
}

#[test]
fn status_pending() {
    let status = Status::Pending;
    assert!(matches!(status, Status::Pending));
}

#[test]
fn status_active() {
    let status = Status::Active;
    assert!(matches!(status, Status::Active));
}

#[test]
fn status_completed() {
    let status = Status::Completed;
    assert!(matches!(status, Status::Completed));
}

#[test]
fn status_failed() {
    let status = Status::Failed;
    assert!(matches!(status, Status::Failed));
}

#[test]
fn status_equality() {
    assert_eq!(Status::Pending, Status::Pending);
    assert_ne!(Status::Pending, Status::Active);
    assert_ne!(Status::Active, Status::Completed);
    assert_ne!(Status::Completed, Status::Failed);
}

#[test]
fn status_clone() {
    let status1 = Status::Active;
    let status2 = status1.clone();
    assert_eq!(status1, status2);
}

// ============================================================================
// create_with_defaults tests
// ============================================================================

#[test]
fn create_with_defaults_i32() {
    let n: i32 = create_with_defaults();
    assert_eq!(n, 0);
}

#[test]
fn create_with_defaults_u64() {
    let n: u64 = create_with_defaults();
    assert_eq!(n, 0);
}

#[test]
fn create_with_defaults_f64() {
    let n: f64 = create_with_defaults();
    assert_eq!(n, 0.0);
}

#[test]
fn create_with_defaults_bool() {
    let b: bool = create_with_defaults();
    assert!(!b);
}

#[test]
fn create_with_defaults_string() {
    let s: String = create_with_defaults();
    assert_eq!(s, "");
}

#[test]
fn create_with_defaults_vec() {
    let v: Vec<i32> = create_with_defaults();
    assert!(v.is_empty());
}

#[test]
fn create_with_defaults_option() {
    let o: Option<i32> = create_with_defaults();
    assert!(o.is_none());
}

#[test]
fn create_with_defaults_counter() {
    let c: Counter = create_with_defaults();
    assert_eq!(c.count, 0);
}

#[test]
fn create_with_defaults_appconfig() {
    let config: AppConfig = create_with_defaults();
    assert_eq!(config.theme, "light");
    assert_eq!(config.font_size, 14);
}

#[test]
fn create_with_defaults_status() {
    let status: Status = create_with_defaults();
    assert!(matches!(status, Status::Pending));
}

// ============================================================================
// merge_with_defaults tests
// ============================================================================

#[test]
fn merge_all_none() {
    let partial = PartialConfig::default();
    let config = merge_with_defaults(partial);
    assert_eq!(config, AppConfig::default());
}

#[test]
fn merge_theme_only() {
    let partial = PartialConfig {
        theme: Some("dark".to_string()),
        ..Default::default()
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "dark");
    assert!(!config.dark_mode);
    assert_eq!(config.font_size, 14);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn merge_dark_mode_only() {
    let partial = PartialConfig {
        dark_mode: Some(true),
        ..Default::default()
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "light");
    assert!(config.dark_mode);
    assert_eq!(config.font_size, 14);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn merge_font_size_only() {
    let partial = PartialConfig {
        font_size: Some(20),
        ..Default::default()
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "light");
    assert!(!config.dark_mode);
    assert_eq!(config.font_size, 20);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn merge_max_connections_only() {
    let partial = PartialConfig {
        max_connections: Some(500),
        ..Default::default()
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "light");
    assert!(!config.dark_mode);
    assert_eq!(config.font_size, 14);
    assert_eq!(config.max_connections, 500);
}

#[test]
fn merge_multiple_fields() {
    let partial = PartialConfig {
        theme: Some("monokai".to_string()),
        dark_mode: Some(true),
        font_size: None,
        max_connections: Some(200),
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "monokai");
    assert!(config.dark_mode);
    assert_eq!(config.font_size, 14);
    assert_eq!(config.max_connections, 200);
}

#[test]
fn merge_all_some() {
    let partial = PartialConfig {
        theme: Some("solarized".to_string()),
        dark_mode: Some(true),
        font_size: Some(18),
        max_connections: Some(1000),
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "solarized");
    assert!(config.dark_mode);
    assert_eq!(config.font_size, 18);
    assert_eq!(config.max_connections, 1000);
}

#[test]
fn merge_empty_theme() {
    let partial = PartialConfig {
        theme: Some(String::new()),
        ..Default::default()
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "");
}

#[test]
fn merge_zero_values() {
    let partial = PartialConfig {
        font_size: Some(0),
        max_connections: Some(0),
        ..Default::default()
    };
    let config = merge_with_defaults(partial);
    assert_eq!(config.font_size, 0);
    assert_eq!(config.max_connections, 0);
}

// ============================================================================
// default_vec tests
// ============================================================================

#[test]
fn default_vec_empty() {
    let v: Vec<i32> = default_vec(0);
    assert!(v.is_empty());
}

#[test]
fn default_vec_single() {
    let v: Vec<i32> = default_vec(1);
    assert_eq!(v, vec![0]);
}

#[test]
fn default_vec_multiple_i32() {
    let v: Vec<i32> = default_vec(5);
    assert_eq!(v, vec![0, 0, 0, 0, 0]);
}

#[test]
fn default_vec_bool() {
    let v: Vec<bool> = default_vec(3);
    assert_eq!(v, vec![false, false, false]);
}

#[test]
fn default_vec_string() {
    let v: Vec<String> = default_vec(4);
    assert_eq!(v, vec!["", "", "", ""]);
}

#[test]
fn default_vec_option() {
    let v: Vec<Option<i32>> = default_vec(3);
    assert_eq!(v, vec![None, None, None]);
}

#[test]
fn default_vec_counter() {
    let v: Vec<Counter> = default_vec(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].count, 0);
    assert_eq!(v[1].count, 0);
}

#[test]
fn default_vec_appconfig() {
    let v: Vec<AppConfig> = default_vec(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].theme, "light");
    assert_eq!(v[1].font_size, 14);
}

#[test]
fn default_vec_f64() {
    let v: Vec<f64> = default_vec(3);
    assert_eq!(v, vec![0.0, 0.0, 0.0]);
}

#[test]
fn default_vec_nested_vec() {
    let v: Vec<Vec<i32>> = default_vec(3);
    assert_eq!(v.len(), 3);
    for inner in &v {
        assert!(inner.is_empty());
    }
}

#[test]
fn default_vec_large() {
    let v: Vec<u8> = default_vec(1000);
    assert_eq!(v.len(), 1000);
    assert!(v.iter().all(|&x| x == 0));
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn integration_config_workflow() {
    // Start with defaults
    let config = AppConfig::default();
    assert_eq!(config.theme, "light");

    // Override some values
    let updated = AppConfig {
        theme: String::from("dark"),
        dark_mode: true,
        ..config
    };
    assert_eq!(updated.theme, "dark");
    assert!(updated.dark_mode);
    assert_eq!(updated.font_size, 14);
}

#[test]
fn integration_partial_config_builder() {
    // Build partial config incrementally
    let mut partial = PartialConfig::default();
    partial.theme = Some("gruvbox".to_string());
    partial.font_size = Some(16);

    let config = merge_with_defaults(partial);
    assert_eq!(config.theme, "gruvbox");
    assert_eq!(config.font_size, 16);
    assert!(!config.dark_mode);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn integration_counter_reset() {
    let mut counter = Counter::new(100);
    counter.increment();
    assert_eq!(counter.count, 101);

    // Reset using default
    counter = Counter::default();
    assert_eq!(counter.count, 0);
}

#[test]
fn integration_bounded_value_workflow() {
    // Start with defaults
    let mut bounded: BoundedValue<i32> = BoundedValue::default();
    assert_eq!(bounded.value, 0);

    // Configure bounds
    bounded.min = 0;
    bounded.max = 100;
    bounded.value = 150;
    bounded.clamp();
    assert_eq!(bounded.value, 100);
}

#[test]
fn integration_status_state_machine() {
    let mut status = Status::default();
    assert!(matches!(status, Status::Pending));

    status = Status::Active;
    assert!(matches!(status, Status::Active));

    status = Status::Completed;
    assert!(matches!(status, Status::Completed));
}

#[test]
fn integration_create_and_modify() {
    // Create default and modify
    let mut configs: Vec<AppConfig> = default_vec(3);
    configs[0].theme = String::from("dark");
    configs[1].dark_mode = true;
    configs[2].font_size = 20;

    assert_eq!(configs[0].theme, "dark");
    assert!(configs[1].dark_mode);
    assert_eq!(configs[2].font_size, 20);
}

#[test]
fn integration_generic_default_chain() {
    // Create defaults of different types in sequence
    let s: String = create_with_defaults();
    let n: i32 = create_with_defaults();
    let v: Vec<bool> = create_with_defaults();
    let c: Counter = create_with_defaults();

    assert_eq!(s, "");
    assert_eq!(n, 0);
    assert!(v.is_empty());
    assert_eq!(c.count, 0);
}

#[test]
fn integration_nested_defaults() {
    // BoundedValue containing Counter
    let bounded: BoundedValue<Counter> = BoundedValue::default();
    assert_eq!(bounded.value.count, 0);
    assert_eq!(bounded.min.count, 0);
    assert_eq!(bounded.max.count, 0);
}

#[test]
fn integration_default_vec_of_bounded() {
    let bounded_values: Vec<BoundedValue<i32>> = default_vec(3);
    assert_eq!(bounded_values.len(), 3);
    for bv in &bounded_values {
        assert_eq!(bv.value, 0);
        assert_eq!(bv.min, 0);
        assert_eq!(bv.max, 0);
    }
}

#[test]
fn integration_status_vec() {
    let statuses: Vec<Status> = default_vec(5);
    assert!(statuses.iter().all(|s| matches!(s, Status::Pending)));
}

#[test]
fn integration_partial_config_default() {
    // PartialConfig itself implements Default
    let partial: PartialConfig = create_with_defaults();
    assert!(partial.theme.is_none());
    assert!(partial.dark_mode.is_none());
    assert!(partial.font_size.is_none());
    assert!(partial.max_connections.is_none());
}

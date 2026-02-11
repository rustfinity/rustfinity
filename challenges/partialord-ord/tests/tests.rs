use partialord_ord::*;
use std::cmp::Ordering;

// ==================== Score Tests ====================

#[test]
fn score_less_than() {
    let s1 = Score(100);
    let s2 = Score(200);
    assert!(s1 < s2);
}

#[test]
fn score_greater_than() {
    let s1 = Score(200);
    let s2 = Score(100);
    assert!(s1 > s2);
}

#[test]
fn score_equal() {
    let s1 = Score(100);
    let s2 = Score(100);
    assert_eq!(s1, s2);
    assert!(!(s1 < s2));
    assert!(!(s1 > s2));
}

#[test]
fn score_cmp() {
    assert_eq!(Score(100).cmp(&Score(200)), Ordering::Less);
    assert_eq!(Score(200).cmp(&Score(100)), Ordering::Greater);
    assert_eq!(Score(100).cmp(&Score(100)), Ordering::Equal);
}

#[test]
fn score_zero() {
    assert_eq!(Score(0).cmp(&Score(0)), Ordering::Equal);
    assert!(Score(0) < Score(1));
}

#[test]
fn score_max() {
    let max = Score(u32::MAX);
    let almost_max = Score(u32::MAX - 1);
    assert!(almost_max < max);
}

// ==================== Version Tests ====================

#[test]
fn version_major_comparison() {
    let v1 = Version::new(1, 9, 9);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2);
}

#[test]
fn version_minor_comparison() {
    let v1 = Version::new(1, 0, 9);
    let v2 = Version::new(1, 1, 0);
    assert!(v1 < v2);
}

#[test]
fn version_patch_comparison() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 0, 1);
    assert!(v1 < v2);
}

#[test]
fn version_equal() {
    let v1 = Version::new(2, 3, 4);
    let v2 = Version::new(2, 3, 4);
    assert_eq!(v1, v2);
    assert_eq!(v1.cmp(&v2), Ordering::Equal);
}

#[test]
fn version_cmp() {
    assert_eq!(
        Version::new(1, 0, 0).cmp(&Version::new(2, 0, 0)),
        Ordering::Less
    );
    assert_eq!(
        Version::new(2, 0, 0).cmp(&Version::new(1, 0, 0)),
        Ordering::Greater
    );
}

#[test]
fn version_zero() {
    let v = Version::new(0, 0, 0);
    assert_eq!(v.cmp(&v), Ordering::Equal);
    assert!(v < Version::new(0, 0, 1));
}

#[test]
fn version_sorting() {
    let mut versions = vec![
        Version::new(2, 0, 0),
        Version::new(1, 0, 0),
        Version::new(1, 1, 0),
        Version::new(1, 0, 1),
    ];
    versions.sort();
    assert_eq!(
        versions,
        vec![
            Version::new(1, 0, 0),
            Version::new(1, 0, 1),
            Version::new(1, 1, 0),
            Version::new(2, 0, 0),
        ]
    );
}

#[test]
fn version_partial_cmp() {
    let v1 = Version::new(1, 2, 3);
    let v2 = Version::new(1, 2, 4);
    assert_eq!(v1.partial_cmp(&v2), Some(Ordering::Less));
}

// ==================== Temperature Tests ====================

#[test]
fn temperature_celsius_comparison() {
    let t1 = Temperature::Celsius(20.0);
    let t2 = Temperature::Celsius(25.0);
    assert!(t1 < t2);
}

#[test]
fn temperature_fahrenheit_comparison() {
    let t1 = Temperature::Fahrenheit(68.0); // 20°C
    let t2 = Temperature::Fahrenheit(77.0); // 25°C
    assert!(t1 < t2);
}

#[test]
fn temperature_cross_unit_equal() {
    let c = Temperature::Celsius(0.0);
    let f = Temperature::Fahrenheit(32.0);
    assert_eq!(c, f);
}

#[test]
fn temperature_cross_unit_boiling() {
    let c = Temperature::Celsius(100.0);
    let f = Temperature::Fahrenheit(212.0);
    assert_eq!(c, f);
}

#[test]
fn temperature_cross_unit_comparison() {
    let c = Temperature::Celsius(20.0);
    let f = Temperature::Fahrenheit(77.0); // 25°C
    assert!(c < f);
}

#[test]
fn temperature_negative() {
    let t1 = Temperature::Celsius(-10.0);
    let t2 = Temperature::Celsius(0.0);
    assert!(t1 < t2);
}

#[test]
fn temperature_fahrenheit_to_celsius() {
    let t = Temperature::Fahrenheit(32.0);
    assert!((t.to_celsius() - 0.0).abs() < 0.0001);

    let t2 = Temperature::Fahrenheit(212.0);
    assert!((t2.to_celsius() - 100.0).abs() < 0.0001);
}

#[test]
fn temperature_partial_cmp() {
    let t1 = Temperature::Celsius(20.0);
    let t2 = Temperature::Celsius(25.0);
    assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Less));
    assert_eq!(t2.partial_cmp(&t1), Some(Ordering::Greater));
}

// ==================== Priority Tests ====================

#[test]
fn priority_ordering() {
    assert!(Priority::Low < Priority::Medium);
    assert!(Priority::Medium < Priority::High);
    assert!(Priority::High < Priority::Critical);
}

#[test]
fn priority_equal() {
    assert_eq!(Priority::Low, Priority::Low);
    assert_eq!(Priority::Critical, Priority::Critical);
}

#[test]
fn priority_cmp() {
    assert_eq!(Priority::Low.cmp(&Priority::Critical), Ordering::Less);
    assert_eq!(Priority::Critical.cmp(&Priority::Low), Ordering::Greater);
    assert_eq!(Priority::High.cmp(&Priority::High), Ordering::Equal);
}

#[test]
fn priority_sorting() {
    let mut priorities = vec![
        Priority::Critical,
        Priority::Low,
        Priority::High,
        Priority::Medium,
    ];
    priorities.sort();
    assert_eq!(
        priorities,
        vec![
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Critical
        ]
    );
}

#[test]
fn priority_min_max() {
    let priorities = [
        Priority::Medium,
        Priority::Low,
        Priority::Critical,
        Priority::High,
    ];
    assert_eq!(priorities.iter().min(), Some(&Priority::Low));
    assert_eq!(priorities.iter().max(), Some(&Priority::Critical));
}

// ==================== Player Tests ====================

#[test]
fn player_higher_score_first() {
    let alice = Player::new("Alice", 100);
    let bob = Player::new("Bob", 200);
    assert!(bob < alice); // Bob has higher score, comes first (is "less" in sort order)
}

#[test]
fn player_same_score_alphabetical() {
    let alice = Player::new("Alice", 100);
    let bob = Player::new("Bob", 100);
    assert!(alice < bob); // Same score, Alice < Bob alphabetically
}

#[test]
fn player_equal() {
    let p1 = Player::new("Alice", 100);
    let p2 = Player::new("Alice", 100);
    assert_eq!(p1, p2);
}

#[test]
fn player_sorting() {
    let mut players = vec![
        Player::new("Charlie", 50),
        Player::new("Alice", 100),
        Player::new("Bob", 100),
        Player::new("Diana", 200),
    ];
    players.sort();
    assert_eq!(
        players,
        vec![
            Player::new("Diana", 200),   // Highest score first
            Player::new("Alice", 100),   // Same score, alphabetical
            Player::new("Bob", 100),     // Same score, alphabetical
            Player::new("Charlie", 50),  // Lowest score last
        ]
    );
}

#[test]
fn player_cmp() {
    let alice = Player::new("Alice", 100);
    let bob = Player::new("Bob", 200);
    assert_eq!(bob.cmp(&alice), Ordering::Less); // Bob comes first (higher score)
}

#[test]
fn player_zero_score() {
    let p1 = Player::new("Alpha", 0);
    let p2 = Player::new("Beta", 0);
    assert!(p1 < p2); // Alphabetical when scores equal
}

// ==================== find_min Tests ====================

#[test]
fn find_min_basic() {
    assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(&1));
}

#[test]
fn find_min_single() {
    assert_eq!(find_min(&[42]), Some(&42));
}

#[test]
fn find_min_empty() {
    assert_eq!(find_min::<i32>(&[]), None);
}

#[test]
fn find_min_negative() {
    assert_eq!(find_min(&[-5, -2, 0, 3]), Some(&-5));
}

#[test]
fn find_min_strings() {
    assert_eq!(find_min(&["cherry", "apple", "banana"]), Some(&"apple"));
}

#[test]
fn find_min_with_scores() {
    let scores = [Score(50), Score(25), Score(75)];
    assert_eq!(find_min(&scores), Some(&Score(25)));
}

#[test]
fn find_min_all_same() {
    assert_eq!(find_min(&[5, 5, 5, 5]), Some(&5));
}

// ==================== find_max Tests ====================

#[test]
fn find_max_basic() {
    assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(&5));
}

#[test]
fn find_max_single() {
    assert_eq!(find_max(&[42]), Some(&42));
}

#[test]
fn find_max_empty() {
    assert_eq!(find_max::<i32>(&[]), None);
}

#[test]
fn find_max_negative() {
    assert_eq!(find_max(&[-5, -2, 0, 3]), Some(&3));
}

#[test]
fn find_max_strings() {
    assert_eq!(find_max(&["cherry", "apple", "banana"]), Some(&"cherry"));
}

#[test]
fn find_max_with_scores() {
    let scores = [Score(50), Score(25), Score(75)];
    assert_eq!(find_max(&scores), Some(&Score(75)));
}

#[test]
fn find_max_all_same() {
    assert_eq!(find_max(&[5, 5, 5, 5]), Some(&5));
}

// ==================== is_sorted Tests ====================

#[test]
fn is_sorted_ascending() {
    assert!(is_sorted(&[1, 2, 3, 4, 5]));
}

#[test]
fn is_sorted_with_duplicates() {
    assert!(is_sorted(&[1, 1, 2, 2, 3]));
}

#[test]
fn is_sorted_not_sorted() {
    assert!(!is_sorted(&[1, 3, 2]));
}

#[test]
fn is_sorted_empty() {
    assert!(is_sorted::<i32>(&[]));
}

#[test]
fn is_sorted_single() {
    assert!(is_sorted(&[42]));
}

#[test]
fn is_sorted_all_same() {
    assert!(is_sorted(&[5, 5, 5, 5]));
}

#[test]
fn is_sorted_descending() {
    assert!(!is_sorted(&[5, 4, 3, 2, 1]));
}

#[test]
fn is_sorted_strings() {
    assert!(is_sorted(&["apple", "banana", "cherry"]));
    assert!(!is_sorted(&["cherry", "apple", "banana"]));
}

#[test]
fn is_sorted_versions() {
    let versions = [Version::new(1, 0, 0), Version::new(1, 1, 0), Version::new(2, 0, 0)];
    assert!(is_sorted(&versions));
}

// ==================== clamp Tests ====================

#[test]
fn clamp_within_bounds() {
    assert_eq!(clamp(&5, &1, &10), &5);
}

#[test]
fn clamp_below_min() {
    assert_eq!(clamp(&0, &1, &10), &1);
}

#[test]
fn clamp_above_max() {
    assert_eq!(clamp(&15, &1, &10), &10);
}

#[test]
fn clamp_at_min() {
    assert_eq!(clamp(&1, &1, &10), &1);
}

#[test]
fn clamp_at_max() {
    assert_eq!(clamp(&10, &1, &10), &10);
}

#[test]
fn clamp_negative_range() {
    assert_eq!(clamp(&-5, &-10, &-1), &-5);
    assert_eq!(clamp(&-15, &-10, &-1), &-10);
    assert_eq!(clamp(&0, &-10, &-1), &-1);
}

#[test]
fn clamp_single_value_range() {
    assert_eq!(clamp(&0, &5, &5), &5);
    assert_eq!(clamp(&5, &5, &5), &5);
    assert_eq!(clamp(&10, &5, &5), &5);
}

#[test]
fn clamp_strings() {
    assert_eq!(clamp(&"dog", &"ant", &"zebra"), &"dog");
    assert_eq!(clamp(&"aardvark", &"ant", &"zebra"), &"ant");
}

#[test]
fn clamp_scores() {
    let value = Score(50);
    let min = Score(25);
    let max = Score(75);
    assert_eq!(clamp(&value, &min, &max), &Score(50));
    assert_eq!(clamp(&Score(10), &min, &max), &Score(25));
    assert_eq!(clamp(&Score(100), &min, &max), &Score(75));
}

// ==================== Integration Tests ====================

#[test]
fn integration_version_min_max() {
    let versions = [
        Version::new(1, 0, 0),
        Version::new(2, 1, 0),
        Version::new(1, 5, 3),
        Version::new(0, 9, 9),
    ];
    assert_eq!(find_min(&versions), Some(&Version::new(0, 9, 9)));
    assert_eq!(find_max(&versions), Some(&Version::new(2, 1, 0)));
}

#[test]
fn integration_player_leaderboard() {
    let mut players = vec![
        Player::new("Zoe", 80),
        Player::new("Alice", 100),
        Player::new("Bob", 100),
        Player::new("Charlie", 120),
    ];
    players.sort();

    // Verify order: highest score first, then alphabetical
    assert_eq!(players[0].name, "Charlie");
    assert_eq!(players[1].name, "Alice");
    assert_eq!(players[2].name, "Bob");
    assert_eq!(players[3].name, "Zoe");
}

#[test]
fn integration_priority_queue_order() {
    let mut tasks: Vec<(Priority, &str)> = vec![
        (Priority::Low, "cleanup"),
        (Priority::Critical, "security_fix"),
        (Priority::High, "feature"),
        (Priority::Medium, "refactor"),
    ];
    tasks.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by priority descending

    assert_eq!(tasks[0].0, Priority::Critical);
    assert_eq!(tasks[1].0, Priority::High);
    assert_eq!(tasks[2].0, Priority::Medium);
    assert_eq!(tasks[3].0, Priority::Low);
}

#[test]
fn integration_temperature_range_check() {
    let room_temp = Temperature::Celsius(22.0);
    let cold = Temperature::Celsius(15.0);
    let hot = Temperature::Celsius(30.0);

    // Check if room_temp is within comfortable range
    assert!(room_temp > cold);
    assert!(room_temp < hot);
}

#[test]
fn integration_version_range() {
    let supported_min = Version::new(1, 5, 0);
    let supported_max = Version::new(2, 0, 0);
    let current = Version::new(1, 8, 3);

    assert!(current >= supported_min);
    assert!(current < supported_max);
}

#[test]
fn integration_find_and_clamp() {
    let values = [5, 10, 15, 20, 25];
    let min_val = find_min(&values).unwrap();
    let max_val = find_max(&values).unwrap();

    assert_eq!(clamp(&3, min_val, max_val), min_val);
    assert_eq!(clamp(&30, min_val, max_val), max_val);
    assert_eq!(clamp(&12, min_val, max_val), &12);
}

#[test]
fn integration_sorted_check_after_sort() {
    let mut values = vec![5, 2, 8, 1, 9];
    assert!(!is_sorted(&values));

    values.sort();
    assert!(is_sorted(&values));
}

#[test]
fn integration_score_ranking() {
    let scores = [Score(85), Score(92), Score(78), Score(92), Score(88)];

    let max_score = find_max(&scores);
    assert_eq!(max_score, Some(&Score(92)));

    let min_score = find_min(&scores);
    assert_eq!(min_score, Some(&Score(78)));

    assert!(!is_sorted(&scores));

    let mut sorted_scores = scores.to_vec();
    sorted_scores.sort();
    assert!(is_sorted(&sorted_scores));
}

#[test]
fn integration_cross_unit_temperature_sorting() {
    // This would require PartialOrd, not Ord, so we can't sort directly.
    // But we can compare individual elements.
    let temps = [
        Temperature::Celsius(0.0),
        Temperature::Fahrenheit(212.0), // 100°C
        Temperature::Celsius(50.0),
        Temperature::Fahrenheit(32.0),  // 0°C
    ];

    // Find extremes manually using partial_cmp
    let mut min_idx = 0;
    let mut max_idx = 0;

    for i in 1..temps.len() {
        if temps[i].partial_cmp(&temps[min_idx]) == Some(Ordering::Less) {
            min_idx = i;
        }
        if temps[i].partial_cmp(&temps[max_idx]) == Some(Ordering::Greater) {
            max_idx = i;
        }
    }

    // Both 0 and 3 are equal (0°C)
    assert!(temps[min_idx].to_celsius().abs() < 0.001);
    // Index 1 is 100°C
    assert!((temps[max_idx].to_celsius() - 100.0).abs() < 0.001);
}

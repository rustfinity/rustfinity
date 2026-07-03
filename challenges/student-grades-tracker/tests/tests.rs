use student_grades_tracker::*;

#[test]
fn test_add_student() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    assert!(tracker.students.contains_key("Alice"));
}

#[test]
fn test_add_duplicate_student() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.students.get_mut("Alice").unwrap().name = "Original Alice".to_string();
    tracker.add_student("Alice"); // Adding a duplicate student should not overwrite
    assert_eq!(tracker.students.len(), 1);
    assert!(tracker.students.contains_key("Alice"));
    assert_eq!(tracker.students.get("Alice").unwrap().name, "Original Alice");
}

#[test]
fn test_add_grade() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    assert_eq!(tracker.get_grades("Alice"), &[85][..]);
}

#[test]
fn test_get_grades() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    assert_eq!(tracker.get_grades("Alice"), &[85, 90][..]);
}

#[test]
fn test_multiple_students_grades() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    assert_eq!(tracker.get_grades("Alice"), &[85, 90][..]);
    assert_eq!(tracker.get_grades("Bob"), &[78][..]);
}

#[test]
fn test_grades_edge_values() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 0);
    tracker.add_grade("Alice", 100);

    assert_eq!(tracker.get_grades("Alice"), &[0, 100][..]);
}

#[test]
fn test_large_number_of_students() {
    let mut tracker = StudentGrades::new();
    for i in 0..10_000 {
        tracker.add_student(&format!("Student {}", i));
    }
    assert_eq!(tracker.students.len(), 10_000);
}

#[test]
fn test_large_number_of_grades() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");

    for grade in 0..=255 {
        tracker.add_grade("Alice", grade as u8);
    }

    assert_eq!(tracker.get_grades("Alice").len(), 256);
    assert_eq!(tracker.get_grades("Alice")[..5], [0, 1, 2, 3, 4]);
    assert_eq!(
        tracker.get_grades("Alice")[251..],
        [251, 252, 253, 254, 255]
    );
}

#[test]
fn test_grades_with_empty_student() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    assert_eq!(tracker.get_grades("Alice"), &[]);
}

#[test]
fn test_add_student_with_special_characters() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Ålice_123!@#");
    assert!(tracker.students.contains_key("Ålice_123!@#"));
}

use std::time::Instant;

use sleep_sort::SleepSort;
use sleep_sort::SleepSortable;

#[test]
fn is_sorted_empty() {
    // Arrange
    let empty_vec: Vec<u8> = Vec::new();

    // Act
    let test_start = Instant::now();
    let is_sorted = empty_vec.is_sleep_sorted();
    let test_end = Instant::now();

    // Assert
    assert!(is_sorted);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn is_sorted_vec() {
    // Arrange
    let vec = vec![1, 2, 3, 4, 5];

    // Act
    let test_start = Instant::now();
    let is_sorted = vec.is_sleep_sorted();
    let test_end = Instant::now();

    // Assert
    assert_eq!(is_sorted, true);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn is_not_sorted_vec() {
    // Arrange
    let vec = vec![5, 2, 1, 4, 3];

    // Act
    let test_start = Instant::now();
    let is_sorted = vec.is_sleep_sorted();
    let test_end = Instant::now();

    // Assert
    assert_eq!(is_sorted, false);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn is_sorted_struct_slice() {
    // Arrange
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct Id(u8);

    impl SleepSortable for Id {
        fn sleeping_time(&self) -> u8 {
            self.0
        }
    }

    let vec = [1, 2, 3, 4, 5].map(|id| Id(id));

    // Act
    let test_start = Instant::now();
    let is_sorted = vec.is_sleep_sorted();
    let test_end = Instant::now();

    // Assert
    assert_eq!(is_sorted, true);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn is_not_sorted_struct_slice() {
    // Arrange
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct Id(u8);

    impl SleepSortable for Id {
        fn sleeping_time(&self) -> u8 {
            self.0
        }
    }

    let vec = [5, 2, 1, 4, 3].map(|id| Id(id));

    // Act
    let test_start = Instant::now();
    let is_sorted = vec.is_sleep_sorted();
    let test_end = Instant::now();

    // Assert
    assert_eq!(is_sorted, false);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn is_sorted_short_circuits() {
    // Arrange
    let vec = [2, 1, 255];

    // Act
    let test_start = Instant::now();
    let is_sorted = vec.is_sleep_sorted();
    let test_end = Instant::now();

    // Assert
    assert_eq!(is_sorted, false);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

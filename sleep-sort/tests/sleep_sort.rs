use std::time::Instant;

use sleep_sort::SleepSort;
use sleep_sort::SleepSortable;

#[test]
fn sort_empty() {
    // Arrange
    let mut empty_vec: Vec<u8> = Vec::new();

    // Act
    let test_start = Instant::now();
    empty_vec.sleep_sort();
    let test_end = Instant::now();

    // Assert
    assert_eq!(empty_vec, Vec::new());
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn sort_vec_1() {
    // Arrange
    let mut unsorted = vec![20, 15, 10];
    let mut sorted = unsorted.clone();
    sorted.sort();

    // Act
    let test_start = Instant::now();
    unsorted.sleep_sort();
    let test_end = Instant::now();

    // Assert
    assert_eq!(unsorted, sorted);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn sort_vec_2() {
    // Arrange
    let mut unsorted = vec![1, 9, 6, 7, 8, 5, 2, 3, 4, 10, 0];
    let mut sorted = unsorted.clone();
    sorted.sort();

    // Act
    let test_start = Instant::now();
    unsorted.sleep_sort();
    let test_end = Instant::now();

    // Assert
    assert_eq!(unsorted, sorted);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn sort_using_custom_struct() {
    // Arrange
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Id(u8);

    impl SleepSortable for Id {
        fn sleeping_time(&self) -> u8 {
            self.0
        }
    }

    let mut unsorted = vec![1, 9, 6, 7, 8, 5, 2, 3, 4, 0]
        .into_iter()
        .map(|id| Id(id))
        .collect::<Vec<_>>();
    let sorted = (0..=9).map(|id| Id(id)).collect::<Vec<_>>();

    // Act
    let test_start = Instant::now();
    unsorted.sleep_sort();
    let test_end = Instant::now();

    // Assert
    assert_eq!(unsorted, sorted);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn sort_slice() {
    // Arrange
    let mut unsorted = [5, 3, 7, 1];
    let mut sorted = [5, 3, 7, 1];
    sorted.sort();

    // Act
    let test_start = Instant::now();
    unsorted.sleep_sort();
    let test_end = Instant::now();

    // Assert
    assert_eq!(unsorted, sorted);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

#[test]
fn sort_enum_slice() {
    // Arrange
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Order {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
    }

    impl SleepSortable for Order {
        fn sleeping_time(&self) -> u8 {
            match self {
                Order::First => 1,
                Order::Second => 2,
                Order::Third => 3,
                Order::Fourth => 4,
                Order::Fifth => 5,
            }
        }
    }

    let mut unsorted = [
        Order::Third,
        Order::Second,
        Order::Fifth,
        Order::Fourth,
        Order::First,
    ];
    let sorted = [
        Order::First,
        Order::Second,
        Order::Third,
        Order::Fourth,
        Order::Fifth,
    ];

    // Act
    let test_start = Instant::now();
    unsorted.sleep_sort();
    let test_end = Instant::now();

    // Assert
    assert_eq!(unsorted, sorted);
    assert!(test_end.duration_since(test_start).as_secs() < 30);
}

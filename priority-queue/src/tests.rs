#![allow(unused_imports)]
use crate::queue::*;

#[test]
fn test_new_empty_queue() {
    let queue: Queue<i32> = Queue::new();
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_push_front_basic() {
    let mut queue = Queue::new();

    queue.push_front(1);
    assert_eq!(queue.size(), 1);
    assert!(!queue.is_empty());
}

#[test]
fn test_push_back_basic() {
    let mut queue = Queue::new();

    queue.push_back(1);
    assert_eq!(queue.size(), 1);
    assert!(!queue.is_empty());
}

#[test]
fn test_push_front_pop_front() {
    let mut queue = Queue::new();

    queue.push_front('a');
    queue.push_front('b');
    queue.push_front('c');

    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());

    assert_eq!(queue.pop_front(), Some('c'));
    assert_eq!(queue.pop_front(), Some('b'));
    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), None);

    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_push_back_pop_back() {
    let mut queue = Queue::new();

    queue.push_back('a');
    queue.push_back('b');
    queue.push_back('c');

    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());

    assert_eq!(queue.pop_back(), Some('c'));
    assert_eq!(queue.pop_back(), Some('b'));
    assert_eq!(queue.pop_back(), Some('a'));
    assert_eq!(queue.pop_back(), None);

    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_push_front_pop_back() {
    let mut queue = Queue::new();

    queue.push_front('a');
    queue.push_front('b');
    queue.push_front('c');

    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());

    assert_eq!(queue.pop_back(), Some('a'));
    assert_eq!(queue.pop_back(), Some('b'));
    assert_eq!(queue.pop_back(), Some('c'));
    assert_eq!(queue.pop_back(), None);

    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_push_back_pop_front() {
    let mut queue = Queue::new();

    queue.push_back('a');
    queue.push_back('b');
    queue.push_back('c');

    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());

    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
    assert_eq!(queue.pop_front(), Some('c'));
    assert_eq!(queue.pop_front(), None);

    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_extend_other_empty() {
    let mut queue = Queue::new();
    queue.push_back('a');
    queue.push_back('b');
    queue.push_back('c');

    let other = Queue::new();
    queue.extend(other);

    assert_eq!(queue.size(), 3);
    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
    assert_eq!(queue.pop_front(), Some('c'));
    assert_eq!(queue.pop_front(), None);
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_extend_this_empty() {
    let mut queue = Queue::new();

    let other = {
        let mut other = Queue::new();
        other.push_back('a');
        other.push_back('b');
        other.push_back('c');
        other
    };
    queue.extend(other);

    assert_eq!(queue.size(), 3);
    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
    assert_eq!(queue.pop_front(), Some('c'));
    assert_eq!(queue.pop_front(), None);
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_extend_complete() {
    let mut queue = Queue::new();
    queue.push_back('a');
    queue.push_back('b');
    queue.push_back('c');

    let other = {
        let mut other = Queue::new();
        other.push_back('d');
        other.push_back('e');
        other.push_back('f');
        other
    };
    queue.extend(other);

    assert_eq!(queue.size(), 6);
    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
    assert_eq!(queue.pop_front(), Some('c'));
    assert_eq!(queue.pop_front(), Some('d'));
    assert_eq!(queue.pop_front(), Some('e'));
    assert_eq!(queue.pop_front(), Some('f'));
    assert_eq!(queue.pop_front(), None);
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_new_empty_pqueue() {
    let pqueue: PQueue<i32> = PQueue::new();
    assert_eq!(pqueue.size(), 0);
    assert!(pqueue.is_empty());
}

#[test]
fn test_insert_pqueue() {
    let mut pqueue = PQueue::new();

    pqueue.insert(1);
    assert_eq!(pqueue.size(), 1);
    assert!(!pqueue.is_empty());
}

#[test]
fn test_insert_to_the_front() {
    let mut queue = PQueue::new();

    queue.insert('b');
    queue.insert('a');

    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
}

#[test]
fn test_insert_to_the_back() {
    let mut queue = PQueue::new();

    queue.insert('a');
    queue.insert('b');

    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
}

#[test]
fn test_insert_to_the_middle() {
    let mut queue = PQueue::new();

    queue.insert('a');
    queue.insert('c');
    queue.insert('b');

    assert_eq!(queue.pop_front(), Some('a'));
    assert_eq!(queue.pop_front(), Some('b'));
    assert_eq!(queue.pop_front(), Some('c'));
}

#[test]
fn test_sort_whole_random() {
    use rand::seq::SliceRandom;

    let mut random_order = ('a'..='z').collect::<Vec<_>>();
    random_order.shuffle(&mut rand::thread_rng());

    let mut queue = Queue::new();
    random_order.into_iter().for_each(|c| queue.push_back(c));

    let mut pqueue: PQueue<_> = queue.into();

    ('a'..='z').for_each(|c| assert_eq!(pqueue.pop_front(), Some(c)));
}

#![allow(dead_code)]

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::{Rc, Weak},
};

pub trait Data: Copy + Clone + Display + Debug {}
impl<T: Copy + Clone + Display + Debug> Data for T {}

pub trait PData: Data + PartialOrd {}
impl<T: Data + PartialOrd> PData for T {}

pub struct Queue<T: Data> {
    size: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Node<T: Data> {
    data: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Data> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }
}

impl<T: Data> Queue<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
            }
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }

        self.tail = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                let new_head = old_head.borrow_mut().next.take();

                match new_head {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }

                self.size -= 1;
                Some(old_head.borrow().data)
            }
            None => None,
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(old_tail) => {
                let new_tail = old_tail.borrow_mut().prev.take();

                match new_tail {
                    Some(new_tail) => {
                        let new_tail_rc = new_tail.upgrade()?;
                        new_tail_rc.borrow_mut().next = None;
                        self.tail = Some(new_tail_rc);
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }

                self.size -= 1;
                Some(old_tail.borrow().data)
            }
            None => None,
        }
    }

    pub fn extend(&mut self, other: Self) {
        match self.tail.as_ref() {
            Some(tail) => {
                if let Some(head) = other.head.as_ref() {
                    tail.borrow_mut().next = Some(Rc::clone(head));
                    head.borrow_mut().prev = Some(Rc::downgrade(tail));

                    self.size += other.size;
                    self.tail = other.tail;
                }
            }
            None => {
                *self = other;
            }
        }
    }
}

pub struct PQueue<T: PData> {
    queue: Queue<T>,
}

impl<T: PData> PQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.queue.size()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.queue.pop_back()
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        let mut previous: Option<Rc<RefCell<Node<T>>>> = None;
        let mut current: Option<Rc<RefCell<Node<T>>>> = self.queue.head.clone();

        while let Some(node) = current {
            if data < node.borrow().data {
                // Insert before the current node
                match previous {
                    // ... somewhere in the middle
                    Some(previous) => {
                        previous.borrow_mut().next = Some(Rc::clone(&new_node));
                        new_node.borrow_mut().prev = Some(Rc::downgrade(&previous));
                    }
                    // ... at the front
                    None => {
                        self.queue.head = Some(Rc::clone(&new_node));
                    }
                }

                new_node.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&new_node));

                self.queue.size += 1;
                return;
            }
            previous = Some(Rc::clone(&node));
            current = node.borrow().next.clone();
        }

        self.queue.push_back(data);
    }
}

impl<T: PData> From<Queue<T>> for PQueue<T> {
    fn from(queue: Queue<T>) -> Self {
        let mut queue = queue;
        let mut pqueue = Self::new();
        while let Some(data) = queue.pop_front() {
            pqueue.insert(data);
        }

        pqueue
    }
}

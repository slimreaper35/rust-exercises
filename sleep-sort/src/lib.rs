use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;
use std::time::Duration;

pub trait SleepSort {
    fn sleep_sort(&mut self);
    fn is_sleep_sorted(&self) -> bool;
}

pub trait SleepSortable {
    fn sleeping_time(&self) -> u8;
}

impl SleepSortable for u8 {
    fn sleeping_time(&self) -> u8 {
        *self
    }
}

fn shut_up_for(duration: Duration) {
    thread::sleep(duration);
}

impl<T: SleepSortable + Send + Copy + 'static> SleepSort for [T] {
    fn sleep_sort(&mut self) {
        let (sender, receiver) = mpsc::channel();
        let barrier = Arc::new(Barrier::new(self.len()));

        for &item in self.iter() {
            let sender = sender.clone();
            let barrier = Arc::clone(&barrier);

            thread::spawn(move || {
                // Wait for all threads to be spawned
                barrier.wait();
                let time = item.sleeping_time() as u64;
                shut_up_for(Duration::from_secs(time));
                sender
                    .send(item)
                    .expect("Oops, receiver terminated before all items were processed");
            });
        }

        // Drop the sender so that the receiver can finish
        drop(sender);

        receiver.iter().enumerate().for_each(|(index, item)| {
            self[index] = item;
        });
    }

    fn is_sleep_sorted(&self) -> bool {
        let (sender, receiver) = mpsc::channel();
        let barrier = Arc::new(Barrier::new(self.len()));

        for (index, &item) in self.iter().enumerate() {
            let sender = sender.clone();
            let barrier = Arc::clone(&barrier);

            thread::spawn(move || {
                // Wait for all threads to be spawned
                barrier.wait();
                let time = item.sleeping_time() as u64;
                shut_up_for(Duration::from_secs(time));

                sender
                    .send(index)
                    .expect("Oops, receiver terminated before all items were processed");
            });
        }

        // Drop the sender so that the receiver can finish
        drop(sender);

        receiver
            .iter()
            .enumerate()
            .all(|(current_index, returned_index)| current_index == returned_index)
    }
}

use pi_estimation_channels::{NUM_POINTS, calculate_estimate, point::Point};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    rng,
};
use std::{
    fmt::Debug,
    sync::mpsc::{self, SyncSender, sync_channel},
    thread,
};

struct MessageGenerator<T> {
    num_messages: usize,
    send_channel: SyncSender<T>,
}

impl<T> MessageGenerator<T>
where
    T: Default + Debug,
    StandardUniform: Distribution<T>,
{
    pub fn new(num_messages: usize, send_channel: SyncSender<T>) -> Self {
        Self {
            num_messages,
            send_channel,
        }
    }

    pub fn send_messages(&self) {
        for _ in 0..self.num_messages {
            let value = rng().random();
            self.send_channel.send(value).unwrap();
        }
    }
}

struct PointManager {
    num_points: usize,
    receive_channel: mpsc::Receiver<Point>,
}

impl PointManager {
    pub fn new(num_points: usize, receive_channel: mpsc::Receiver<Point>) -> Self {
        Self {
            num_points,
            receive_channel,
        }
    }

    pub fn receive_stuff(&self) {
        let mut num_inside = 0;
        let mut total_points = 0;

        // TODO: Change to `while let`
        for i in 0..self.num_points {
            let point: Point = self.receive_channel.recv().unwrap();
            if point.inside_unit_circle() {
                num_inside += 1;
            }
            total_points += 1;

            if i % 10_000 == 0 {
                let estimate = calculate_estimate(num_inside, total_points);
                println!("Estimate after {i} points is {estimate}.");
            }
        }
    }
}

fn main() {
    let (send_channel, receive_channel) = sync_channel::<Point>(1_000);

    let sender_thread = thread::spawn(move || {
        let sender = MessageGenerator::new(NUM_POINTS, send_channel);
        sender.send_messages();
    });

    let receiver_thread = thread::spawn(move || {
        let receiver = PointManager::new(NUM_POINTS, receive_channel);
        receiver.receive_stuff();
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();

    println!("All done!")
}

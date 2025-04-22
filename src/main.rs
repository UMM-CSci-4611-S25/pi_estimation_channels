pub mod point;

use point::Point;
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

struct Sender<T> {
    num_messages: u32,
    send_channel: SyncSender<T>,
}

impl<T> Sender<T>
where
    T: Default + Debug,
    StandardUniform: Distribution<T>,
{
    pub fn new(num_messages: u32, send_channel: SyncSender<T>) -> Self {
        Self {
            num_messages,
            send_channel,
        }
    }

    pub fn send_stuff(&self) {
        for _ in 0..self.num_messages {
            let value = rng().random();
            println!("About to send {value:?}.");
            self.send_channel.send(value).unwrap();
        }
    }
}

struct Receiver<T> {
    num_messages: u32,
    receive_channel: mpsc::Receiver<T>,
}

impl<T> Receiver<T>
where
    T: Debug,
{
    pub fn new(num_messages: u32, receive_channel: mpsc::Receiver<T>) -> Self {
        Self {
            num_messages,
            receive_channel,
        }
    }

    pub fn receive_stuff(&self) {
        for _ in 0..self.num_messages {
            let received = self.receive_channel.recv().unwrap();
            println!("Just received {received:?}.");
        }
    }
}

const NUM_POINTS: u32 = 10_000_000;

fn main() {
    let mut num_inside = 0;
    let mut total_points = 0;

    for i in 0..NUM_POINTS {
        let point: Point = rng().random();
        if point.inside_unit_circle() {
            num_inside += 1;
        }
        total_points += 1;

        if i % 10_000 == 0 {
            print_estimate(num_inside, total_points);
        }
    }
}

fn print_estimate(num_inside: u32, total_points: u32) {
    let estimate = 4.0 * (num_inside as f64) / (total_points as f64);
    println!("After {total_points} samples our estimate is {estimate}.");
}

// fn main() {
//     const NUM_MESSAGES: u32 = 10;

//     let (send_channel, receive_channel) = sync_channel::<Point>(3);

//     let sender_thread = thread::spawn(move || {
//         let sender = Sender::new(NUM_MESSAGES, send_channel);
//         sender.send_stuff();
//     });

//     let receiver_thread = thread::spawn(move || {
//         let receiver = Receiver::new(NUM_MESSAGES, receive_channel);
//         receiver.receive_stuff();
//     });

//     sender_thread.join().unwrap();
//     receiver_thread.join().unwrap();

//     println!("All done!")
// }

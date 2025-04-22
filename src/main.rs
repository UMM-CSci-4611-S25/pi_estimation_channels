use std::{
    fmt::Debug,
    sync::mpsc::{self, SyncSender, sync_channel},
    thread,
};

use rand::{
    Rng,
    distr::{Distribution, StandardUniform, Uniform},
    rng,
};

#[derive(Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

/// Specify how to make a random point whose coordinates are between
/// -1 and 1. With this implementation we can do things like:
/// `let pt: Point = rng.random()`, and it'll figure out what
/// _kind_ of thing we want (from the type of `pt`) and then call the
/// "right" version of `sample` (namely this one). How the pieces
/// connect are somewhat complex and subtle, but it does work.
impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let dist = Uniform::new(-1.0, 1.0).unwrap();
        let x = rng.sample(dist);
        let y = rng.sample(dist);

        Point { x, y }
    }
}

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

fn main() {
    const NUM_MESSAGES: u32 = 10;

    let (send_channel, receive_channel) = sync_channel::<Point>(3);

    let sender_thread = thread::spawn(move || {
        let sender = Sender::new(NUM_MESSAGES, send_channel);
        sender.send_stuff();
    });

    let receiver_thread = thread::spawn(move || {
        let receiver = Receiver::new(NUM_MESSAGES, receive_channel);
        receiver.receive_stuff();
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();

    println!("All done!")
}

use std::{
    sync::mpsc::{self, channel, sync_channel},
    thread,
};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn random() -> Self {
        let x: f64 = rand::random();
        let y: f64 = rand::random();
        Self { x, y }
    }
}

struct Sender {
    channel: mpsc::Sender<u32>,
}

impl Sender {
    fn new(channel: mpsc::Sender<u32>) -> Self {
        Self { channel }
    }

    fn send_n_points(&self, n: u32) {
        for i in 0..n {
            println!("About to send {i:?}.");
            self.channel.send(i).unwrap();
        }
    }
}

struct Receiver {
    channel: mpsc::Receiver<u32>,
}

impl Receiver {
    fn new(channel: mpsc::Receiver<u32>) -> Self {
        Self { channel }
    }

    fn receive_n_points(&self, n: u32) {
        for _ in 0..n {
            let i = self.channel.recv().unwrap();
            println!("We received {i:?}.");
        }
    }
}

fn main() {
    const NUM_MESSAGES: u32 = 10;

    let (send_channel, receive_channel) = channel::<u32>();

    let send_thread = thread::spawn(move || {
        let sender = Sender::new(send_channel);
        sender.send_n_points(NUM_MESSAGES);
    });

    let recv_thread = thread::spawn(move || {
        let receiver = Receiver::new(receive_channel);
        receiver.receive_n_points(NUM_MESSAGES);
    });

    send_thread.join().unwrap();
    recv_thread.join().unwrap();

    println!("All done!")
}

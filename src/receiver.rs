use std::{
    fmt::Debug,
    sync::mpsc::{self, sync_channel},
};

pub struct Receiver<T> {
    send_channel: mpsc::SyncSender<T>,
    receive_channel: mpsc::Receiver<T>,
}

const CHANNEL_SIZE: usize = 100;

impl<T> Receiver<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        let (send_channel, receive_channel) = sync_channel(CHANNEL_SIZE);
        Self {
            send_channel,
            receive_channel,
        }
    }

    pub fn send_channel(&self) -> mpsc::SyncSender<T> {
        self.send_channel.clone()
    }

    pub fn receive_stuff(self) {
        // TODO: Document why we're dropping the `send_channel`.
        drop(self.send_channel);
        while let Ok(message) = self.receive_channel.recv() {
            println!("Just received {message:?}.");
        }
    }
}

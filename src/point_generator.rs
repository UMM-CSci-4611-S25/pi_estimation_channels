use rand::distr::Distribution;
use rand::distr::StandardUniform;
use std::fmt::Debug;
use std::sync::mpsc::SyncSender;

pub(crate) struct PointGenerator<T> {
    pub(crate) num_points_to_generate: u32,
    pub(crate) send_channel: SyncSender<T>,
}

impl<T> PointGenerator<T>
where
    T: Default + Debug,
    StandardUniform: Distribution<T>,
{
    pub fn new(num_points_to_generate: u32, send_channel: SyncSender<T>) -> Self {
        Self {
            num_points_to_generate,
            send_channel,
        }
    }

    pub fn generate_points(self) {
        for _ in 0..self.num_points_to_generate {
            let value = rand::random();
            println!("About to send {value:?}.");
            self.send_channel.send(value).unwrap();
        }
        drop(self.send_channel);
    }
}

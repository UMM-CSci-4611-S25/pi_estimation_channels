use std::{
    fmt::Debug,
    iter::repeat_with,
    sync::mpsc::{self, sync_channel},
    thread,
};

use point::Point;
use point_generator::PointGenerator;

mod point;
mod point_generator;
mod quadrant_classifier;

mod receiver;

fn main() {
    const NUM_MESSAGES: u32 = 10;

    let receiver: receiver::Receiver<Point> = receiver::Receiver::new();

    let generators = repeat_with(|| PointGenerator::new(NUM_MESSAGES, receiver.send_channel()))
        .take(4)
        .collect::<Vec<_>>();

    thread::scope(|scope| {
        scope.spawn(|| receiver.receive_stuff());

        for generator in generators {
            scope.spawn(|| generator.generate_points());
        }
    });

    println!("All done!")
}

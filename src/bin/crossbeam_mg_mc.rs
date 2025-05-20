use clap::Parser;
use crossbeam_channel::{Sender, unbounded};
use pi_estimation_channels::{NUM_POINTS, calculate_estimate, point::Point};
use rand::{Rng, rng};
use std::{fmt::Debug, iter::repeat_with, thread};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'g', long, default_value_t = 1)]
    num_generators: usize,
    #[arg(short = 'm', long, default_value_t = 1)]
    num_managers: usize,
}

struct MessageGenerator<T> {
    num_messages: usize,
    send_channel: Sender<T>,
}

impl MessageGenerator<Point> {
    pub fn new(num_messages: usize, send_channel: Sender<Point>) -> Self {
        Self {
            num_messages,
            send_channel,
        }
    }

    pub fn send_messages(self) {
        for _ in 0..self.num_messages {
            let value = rng().random();
            self.send_channel.send(value).unwrap();
        }

        // It's vital to drop the `send_channel` as a way of telling
        // the other end that we're done and that no more messages
        // will be coming from us. Without that the channel will never
        // close and the receiving channel will never terminate.
        drop(self.send_channel);
        println!("Done sending messages");
    }
}

struct PointManager {
    report_sender: Sender<Report>,
    point_receiver: crossbeam_channel::Receiver<Point>,
}

impl PointManager {
    pub fn new(
        report_sender: Sender<Report>,
        point_receiver: crossbeam_channel::Receiver<Point>,
    ) -> Self {
        Self {
            report_sender,
            point_receiver,
        }
    }

    pub fn receive_stuff(self) {
        let mut num_inside = 0;
        let mut total_points = 0;

        while let Ok(point) = self.point_receiver.recv() {
            if point.inside_unit_circle() {
                num_inside += 1;
            }
            total_points += 1;

            if total_points % 10_000 == 0 {
                self.report_sender
                    .send(Report::new(num_inside, total_points))
                    .unwrap();
                num_inside = 0;
                total_points = 0;
            }
        }

        println!("Done reading from generator.");

        if total_points > 0 {
            self.report_sender
                .send(Report::new(num_inside, total_points))
                .unwrap();
        }

        println!("Done receiving messages & classifying points");
    }
}

#[derive(Debug)]
struct Report {
    num_inside_points: usize,
    total_num_points: usize,
}

impl Report {
    pub fn new(num_inside_points: usize, total_num_points: usize) -> Self {
        Report {
            num_inside_points,
            total_num_points,
        }
    }
}

struct Reporter {
    sender: Sender<Report>,
    receiver: crossbeam_channel::Receiver<Report>,
}

impl Reporter {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    pub fn get_sender(&self) -> Sender<Report> {
        self.sender.clone()
    }

    pub fn report_stuff(self) {
        let mut num_inside_points = 0;
        let mut total_num_points = 0;

        drop(self.sender);

        while let Ok(report) = self.receiver.recv() {
            num_inside_points += report.num_inside_points;
            total_num_points += report.total_num_points;
            let estimate = calculate_estimate(num_inside_points, total_num_points);
            println!(
                "After {} points the estimate is {estimate}.",
                total_num_points,
            );
        }

        println!("Done generating reports");
    }
}

fn main() {
    let args = Args::parse();

    let reporter = Reporter::new();

    let (generator_sender, managers_receiver) = unbounded();

    let managers =
        repeat_with(|| PointManager::new(reporter.get_sender(), managers_receiver.clone()))
            .take(args.num_managers)
            .collect::<Vec<_>>();

    let generators = repeat_with(|| {
        MessageGenerator::new(NUM_POINTS / args.num_generators, generator_sender.clone())
    })
    .take(args.num_generators)
    .collect::<Vec<_>>();

    // It is crucial to drop these clones of the sender and receiver
    // used to communicate between the generators and the managers
    // so that the channel can be properly closed when the generators
    // are finished adding points to the channel.
    drop(managers_receiver);
    drop(generator_sender);

    thread::scope(|s| {
        for g in generators {
            s.spawn(move || {
                g.send_messages();
            });
        }
        for m in managers {
            s.spawn(move || m.receive_stuff());
        }
        s.spawn(move || {
            reporter.report_stuff();
        });
    });

    println!("All done!")
}

use clap::Parser;
use pi_estimation_channels::{NUM_POINTS, calculate_estimate, point::Point};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    rng,
};
use std::{
    fmt::Debug,
    iter::repeat_with,
    sync::mpsc::{self, Receiver, SyncSender, sync_channel},
    thread,
};

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
    send_channels: Vec<SyncSender<T>>,
}

impl<T> MessageGenerator<T>
where
    T: Default + Debug,
    StandardUniform: Distribution<T>,
{
    pub fn new(num_messages: usize, send_channels: &[SyncSender<T>]) -> Self {
        Self {
            num_messages,
            send_channels: send_channels.to_vec(),
        }
    }

    pub fn send_messages(self) {
        let num_channels = self.send_channels.len();
        for i in 0..self.num_messages {
            let value = rng().random();
            self.send_channels[i % num_channels].send(value).unwrap();
        }
        println!("Done sending messages");
    }
}

struct PointManager {
    report_sender: SyncSender<Report>,
    point_sender: SyncSender<Point>,
    point_receiver: mpsc::Receiver<Point>,
}

impl PointManager {
    pub fn new(report_sender: SyncSender<Report>) -> Self {
        let (point_sender, point_receiver) = sync_channel(1_000);
        Self {
            report_sender,
            point_sender,
            point_receiver,
        }
    }

    pub fn get_sender(&self) -> SyncSender<Point> {
        self.point_sender.clone()
    }

    pub fn receive_stuff(self) {
        let mut num_inside = 0;
        let mut total_points = 0;

        drop(self.point_sender);
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
    sender: SyncSender<Report>,
    receiver: Receiver<Report>,
}

impl Reporter {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel(1_000);
        Self { sender, receiver }
    }

    pub fn get_sender(&self) -> SyncSender<Report> {
        self.sender.clone()
    }

    pub fn report_stuff(self) {
        let mut num_inside_points = 0;
        let mut total_num_points = 0;

        drop(self.sender);

        while let Ok(report) = self.receiver.recv() {
            // println!("{report:?}");
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

    let managers = repeat_with(|| PointManager::new(reporter.get_sender()))
        .take(args.num_managers)
        .collect::<Vec<_>>();

    // We need to extract a vector of (clones of) the senders so that we can
    // send these to the generators.
    let senders = managers.iter().map(|m| m.get_sender()).collect::<Vec<_>>();

    let generators =
        repeat_with(|| MessageGenerator::new(NUM_POINTS / args.num_generators, &senders))
            .take(args.num_generators)
            .collect::<Vec<_>>();

    // It is crucial to drop these clones of the senders so that
    // when the generators finish the channels will close, allowing
    // the managers to terminate properly.
    for s in senders {
        drop(s);
    }

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

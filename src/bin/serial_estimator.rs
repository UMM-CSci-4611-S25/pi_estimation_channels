use pi_estimation_channels::{NUM_POINTS, point::Point, print_estimate};
use rand::{Rng, rng};

fn main() {
    let mut num_inside = 0;
    let mut num_outside = 0;

    for i in 0..NUM_POINTS {
        let point: Point = rng().random();
        if point.inside_unit_circle() {
            num_inside += 1;
        } else {
            num_outside += 1;
        }

        if i % 1_000 == 0 {
            print_estimate(num_inside, num_outside);
        }
    }
}

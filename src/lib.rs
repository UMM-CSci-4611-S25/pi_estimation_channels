pub mod point;

pub const NUM_POINTS: u32 = 10_000_000;

pub fn print_estimate(num_inside: u32, total_points: u32) {
    let estimate = 4.0 * (num_inside as f64) / (total_points as f64);
    println!("After {total_points} samples our estimate is {estimate}.");
}

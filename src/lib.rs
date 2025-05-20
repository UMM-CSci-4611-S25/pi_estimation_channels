pub mod point;

/// The total number of samples to take.
pub const NUM_POINTS: usize = 10_000_000;

/// `num_inside` is the number of randomly sampled points that are
/// inside the circle out of the `total_points` number of random
/// samples from the circumscribed square. We can use this to
/// estimate π via:
///
///     num_inside / total_points = (π r^2) / (4 r^2) = π/4
///
/// or equivalently
///
///     π = 4 * num_inside / total_points
///
pub fn calculate_estimate(num_inside: usize, total_points: usize) -> f64 {
    4.0 * (num_inside as f64) / (total_points as f64)
}

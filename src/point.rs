use rand::distr::Uniform;

use rand::Rng;

use rand::distr::StandardUniform;

use rand::distr::Distribution;

#[derive(Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn inside_unit_circle(&self) -> bool {
        self.x * self.x + self.y * self.y <= 1.0
    }
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

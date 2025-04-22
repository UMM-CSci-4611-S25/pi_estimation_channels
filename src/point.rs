use rand::{
    Rng,
    distr::{Distribution, StandardUniform, Uniform},
};

#[derive(Debug, Default)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let dist = Uniform::new(-1.0, 1.0).unwrap();
        let x = rng.sample(dist);
        let y = rng.sample(dist);

        Point { x, y }
    }
}

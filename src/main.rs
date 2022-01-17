use std::fmt;
use rand::{Rng};

struct Point {
    x: usize,
    y: usize,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


fn generate_points(number: usize, size: usize) -> Vec<Point> {
    let mut points_vector: Vec<Point> = Vec::with_capacity(number);
    let mut rng = rand::thread_rng();
    for _ in 0..points_vector.capacity() {
        let point : Point = Point {x: rng.gen_range(0..size - 1), y: rng.gen_range(0..size - 1)};
        points_vector.push(point);
    };

    return points_vector;
}


fn main() {
    const SIZE: usize = 1024;
    // const HEIGHT: usize = 256;
    const MAP_SEED: usize = 123456789;

    println!("Hello, world!");
    println!("{}", MAP_SEED);
    let points: Vec<Point> = generate_points(SIZE/4, SIZE);

    for i in 0..10{
        // & is useful to do a copy of the variable
        println!("{}: {}", i, &points[i])
    };
}

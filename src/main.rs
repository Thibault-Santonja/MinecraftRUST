use rand::{Rng};
pub mod voronoi;
use self::voronoi::point;


fn generate_points(number: isize, size: isize) -> Vec<point::Point> {
    let mut points_vector: Vec<point::Point> = Vec::with_capacity(number as usize);
    let mut rng = rand::thread_rng();
    for _ in 0..points_vector.capacity() {
        let point : point::Point = point::Point {x: rng.gen_range(1 -size..size - 1), y: rng.gen_range(1-size..size - 1)};
        points_vector.push(point);
    };

    return points_vector;
}

fn process_voronoi(points: Vec<point::Point>, size: isize) -> Vec<point::Point> {
    let mut edge_points_vector: Vec<point::Point> = Vec::with_capacity(size as usize + 4);
    edge_points_vector.push(point::Point::new(size, size));
    edge_points_vector.push(point::Point {x: -size, y: size});
    edge_points_vector.push(point::Point {x: size, y: -size});
    edge_points_vector.push(point::Point {x: -size, y: -size});
    edge_points_vector.extend(points);
    return process_fortune_algorithm(edge_points_vector);
}

fn process_fortune_algorithm(points: Vec<point::Point>) -> Vec<point::Point> {
    // Compute Voronoi diagram using Fortune's algorithm
    // wiki doc : https://en.wikipedia.org/wiki/Fortune's_algorithm
    return points
}


fn main() {
    const SIZE: isize = 1024;
    // const HEIGHT: usize = 256;
    const MAP_SEED: usize = 123456789;

    println!("Hello, world!");
    println!("{}", MAP_SEED);
    let mut points: Vec<point::Point> = generate_points(SIZE/4, SIZE);

    points = process_voronoi(points, SIZE);

    for i in 0..10{
        // & is useful to do a copy of the variable
        println!("{}: {}", i, &points[i])
    };
}

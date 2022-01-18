use std::fmt;

pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    /// Constructs a new `Point`.
    pub fn new(x: isize, y: isize) -> Self {
        Point {x, y}
    }

    /// Getter for the x coordinate.
    pub fn x(&self) -> isize {
        self.x
    }

    /// Getter for the y coordinate.
    pub fn y(&self) -> isize {
        self.y
    }

    /// Computes the cross product of two points, viewed as vectors from the origin.
    pub fn cross(self, point: Point) -> isize {
        self.x() * point.y() - self.y() * point.x()
    }

    /// Computes the dot product of two points, viewed as vectors from the origin.
    pub fn dot(self, point: Point) -> isize {
        self.x() * point.x() + self.y() * point.y()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0:.1}, {1:.1})", self.x(), self.y())
    }
}

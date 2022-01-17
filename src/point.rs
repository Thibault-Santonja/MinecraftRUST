use std::fmt;

pub(crate) struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
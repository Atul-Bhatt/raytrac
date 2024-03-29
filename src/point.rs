use std::ops::{Add, Mul, Div};

struct Point(f64, f64, f64);

impl Point {
    fn new() -> Point {
        Point(0.0,0.0,0.0)
    }
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        let temp = self * 1.0/rhs;
        temp
    }
}

//length
//unary negative

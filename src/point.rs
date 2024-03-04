
struct Point(f64, f64, f64);

impl Point {
    fn x(&self) -> i64 {
        self.0
    }
    fn y(&self) -> i64 {
        self.1
    }
    fn z(&self) -> i64 {
        self.2
    }
}

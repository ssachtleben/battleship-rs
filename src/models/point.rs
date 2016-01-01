pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

#[test]
fn test_get_x() {
    assert_eq!(3, Point::new(3, 5).get_x());
}

#[test]
fn test_get_y() {
    assert_eq!(5, Point::new(3, 5).get_y());
}

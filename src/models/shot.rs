struct Shot {
    x: usize,
    y: usize
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Shot {
        Shot {
            x: x,
            y: y
        }
    }

    pub fn get_x(&self) -> usize {
        return self.x;
    }

    pub fn get_y(&self) -> usize {
        return self.y;
    }
}

#[test]
fn test_get_x() {
    assert_eq!(3, Shot::new(3, 5).get_x());
}

#[test]
fn test_get_y() {
    assert_eq!(5, Shot::new(3, 5).get_y());
}

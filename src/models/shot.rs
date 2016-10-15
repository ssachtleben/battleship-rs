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

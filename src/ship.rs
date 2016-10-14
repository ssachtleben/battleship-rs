pub struct Ship {
    pub position: (usize, usize),
    pub horizontal: bool,
    pub length: usize
}

impl Ship {
    pub fn new(position_x: usize, position_y: usize, horizontal: bool, length: usize) -> Ship {
        Ship {
            position: (position_x, position_y),
            horizontal: horizontal,
            length: length
        }
    }
}

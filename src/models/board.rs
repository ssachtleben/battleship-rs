pub struct Board {
    width: usize,
    height: usize
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width: width,
            height: height
        }
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        return x > 0 && y > 0 && x <= self.get_width() && y <= self.get_height();
    }
}

#[test]
fn test_is_valid() {
    let board: Board = Board::new(10, 10);
    assert_eq!(true, board.is_valid(1, 1));
    assert_eq!(true, board.is_valid(10, 10));
    assert_eq!(false, board.is_valid(0, 0));
    assert_eq!(false, board.is_valid(1, 0));
    assert_eq!(false, board.is_valid(0, 1));
    assert_eq!(false, board.is_valid(11, 11));
    assert_eq!(false, board.is_valid(11, 10));
    assert_eq!(false, board.is_valid(10, 11));
}

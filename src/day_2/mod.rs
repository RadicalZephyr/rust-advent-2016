pub mod parse;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Keypad {
    pub num: u8,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { num: 5 }
    }

    pub fn next_key(&mut self, directions: Vec<Direction>) {
        for d in directions {
            self.do_move(d);
        }
    }

    pub fn do_move(&mut self, direction: Direction) {
        use self::Direction::*;
        match direction {
            Up => self.up(),
            Down => self.down(),
            Left => self.left(),
            Right => self.right(),
        }
    }

    pub fn up(&mut self) {
        self.num = match self.num {
            n @ 1...3 => n,
            n @ 4...9 => n - 3,
            n @ _ => n,
        }
    }

    pub fn down(&mut self) {
        self.num = match self.num {
            n @ 1...6 => n + 3,
            n @ 7...9 => n,
            n @ _ => n,
        }
    }

    pub fn left(&mut self) {
        self.num = match self.num {
            1 | 4 | 7 => self.num,
            2 | 3 | 5 | 6 | 8 | 9 => self.num - 1,
            n @ _ => n,
        }
    }

    pub fn right(&mut self) {
        self.num = match self.num {
            3 | 6 | 9 => self.num,
            1 | 2 | 4 | 5 | 7 | 8 => self.num + 1,
            n @ _ => n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_key() {
        use super::Direction::*;
        let moves = vec![Up, Left, Left];
        let mut kp = Keypad::new();
        kp.next_key(moves);
        assert_eq!(1, kp.num);
    }

    #[test]
    fn test_do_move() {
        let mut kp = Keypad::new();
        kp.do_move(Direction::Up);
        assert_eq!(2, kp.num);
        kp.do_move(Direction::Down);
        assert_eq!(5, kp.num);
    }

    #[test]
    fn test_up() {
        let mut kp = Keypad { num: 9 };
        kp.up();
        assert_eq!(6, kp.num);
        let mut kp = Keypad { num: 8 };
        kp.up();
        assert_eq!(5, kp.num);
        kp.up();
        assert_eq!(2, kp.num);
        kp.up();
        assert_eq!(2, kp.num);
    }

    #[test]
    fn test_down() {
        let mut kp = Keypad { num: 4 };
        kp.down();
        assert_eq!(7, kp.num);
        let mut kp = Keypad { num: 3 };
        kp.down();
        assert_eq!(6, kp.num);
        kp.down();
        assert_eq!(9, kp.num);
        kp.down();
        assert_eq!(9, kp.num);
    }

    #[test]
    fn test_left() {
        let mut kp = Keypad { num: 2 };
        kp.left();
        assert_eq!(1, kp.num);
        let mut kp = Keypad { num: 5 };
        kp.left();
        assert_eq!(4, kp.num);
        let mut kp = Keypad { num: 9 };
        kp.left();
        assert_eq!(8, kp.num);
        kp.left();
        assert_eq!(7, kp.num);
        kp.left();
        assert_eq!(7, kp.num);
    }

    #[test]
    fn test_right() {
        let mut kp = Keypad { num: 2 };
        kp.right();
        assert_eq!(3, kp.num);
        let mut kp = Keypad { num: 4 };
        kp.right();
        assert_eq!(5, kp.num);
        kp.right();
        assert_eq!(6, kp.num);
        kp.right();
        assert_eq!(6, kp.num);
    }
}

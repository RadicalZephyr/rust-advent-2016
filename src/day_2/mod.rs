use std::collections::HashMap;

pub mod parse;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
                m
        }
    };
);

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Keypad {
    pub num: u8,
    lookup: HashMap<u8, HashMap<Direction, u8>>,
}

impl Keypad {
    pub fn new(lookup: HashMap<u8, HashMap<Direction, u8>>) -> Self {
        Keypad {
            num: 5,
            lookup: lookup,
        }
    }

    pub fn next_key(&mut self, directions: Vec<Direction>) {
        for d in directions {
            self.do_move(d);
        }
    }

    pub fn do_move(&mut self, direction: Direction) {
        self.num = self.lookup_move(direction);
    }

    pub fn lookup_move(&self, direction: Direction) -> u8 {
        *self.lookup
            .get(&self.num)
            .map(|direction_lookup| {
                direction_lookup.get(&direction)
                    .unwrap_or(&self.num)
            })
            .unwrap_or(&self.num)
    }
}

pub fn simple_lookup() -> HashMap<u8, HashMap<Direction, u8>> {
    use self::Direction::*;
    map!(
        1 => map!(Right => 2, Down => 4),
        2 => map!(Left => 1, Right => 3, Down => 5),
        3 => map!(Left => 2, Down => 6),
        4 => map!(Right => 5, Up => 1, Down => 7),
        5 => map!(Left => 4, Right => 6, Up => 2, Down => 8),
        6 => map!(Left => 5, Up => 3, Down => 9),
        7 => map!(Right => 8, Up => 4),
        8 => map!(Left => 7, Right => 9, Up => 5),
        9 => map!(Left => 8, Up => 6)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_key() {
        use super::Direction::*;
        let moves = vec![Up, Left, Left];
        let mut kp = Keypad::new(simple_lookup());
        kp.next_key(moves);
        assert_eq!(1, kp.num);
    }

    #[test]
    fn test_do_move() {
        let mut kp = Keypad::new(simple_lookup());
        kp.do_move(Direction::Up);
        assert_eq!(2, kp.num);
        kp.do_move(Direction::Down);
        assert_eq!(5, kp.num);
    }
}

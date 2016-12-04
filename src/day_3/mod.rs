pub mod parse;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

impl Triangle {
    pub fn new(sides: (u16, u16, u16)) -> Self {
        Triangle {
            a: sides.0,
            b: sides.1,
            c: sides.2,
        }
    }

    pub fn from_block(block: ((u16, u16, u16), (u16, u16, u16), (u16, u16, u16))) -> Vec<Self> {
        let ((a1, a2, a3), (b1, b2, b3), (c1, c2, c3)) = block;
        vec![Triangle::new((a1, b1, c1)), Triangle::new((a2, b2, c2)), Triangle::new((a3, b3, c3))]
    }

    pub fn valid(&self) -> bool {
        (self.a + self.b > self.c) && (self.b + self.c > self.a) && (self.c + self.a > self.b)
    }
}

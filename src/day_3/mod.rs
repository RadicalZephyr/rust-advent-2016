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
}

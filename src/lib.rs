#[macro_use]
extern crate nom;

#[macro_use]
pub mod macros;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

#[macro_export]
macro_rules! match_parse(
    {
        $name:ident = $value:expr => $success_call:expr
    }
    =>
    {
        match $value {
            IResult::Done(_, $name) => $success_call,
            IResult::Error(error) => panic!("Error: {:?}", error),
            IResult::Incomplete(needed) => panic!("Incomplete input: {:?}", needed),
        }
    };
);

// use std::str;
// use std::str::FromStr;

named!(pub item<()>,
       value!(())
);

named!(pub things<Vec<()> >,
       many1!(item)
);

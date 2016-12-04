pub mod parse;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Room {
    sector_id: u16,
    name: String,
    checksum: String,
}

impl Room {
    pub fn from_tuple(vals: (Vec<&str>, u16, &str)) -> Self {
        let (name_parts, sector_id, checksum) = vals;
        Room {
            sector_id: sector_id,
            name: name_parts.into_iter().fold(String::new(), |mut acc, n| {
                acc.push_str(n);
                acc
            }),
            checksum: checksum.to_owned(),
        }
    }
}

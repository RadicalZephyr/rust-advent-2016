pub mod parse;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Room {
    sector_id: u16,
    checksum: String,
    name: String,
}

impl Room {
    pub fn from_tuple(vals: (u16, &str, Vec<&str>)) -> Self {
        let (sector_id, checksum, name_parts) = vals;
        Room {
            sector_id: sector_id,
            checksum: checksum.to_owned(),
            name: name_parts.into_iter().fold(String::new(), |mut acc, n| {
                acc.push_str(n);
                acc
            }),
        }
    }
}

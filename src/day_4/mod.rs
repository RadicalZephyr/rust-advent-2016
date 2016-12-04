use std::collections::HashMap;
use std::cmp::Ordering;

pub mod parse;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Room {
    pub sector_id: u16,
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

    pub fn generate_checksum(&self) -> String {
        let frequencies = self.name
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
        let mut freq_kv: Vec<(&char, &usize)> = frequencies.iter().collect();
        freq_kv.sort_by(|&(c1, count1), &(c2, count2)| {
            match count2.cmp(count1) {
                Ordering::Equal => c1.cmp(c2),
                r @ _ => r,
            }
        });
        freq_kv.into_iter().take(5).fold(String::new(), |mut acc, (c, _)| {
            acc.push(*c);
            acc
        })
    }

    pub fn is_real(&self) -> bool {
        match self.checksum.cmp(&self.generate_checksum()) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Room;

    #[test]
    fn test_generate_checksum() {
        let r = Room::from_tuple((123, "abxyz", vec!["aaaaa", "bbb", "z", "y", "x"]));
        assert_eq!(r.generate_checksum(), "abxyz");
        let r = Room::from_tuple((987, "abcde", vec!["a", "b", "c", "d", "e", "f", "g", "h"]));
        assert_eq!(r.generate_checksum(), "abcde");
        let r = Room::from_tuple((404, "oarel", vec!["not", "a", "real", "room"]));
        assert_eq!(r.generate_checksum(), "oarel");
    }

    #[test]
    fn test_is_real() {
        let r = Room::from_tuple((123, "abxyz", vec!["aaaaa", "bbb", "z", "y", "x"]));
        assert_eq!(r.is_real(), true);
        let r = Room::from_tuple((987, "abcde", vec!["a", "b", "c", "d", "e", "f", "g", "h"]));
        assert_eq!(r.is_real(), true);
        let r = Room::from_tuple((404, "oarel", vec!["not", "a", "real", "room"]));
        assert_eq!(r.is_real(), true);

        let r = Room::from_tuple((404, "decoy", vec!["totally", "real", "room"]));
        assert_eq!(r.is_real(), false);
    }
}

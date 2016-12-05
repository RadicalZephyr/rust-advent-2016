#[macro_use]
extern crate advent_2016;
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn main() {
    let door_id = "ugkcyxxp";
    part_1(door_id);
}

fn part_1(door_id: &str) {
    let mut hasher = Md5::new();
    let key = door_id.as_bytes();
    let mut digits_found = 0;

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            print!("{:X}", output[2] & 0x0f);
            digits_found += 1;
            if digits_found >= 8 {
                break;
            }
        }
        hasher.reset();
    }
}

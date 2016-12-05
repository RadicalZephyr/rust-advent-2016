#[macro_use]
extern crate advent_2016;
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn main() {
    let door_id = "ugkcyxxp";
    part_1_and_2(door_id);
}

fn part_1_and_2(door_id: &str) {
    let mut second_door_code: [u8; 8] = [0; 8];
    let mut found_code_position: u8 = 0;

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
            let sixth_char_byte: u8 = output[2] & 0x0f;
            let seventh_char_byte: u8 = output[3] >> 4;

            print!("{:x}", sixth_char_byte);
            digits_found += 1;

            if sixth_char_byte <= 7 {
                let nth_bit = found_code_position << sixth_char_byte;
                if (nth_bit & found_code_position) == nth_bit {
                    second_door_code[sixth_char_byte as usize] = seventh_char_byte;
                    found_code_position = found_code_position | nth_bit;
                }
            }

            if digits_found >= 8 && (sixth_char_byte & found_code_position) == 0xff {
                break;
            }
        }
        hasher.reset();
    }
    println!("");
    for byte in second_door_code.iter() {
        print!("{:x}", byte);
    }
}

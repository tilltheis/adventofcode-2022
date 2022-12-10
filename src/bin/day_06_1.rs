extern crate core;

use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    println!(
        "{}",
        find_end_of_marker(read_chars("src/bin/day_06_input.txt"))
    );
}

fn read_chars(path: &str) -> impl Iterator<Item = u8> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.bytes().map(|x| x.unwrap())
}

fn find_end_of_marker(mut chars: impl Iterator<Item = u8>) -> usize {
    let mut count_table = [0u8; 26];
    count_table[0] = 4;
    let mut count_multiple_sum = 1u8;
    let mut marker_end = 0usize;
    let mut marker_buffer = ['a' as u8; 4];

    #[inline]
    fn bool_to_u8(bool: bool) -> u8 {
        if bool {
            1
        } else {
            0
        }
    }

    while let Some(new_char) = chars.next() {
        let old_char = marker_buffer[marker_end % 4];
        let old_count_index = (old_char as usize) - ('a' as usize);
        count_multiple_sum -= bool_to_u8(count_table[old_count_index] > 1);
        count_table[old_count_index] -= 1;
        count_multiple_sum += bool_to_u8(count_table[old_count_index] > 1);

        marker_buffer[marker_end % 4] = new_char;
        let new_count_index = (new_char as usize) - ('a' as usize);
        count_multiple_sum -= bool_to_u8(count_table[new_count_index] > 1);
        count_table[new_count_index] += 1;
        count_multiple_sum += bool_to_u8(count_table[new_count_index] > 1);

        marker_end += 1;

        if count_multiple_sum == 0 && marker_end >= 5 {
            return marker_end;
        }
    }

    panic!("no marker found")
}

#[cfg(test)]
mod tests {
    use crate::find_end_of_marker;

    #[test]
    fn find_end_of_marker_works0() {
        assert_eq!(
            find_end_of_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".bytes()),
            7
        );
    }

    #[test]
    fn find_end_of_marker_works1() {
        assert_eq!(
            find_end_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".bytes()),
            5
        );
    }

    #[test]
    fn find_end_of_marker_works2() {
        assert_eq!(
            find_end_of_marker("nppdvjthqldpwncqszvftbrmjlhg".bytes()),
            6
        );
    }

    #[test]
    fn find_end_of_marker_works3() {
        assert_eq!(
            find_end_of_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".bytes()),
            10
        );
    }

    #[test]
    fn find_end_of_marker_works4() {
        assert_eq!(
            find_end_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".bytes()),
            11
        );
    }
}

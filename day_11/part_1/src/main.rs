use std::{
    fs::File,
    io::{BufReader, Bytes, Read},
};

fn main() {
    let blinks = std::env::args()
        .nth(1)
        .expect("No blinks provided")
        .parse()
        .expect("Cannot parse blinks");
    let path = std::env::args()
        .nth(2)
        .expect("No input file path provided");

    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let count = process(reader, blinks);

    println!("Count: {count}");
}

fn process(reader: impl Read, blinks: usize) -> usize {
    let mut nums = NumCursor::new(reader).collect::<Vec<_>>();

    for _ in 0..blinks {
        apply(&mut nums);
    }

    nums.len()
}

fn apply(nums: &mut Vec<usize>) {
    let mut i = 0;
    while i < nums.len() {
        if nums[i] == 0 {
            nums[i] = 1;
            i += 1;
            continue;
        }

        let string = nums[i].to_string();
        if string.len() % 2 == 0 {
            let (first_half, second_half) = string.split_at(string.len() / 2);
            nums[i] = first_half.parse().expect("Cannot parse number");
            nums.insert(i + 1, second_half.parse().expect("Cannot parse number"));
            i += 2;
            continue;
        }

        nums[i] *= 2024;
        i += 1;
    }
}

struct NumCursor<R>(Bytes<R>, String);

impl<R: Read> NumCursor<R> {
    fn new(reader: R) -> Self {
        Self(reader.bytes(), String::new())
    }

    fn next_byte(&mut self) -> Option<u8> {
        self.0.next().map(|byte| byte.expect("Cannot read byte"))
    }

    fn parse_clear(&mut self) -> usize {
        let result = self.1.parse().expect("Cannot parse number");
        self.1.clear();
        result
    }

    fn next(&mut self) -> Option<usize> {
        loop {
            let byte = match self.next_byte() {
                Some(byte) => byte,
                None if self.1.len() > 0 => return Some(self.parse_clear()),
                None => return None,
            };

            match byte {
                b'0'..=b'9' => self.1.push(byte as char),
                b' ' => return Some(self.parse_clear()),
                _ => panic!("Invalid byte"),
            }
        }
    }
}

impl<R: Read> Iterator for NumCursor<R> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let test = r"0 1 10 99 999";

        let mut cursor = NumCursor::new(test.as_bytes());

        assert_eq!(cursor.next(), Some(0));
        assert_eq!(cursor.next(), Some(1));
        assert_eq!(cursor.next(), Some(10));
        assert_eq!(cursor.next(), Some(99));
        assert_eq!(cursor.next(), Some(999));
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test() {
        let test = r"0 1 10 99 999";

        let result = process(test.as_bytes(), 1);
        assert_eq!(result, 7);

        let test = r"125 17";

        let result = process(test.as_bytes(), 25);
        assert_eq!(result, 55312);
    }
}

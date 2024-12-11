use std::{
    collections::HashMap,
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
    let mut count = 0;
    let mut cache = HashMap::new();
    for num in NumCursor::new(reader) {
        count += apply(num, blinks, &mut cache);
    }

    count
}

fn apply(num: usize, mut rem: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if rem == 0 {
        return 1;
    }

    rem -= 1;

    if let Some(count) = cache.get(&(num, rem)) {
        return *count;
    }

    if num == 0 {
        let result = apply(1, rem, cache);
        cache.insert((num, rem), result);
        return result;
    }

    let string = num.to_string();
    if string.len() % 2 == 1 {
        let result = apply(num * 2024, rem, cache);
        cache.insert((num, rem), result);
        return result;
    }

    let (first_half, second_half) = string.split_at(string.len() / 2);
    let first_half = first_half.parse().expect("Cannot parse number");
    let second_half = second_half.parse().expect("Cannot parse number");

    let first_half = apply(first_half, rem, cache);

    let result = first_half + apply(second_half, rem, cache);

    cache.insert((num, rem), result);

    result
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

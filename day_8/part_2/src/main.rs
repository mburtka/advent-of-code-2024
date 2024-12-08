use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut locations: [_; TOTAL_LEN] = std::array::from_fn(|_| Vec::new());
    let mut offset = 0;
    let mut columns = None;
    let mut rows = 0;
    let mut exited_newline = false;

    for byte in reader.bytes().map(|byte| byte.expect("Cannot read byte")) {
        exited_newline = false;

        if byte == b'\r' {
            continue;
        }

        if byte == b'\n' {
            exited_newline = true;
            let test = columns.get_or_insert(offset);
            assert_eq!(offset % *test, 0);
            rows += 1;
            continue;
        }

        if byte == b'.' {
            offset += 1;
            continue;
        }

        let idx = index(byte);
        locations[idx].push(offset);

        offset += 1;
    }

    if !exited_newline {
        rows += 1;
    }

    let columns = columns.expect("No columns found");
    let grid = Grid { columns, rows };
    let mut satellites = HashSet::new();

    for location in locations.iter().filter(|location| location.len() > 1) {
        location
            .iter()
            .flat_map(|x| location.iter().map(|y| (*x, *y)))
            .filter_map(|(x, y)| (x != y).then(|| grid.inverted_distanced_points(x, y)))
            .flatten()
            .for_each(|offset| {
                satellites.insert(offset);
            });
    }

    println!("Satellites: {}", satellites.len());
}

struct Grid {
    columns: usize,
    rows: usize,
}

impl Grid {
    fn offset_to_point(&self, offset: usize) -> Option<(usize, usize)> {
        match offset {
            offset if offset < self.columns * self.rows => {
                Some((offset % self.columns, offset / self.columns))
            }
            _ => None,
        }
    }

    fn point_to_offset(&self, point: (usize, usize)) -> Option<usize> {
        match point.1 * self.columns + point.0 {
            offset if offset < self.columns * self.rows => Some(offset),
            _ => None,
        }
    }

    fn inverted_distanced_points(
        &self,
        first: usize,
        second: usize,
    ) -> impl '_ + Iterator<Item = usize> {
        struct Iter<'a> {
            curr: Option<usize>,
            next: usize,
            grid: &'a Grid,
        }

        impl Iterator for Iter<'_> {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                let curr = self.curr?;
                let next = self.grid.inverted_distanced_point(curr, self.next);
                self.next = curr;
                self.curr = next;
                Some(curr)
            }
        }

        Iter {
            curr: Some(first),
            next: second,
            grid: self,
        }
    }

    fn inverted_distanced_point(&self, first: usize, second: usize) -> Option<usize> {
        let first = self.offset_to_point(first).expect("Invalid offset");
        let second = self.offset_to_point(second).expect("Invalid offset");

        let x = if first.0 < second.0 {
            let x = second.0 - first.0;
            if x > first.0 {
                return None;
            }
            first.0 - x
        } else {
            first.0 * 2 - second.0
        };

        let y = if first.1 < second.1 {
            let y = second.1 - first.1;
            if y > first.1 {
                return None;
            }
            first.1 - y
        } else {
            first.1 * 2 - second.1
        };

        if x >= self.columns || y >= self.rows {
            return None;
        }

        self.point_to_offset((x, y))
    }
}

const DIGITS_LEN: usize = len().0;
const LOWER_LEN: usize = len().1;
const TOTAL_LEN: usize = len().2;

fn index(byte: u8) -> usize {
    if byte.is_ascii_digit() {
        byte as usize - const { b'0' as usize }
    } else if byte.is_ascii_lowercase() {
        byte as usize - const { b'a' as usize - DIGITS_LEN }
    } else if byte.is_ascii_uppercase() {
        byte as usize - const { b'A' as usize - LOWER_LEN }
    } else {
        panic!("Invalid byte: {}", byte);
    }
}

const fn len() -> (usize, usize, usize) {
    let mut len = 0;
    let mut curr = b'0';
    while curr <= b'9' {
        curr += 1;
        len += 1;
    }

    let digits = len;

    let mut curr = b'a';
    while curr <= b'z' {
        curr += 1;
        len += 1;
    }

    let lower = len;

    let mut curr = b'A';
    while curr <= b'Z' {
        curr += 1;
        len += 1;
    }

    (digits, lower, len)
}

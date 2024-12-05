use std::{
    fs::File,
    io::{BufReader, Bytes, Read},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut cursor = reader.bytes();

    let mut lines = Vec::new();
    lines.push(Line::new());

    let cols = {
        let line = lines.last_mut().unwrap();
        read_line(&mut cursor, line, None).expect("Empty file")
    };

    let mut next = || {
        lines.push(Line::with_capacity(cols));
        let line = lines.last_mut().unwrap();
        read_line(&mut cursor, line, Some(cols))
    };

    while let Some(_) = next() {}

    let mut sum = 0;
    let mut i = 0;

    let at_least_four_down = lines.len() > 3;
    for _ in lines.iter() {
        process_line(&lines, &mut sum, i, at_least_four_down, cols);
        i += 1;
    }

    println!("{}", sum);
}

fn process_line(
    lines: &[Line],
    sum: &mut usize,
    line_offset: usize,
    at_least_four_down: bool,
    cols: usize,
) {
    let check_up = line_offset > 2;
    let check_down = at_least_four_down && line_offset < lines.len() - 3;

    let check_left = |col_offset| col_offset > 2;
    let check_right = |col_offset| cols > 3 && col_offset < cols - 3;

    let check_up_left = |col_offset| check_up && check_left(col_offset);
    let check_up_right = |col_offset| check_up && check_right(col_offset);
    let check_down_left = |col_offset| check_down && check_left(col_offset);
    let check_down_right = |col_offset| check_down && check_right(col_offset);

    for col_offset in lines[line_offset].xs.iter().copied() {
        const UP: Dir = Dir::Back;
        const DOWN: Dir = Dir::Forward;

        const LEFT: Dir = Dir::Back;
        const RIGHT: Dir = Dir::Forward;

        let checks = [
            (check_up, Dir::None, UP),
            (check_down, Dir::None, DOWN),
            (check_left(col_offset), LEFT, Dir::None),
            (check_right(col_offset), RIGHT, Dir::None),
            (check_up_left(col_offset), LEFT, UP),
            (check_up_right(col_offset), RIGHT, UP),
            (check_down_left(col_offset), LEFT, DOWN),
            (check_down_right(col_offset), RIGHT, DOWN),
        ];

        'outer: for (_c, x_shift, y_shift) in checks.iter().filter(|(check, _, _)| *check) {
            let mut line_offset = line_offset;
            let mut col_offset = col_offset;

            for b in b"MAS" {
                x_shift.apply(&mut col_offset);
                y_shift.apply(&mut line_offset);

                if lines[line_offset].bytes[col_offset] != *b {
                    continue 'outer;
                }
            }

            *sum += 1;
        }
    }
}

fn read_line(
    cursor: &mut Bytes<BufReader<File>>,
    line: &mut Line,
    expect: Option<usize>,
) -> Option<usize> {
    let mut cols = 0;
    let out = loop {
        match cursor.next() {
            Some(Ok(b'\r')) => {}

            Some(Ok(b'\n')) => break (true, cols),

            Some(x) => {
                let x = x.expect("Error reading file");

                if x == b'X' {
                    line.xs.push(cols);
                }

                cols += 1;

                line.bytes.push(x);
            }

            None => break (false, cols),
        }
    };

    if !out.0 && cols == 0 {
        return None;
    }

    if let Some(expect) = expect {
        assert_eq!(out.1, expect, "Mismatched column count");
    }

    out.0.then(|| out.1)
}

#[derive(Clone, Copy)]
enum Dir {
    Back,
    Forward,
    None,
}

impl Dir {
    fn apply(&self, to: &mut usize) {
        match self {
            Self::Back => *to -= 1,
            Self::Forward => *to += 1,
            Self::None => {}
        }
    }
}

struct Line {
    xs: Vec<usize>,
    bytes: Vec<u8>,
}

impl Line {
    fn new() -> Self {
        Self {
            xs: Vec::new(),
            bytes: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            xs: Vec::new(),
            bytes: Vec::with_capacity(capacity),
        }
    }
}

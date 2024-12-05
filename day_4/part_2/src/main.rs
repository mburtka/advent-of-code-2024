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

    for i in 0..(lines.len() - 2) {
        for (offset, find) in lines[i]
            .m_or_s
            .iter()
            .copied()
            .filter(|(i, _)| *i < cols - 2)
        {
            if lines[i + 1].bytes[offset + 1] != b'A' {
                continue;
            }

            let next = match lines[i].bytes[offset + 2] {
                b'M' => b'S',
                b'S' => b'M',
                _ => continue,
            };

            let line = &lines[i + 2];
            if line.bytes[offset + 2] == find && line.bytes[offset] == next {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
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

                match x {
                    b'M' => line.m_or_s.push((cols, b'S')),
                    b'S' => line.m_or_s.push((cols, b'M')),
                    _ => {}
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

struct Line {
    m_or_s: Vec<(usize, u8)>,
    bytes: Vec<u8>,
}

impl Line {
    fn new() -> Self {
        Self {
            m_or_s: Vec::new(),
            bytes: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            m_or_s: Vec::new(),
            bytes: Vec::with_capacity(capacity),
        }
    }
}

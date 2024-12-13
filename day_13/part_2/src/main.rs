use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");

    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let count = process(reader);

    println!("Count: {count}");
}

fn process(reader: impl BufRead) -> usize {
    const ADD: isize = 10000000000000;

    let mut lines = reader.lines().map(|line| line.expect("No button A line"));

    let mut total = 0;
    while let Some(line) = lines.next() {
        let (a1, a2) = parse_button_line(&line);
        let (b1, b2) = parse_button_line(&lines.next().expect("No button B line"));
        let (mut c1, mut c2) = parse_prize_line(&lines.next().expect("No prize line"));

        lines.next();

        c1 += ADD;
        c2 += ADD;

        if a1 == a2 && b1 == b2 {
            if c1 == c2 {
                panic!("Infinite solutions");
            }

            continue;
        }

        let num = c1 * b2 - c2 * b1;
        let den = a1 * b2 - a2 * b1;
        let x = num / den;

        if num % den != 0 || x < 0 {
            continue;
        }

        let num = c2 - a2 * x;
        let den = b2;
        let y = num / den;

        if num % den != 0 || y < 0 {
            continue;
        }

        total += x as usize * 3 + y as usize;
    }

    total
}

fn parse_button_line(line: &str) -> (isize, isize) {
    const BUTTON_A: usize = "Button A: X+".len();

    let comma = line.find(',').expect("No comma found");
    let x = &line[BUTTON_A..comma];
    let y = &line[comma + const { ", Y+" }.len()..];

    (
        x.parse().expect("Cannot parse X"),
        y.parse().expect("Cannot parse Y"),
    )
}

fn parse_prize_line(line: &str) -> (isize, isize) {
    const PRIZE: usize = "Prize: X=".len();

    let comma = line.find(',').expect("No comma found");
    let x = &line[PRIZE..comma];
    let y = &line[comma + const { ", Y=" }.len()..];

    (
        x.parse().expect("Cannot parse X"),
        y.parse().expect("Cannot parse Y"),
    )
}

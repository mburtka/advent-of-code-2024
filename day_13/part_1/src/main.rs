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
    let mut lines = reader.lines().map(|line| line.expect("Cannot read line"));

    let mut total = 0;

    while let Some(line) = lines.next() {
        let a = parse_button_line(&line);
        let b = parse_button_line(&lines.next().expect("No button B line"));
        let prize = parse_prize_line(&lines.next().expect("No prize line"));

        if let Some(count) = solve(a, b, prize) {
            total += count;
        }

        lines.next();
    }

    total
}

fn solve(a: Point, b: Point, prize: Point) -> Option<usize> {
    for (a_count, b_count) in Matches::new(a.0, b.0, prize.0) {
        let result = a.mul(a_count) + b.mul(b_count);
        if result == prize {
            return Some(3 * a_count + b_count);
        }
    }
    None
}

fn parse_button_line(line: &str) -> Point {
    const BUTTON_A: usize = "Button A: X+".len();

    let comma = line.find(',').expect("No comma found");
    let x = &line[BUTTON_A..comma];
    let y = &line[comma + const { ", Y+" }.len()..];

    Point(
        x.parse().expect("Cannot parse X"),
        y.parse().expect("Cannot parse Y"),
    )
}

fn parse_prize_line(line: &str) -> Point {
    const PRIZE: usize = "Prize: X=".len();

    let comma = line.find(',').expect("No comma found");
    let x = &line[PRIZE..comma];
    let y = &line[comma + const { ", Y=" }.len()..];

    Point(
        x.parse().expect("Cannot parse X"),
        y.parse().expect("Cannot parse Y"),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn mul(&self, value: usize) -> Self {
        Self(self.0 * value, self.1 * value)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

struct Matches {
    a_value: usize,
    b_value: usize,
    a_count: usize,
    b_count: usize,
    target: usize,
}

impl Matches {
    fn new(a_value: usize, b_value: usize, target: usize) -> Self {
        let b_count = target / b_value;
        Self {
            a_value,
            b_value,
            a_count: 0,
            b_count,
            target,
        }
    }

    fn score(&self) -> usize {
        self.a_count * self.a_value + self.b_count * self.b_value
    }

    fn next(&mut self) -> Option<(usize, usize)> {
        while self.b_count > 0 {
            while self.score() < self.target {
                self.a_count += 1;
            }

            if self.score() == self.target {
                let b_count = self.b_count;
                if self.b_count > 0 {
                    self.b_count -= 1;
                }

                return Some((self.a_count, b_count));
            }

            while self.score() > self.target && self.b_count > 0 {
                self.b_count -= 1;
            }

            if self.score() == self.target {
                let b_count = self.b_count;
                if self.b_count > 0 {
                    self.b_count -= 1;
                }

                return Some((self.a_count, b_count));
            }
        }
        None
    }
}

impl Iterator for Matches {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        let a = Point(94, 34);
        let b = Point(22, 67);
        let prize = Point(8400, 5400);

        let matches = Matches::new(a.0, b.0, prize.0);
        for candidate in matches {
            let result = a.mul(candidate.0) + b.mul(candidate.1);
            assert_eq!(result.0, prize.0);
        }
    }

    #[test]
    fn test() {
        let test = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
        "
        .trim();

        let result = process(test.as_bytes());

        assert_eq!(result, 480);
    }
}

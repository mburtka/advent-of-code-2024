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

    let mut safe = 0;

    'outer: for line in reader.lines().map(|line| line.expect("Cannot read line")) {
        let mut iter = line
            .split(' ')
            .map(|number| number.parse::<isize>().expect("Cannot parse number"))
            .into_iter();

        let prev = iter.next().expect("First number missing");
        let next = iter.next().expect("Second number missing");

        let diff = next - prev;
        let is_asc = match diff {
            -3..=-1 => false,
            1..=3 => true,
            _ => continue,
        };

        let mut prev = next;
        while let Some(next) = iter.next() {
            match (is_asc, next - prev) {
                (true, 1..=3) | (false, -3..=-1) => {}
                _ => continue 'outer,
            }

            prev = next;
        }

        safe += 1;
    }

    println!("Safe: {}", safe);
}

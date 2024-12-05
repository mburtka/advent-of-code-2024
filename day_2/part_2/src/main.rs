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

        let cmp = |x, y| match y - x {
            -3..=-1 => Some(false),
            1..=3 => Some(true),
            _ => None,
        };

        let first = iter.next().expect("No first number");
        let second = iter.next().expect("No second number");

        let third = match iter.next() {
            None => {
                safe += 1;
                continue;
            }
            Some(third) => third,
        };

        let mut next = match iter.next() {
            None => {
                for check in [(first, second), (second, third), (first, third)].iter() {
                    if cmp(check.0, check.1).is_some() {
                        safe += 1;
                        continue 'outer;
                    }
                }
                continue;
            }
            Some(fourth) => fourth,
        };

        let three_way_cmp = |x, y, z| match (cmp(x, y), cmp(y, z)) {
            (Some(x), Some(y)) if x == y => Some(x),
            _ => None,
        };

        let matches = |x, y, is_asc| matches!(cmp(x, y), Some(x) if x == is_asc);

        // three of the first four must form our asc/desc determination
        let (mut prev, mut skipped, is_asc) =
            if let Some(is_asc) = three_way_cmp(first, second, third) {
                (third, false, is_asc)
            } else {
                let checks = [
                    (first, second, next),
                    (first, third, next),
                    (second, third, next),
                ];
                let mut check = checks.iter();

                loop {
                    let (x, y, z) = match check.next() {
                        None => continue 'outer,
                        Some(check) => check,
                    };

                    match three_way_cmp(*x, *y, *z) {
                        Some(is_asc) => {
                            next = match iter.next() {
                                None => {
                                    safe += 1;
                                    continue 'outer;
                                }
                                Some(next) => next,
                            };

                            break (*z, true, is_asc);
                        }
                        None => {}
                    }
                }
            };

        loop {
            let hold = match iter.next() {
                None => {
                    if !skipped || matches(prev, next, is_asc) {
                        safe += 1;
                    }
                    break;
                }
                Some(x) => x,
            };

            if matches(prev, next, is_asc) {
                prev = next;
                next = hold;

                continue;
            }

            if skipped {
                break;
            }

            skipped = true;

            if matches(prev, hold, is_asc) {
                next = hold;
                continue;
            }

            break;
        }
    }

    println!("Safe: {}", safe);
}

use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut count = HashMap::new();

    for line in reader.lines().map(|line| line.expect("Cannot read line")) {
        let (left, right) = line.split_once(',').expect("Line is not comma delimited");
        let left = left.parse::<usize>().expect("Cannot parse left number");
        let right = right.parse::<usize>().expect("Cannot parse right number");

        match count.entry(left) {
            Entry::Vacant(entry) => {
                if left == right {
                    entry.insert((1usize, true));
                    continue;
                } else {
                    entry.insert((0, true));
                }
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                entry.1 = true;

                if left == right {
                    entry.0 += 1;
                    continue;
                }
            }
        }

        match count.entry(right) {
            Entry::Vacant(entry) => {
                entry.insert((1, false));
            }
            Entry::Occupied(mut entry) => {
                let (ref mut value, _) = entry.get_mut();
                *value += 1;
            }
        }
    }

    let sum: usize = count
        .iter()
        .filter_map(|(left, (count, has_left))| {
            if *has_left {
                Some(*left * *count)
            } else {
                None
            }
        })
        .sum();

    println!("Sum: {}", sum);
}

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

    let mut left_data = Vec::new();
    let mut right_data = Vec::new();

    for line in reader.lines().map(|line| line.expect("Cannot read line")) {
        let (left, right) = line.split_once(',').expect("Line is not comma delimited");
        let left = left.parse::<usize>().expect("Cannot parse left number");
        let right = right.parse::<usize>().expect("Cannot parse right number");

        left_data.push(left);
        right_data.push(right);
    }

    left_data.sort();
    right_data.sort();

    let mut sum = 0;

    for (i, left) in left_data.iter().enumerate() {
        let right = right_data[i];

        sum += match left.cmp(&right) {
            std::cmp::Ordering::Less => right - left,
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Greater => left - right,
        }
    }

    println!("Sum: {}", sum);
}

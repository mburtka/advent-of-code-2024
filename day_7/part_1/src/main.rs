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

    let mut sum = 0;

    for line in reader.lines().map(|line| line.expect("Cannot read line")) {
        let (test, nums) = line.split_once(':').expect("Invalid input");

        let test = test.parse().expect("Invalid number");
        let nums = nums
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("Invalid number"))
            .collect();

        if solve(test, &nums) {
            sum += test;
        }
    }

    println!("Sum: {}", sum);
}

fn solve(test: usize, nums: &Vec<usize>) -> bool {
    fn solve(test: usize, nums: &[usize], idx: usize, running: usize) -> bool {
        if idx == nums.len() {
            return running == test;
        }

        let num = nums[idx];
        let idx = idx + 1;

        solve(test, nums, idx, running + num) || solve(test, nums, idx, running * num)
    }
    solve(test, nums, 0, 0)
}

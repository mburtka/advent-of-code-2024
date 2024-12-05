use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut rules = HashMap::new();

    let iter = reader.lines().map(|line| line.expect("Cannot read line"));
    let mut cursor = RuleCursor::new(iter);

    while let Some((left, right)) = cursor.next() {
        match rules.entry(left) {
            Entry::Vacant(entry) => {
                let mut set = HashSet::new();
                set.insert(right);
                entry.insert(set);
            }

            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(right);
            }
        }
    }

    let mut pages = cursor.into_pages();
    let mut sum = 0;

    'outer: while let Some(pages) = pages.next() {
        let mut cant_be = HashSet::new();
        let mut encountered = HashSet::new();
        for page in pages.iter().rev() {
            if cant_be.contains(page) {
                continue 'outer;
            }

            encountered.insert(page);

            if let Some(rules) = rules.get(page) {
                for rule in rules {
                    cant_be.insert(*rule);
                }
            }
        }

        let mid = pages.len() / 2;
        sum += pages[mid];
    }

    println!("Sum: {}", sum);
}

struct RuleCursor<I> {
    iter: I,
}

impl<I: Iterator<Item = String>> RuleCursor<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }

    fn next(&mut self) -> Option<(usize, usize)> {
        let line = self.iter.next()?;

        if line == "" {
            return None;
        }

        let (left, right) = line.split_once('|').expect("Line is not comma delimited");
        let left = left.parse().expect("Cannot parse left number");
        let right = right.parse().expect("Cannot parse right number");

        Some((left, right))
    }

    fn into_pages(self) -> PagesCursor<I> {
        PagesCursor { iter: self.iter }
    }
}

struct PagesCursor<I> {
    iter: I,
}

impl<I: Iterator<Item = String>> PagesCursor<I> {
    fn next(&mut self) -> Option<Vec<usize>> {
        Some(
            self.iter
                .next()?
                .split(',')
                .map(|num| num.parse().expect("Cannot parse number"))
                .collect(),
        )
    }
}

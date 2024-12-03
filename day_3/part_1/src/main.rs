use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut cursor = reader.bytes();

    let mut sum = 0;
    let mut skip_m = false;

    'outer: loop {
        for c in b"mul(".iter().skip({
            if skip_m {
                skip_m = false;
                1
            } else {
                0
            }
        }) {
            match cursor.next() {
                Some(Ok(b)) if b == *c => {},
                Some(x) => {
                    assert!(x.is_ok(), "Could not read byte");
                    continue 'outer;
                },
                None => break 'outer,
            }
        }

        let mut num_1 = String::with_capacity(3);
        let mut num_2 = String::with_capacity(3);

        let mut is_num_1 = true;
        let mut num = &mut num_1;

        let (num_1, num_2) = loop {
            let next = cursor.next();

            match next {
                Some(Ok(x)) if b'0' <= x && x <= b'9' => {
                    num.push(x as char);

                    if num.len() > 3 {
                        continue 'outer;
                    }
                },

                Some(Ok(b',')) if is_num_1 => {
                    if num.len() == 0 {
                        continue 'outer;
                    }

                    is_num_1 = false;
                    num = &mut num_2;
                },

                Some(Ok(b')')) if !is_num_1 => {
                    if num.len() == 0 {
                        continue 'outer;
                    }

                    break (num_1, num_2);
                },

                Some(Ok(x)) => {
                    skip_m = x == b'm';
                    continue 'outer;
                },

                Some(Err(_)) => {
                    panic!("Could not read byte");
                },

                None => break 'outer,
            }
        };

        let num_1 = num_1.parse::<usize>().expect("Could not parse number");
        let num_2 = num_2.parse::<usize>().expect("Could not parse number");

        sum += num_1 * num_2;
    }

    println!("Sum: {}", sum);
}

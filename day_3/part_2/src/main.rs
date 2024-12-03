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
    let mut continue_with = None;
    let mut enabled = true;

    'outer: loop {
        let mut next = continue_with.take().or_else(|| cursor.next());

        for c in b"mul(" {
            match next {
                Some(Ok(b)) if enabled && b == *c => {},

                Some(Ok(b)) if b == b'd' => {
                    let next  = cursor.next();
                    match next {
                        Some(Ok(b'o')) => {},
                        None => break 'outer,
                        _ => {
                            continue_with = next;
                            continue 'outer;
                        }
                    }

                    let next = cursor.next();
                    match next {
                        Some(Ok(b'n')) => {},

                        Some(Ok(b'(')) => {
                            let next = cursor.next();
                            if let Some(Ok(b')')) = next {
                                enabled = true;
                            } else {
                                continue_with = next;
                            }
                            continue 'outer;
                        },

                        None => break 'outer,

                        _ => {
                            continue_with = next;
                            continue 'outer;
                        }
                    }

                    for c in b"'t()" {
                        let next = cursor.next();
                        match next {
                            Some(Ok(b)) if b == *c => {},
                            Some(_) => {
                                continue_with = next;
                                continue 'outer;
                            },
                            None => break 'outer,
                        }
                    }

                    enabled = false;
                    continue 'outer;
                },

                Some(x) => {
                    assert!(x.is_ok(), "Could not read byte");
                    continue 'outer;
                },

                None => break 'outer,
            }
            next = cursor.next();
        }

        let mut num_1 = String::with_capacity(3);
        let mut num_2 = String::with_capacity(3);

        let mut is_num_1 = true;
        let mut num = &mut num_1;

        let (num_1, num_2) = loop {
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

                Some(_) => {
                    continue_with = next;
                    continue 'outer;
                },

                None => break 'outer,
            }
        
            next = cursor.next();
        };

        let num_1 = num_1.parse::<usize>().expect("Could not parse number");
        let num_2 = num_2.parse::<usize>().expect("Could not parse number");

        sum += num_1 * num_2;
    }

    println!("Sum: {}", sum);
}

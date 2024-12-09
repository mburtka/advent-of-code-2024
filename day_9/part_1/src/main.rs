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

    let sum = process(reader);

    println!("{sum}");
}

fn process(reader: impl Read) -> usize {
    let mut bytes = reader
        .bytes()
        .map(|byte| byte.expect("Cannot read byte"))
        .map(|b| {
            assert!(b.is_ascii_digit());
            (b - b'0') as usize
        });

    let mut disk = Vec::new();

    while let Some(size) = bytes.next() {
        let free = bytes.next().unwrap_or(0);
        disk.push((size, free));
    }

    let mut sum = 0;
    let mut offset = 0;
    let mut id = 0;
    let mut tail = disk.len() - 1;

    loop {
        let start = offset;

        let (size, _) = disk[id];

        offset += size;

        sum += (start..offset).map(|i| i * id).sum::<usize>();

        if id == tail {
            break;
        }

        while tail > id {
            let (front, back) = disk.split_at_mut(tail);

            let (size, tail_free) = &mut back[0];
            let fill = (*size).min(front[id].1);

            *size -= fill;
            *tail_free += fill;

            let start = offset;
            offset += fill;

            front[id].1 -= fill;

            sum += (start..offset).map(|i| i * tail).sum::<usize>();

            if size == &0 {
                tail -= 1;
            }

            if front[id].1 == 0 {
                break;
            }
        }

        if id == tail {
            break;
        }

        id += 1;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        let reader = input.as_bytes();
        let sum = process(reader);

        assert_eq!(sum, 1928);
    }
}

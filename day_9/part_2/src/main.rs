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

    let mut offset = 0;
    while let Some(size) = bytes.next() {
        let free = bytes.next().unwrap_or(0);
        disk.push((size, free, 0, offset));
        offset += size + free;
    }

    let mut sum = 0;
    let tail = disk.len() - 1;

    for tail in (1..=tail).rev() {
        let (front, back) = disk.split_at_mut(tail);
        let (back_size, _, _, _) = &mut back[0];

        for i in 0..tail {
            let (front_size, free, used, offset) = &mut front[i];

            if back_size > free {
                continue;
            }

            let start = *offset + *front_size + *used;
            let end = start + *back_size;

            sum += (start..end).map(|i| i * tail).sum::<usize>();

            *used += *back_size;
            *free -= *back_size;
            *back_size = 0;

            break;
        }
    }

    for (id, (size, _, _, offset)) in disk.into_iter().enumerate() {
        let end = offset + size;
        sum += (offset..end).map(|i| i * id).sum::<usize>();
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

        assert_eq!(sum, 2858);
    }
}

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
    let bytes = reader.bytes().map(|byte| byte.expect("Cannot read byte"));

    let mut grid = Vec::new();
    let mut offset = 0;
    let mut columns = None;
    let mut zeroes = Vec::new();

    for byte in bytes {
        match byte {
            b'0' => {
                grid.push(0);
                zeroes.push(offset);
                offset += 1;
            }

            b'1'..=b'9' => {
                grid.push(byte - b'0');
                offset += 1;
            }

            b'\n' => {
                columns.get_or_insert(offset);
            }

            b'\r' => {}

            _ => panic!("Invalid byte"),
        }
    }

    let grid = Grid {
        columns: columns.expect("No columns found"),
        grid,
    };

    grid.solve(&zeroes)
}

struct Grid {
    columns: usize,
    grid: Vec<u8>,
}

impl Grid {
    fn offset_to_point(&self, offset: usize) -> Option<(usize, usize)> {
        match offset {
            offset if offset < self.grid.len() => {
                Some((offset % self.columns, offset / self.columns))
            }
            _ => None,
        }
    }

    fn point_to_offset(&self, point: (usize, usize)) -> Option<usize> {
        match point.1 * self.columns + point.0 {
            offset if offset < self.grid.len() => Some(offset),
            _ => None,
        }
    }

    fn solve(&self, starts: &[usize]) -> usize {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Dir {
            Up,
            Down,
            Left,
            Right,
        }

        impl Dir {
            fn apply(&self, grid: &Grid, offset: usize) -> Option<usize> {
                let (x, y) = grid.offset_to_point(offset).expect("Invalid offset");
                if x == 0 && self == &Dir::Left || y == 0 && self == &Dir::Up {
                    return None;
                }

                let x = match self {
                    Dir::Left => x - 1,
                    Dir::Right => x + 1,
                    _ => x,
                };

                let y = match self {
                    Dir::Up => y - 1,
                    Dir::Down => y + 1,
                    _ => y,
                };

                grid.point_to_offset((x, y))
            }

            const fn next(&self) -> [Self; 3] {
                match self {
                    Dir::Up => [Dir::Left, Dir::Up, Dir::Right],
                    Dir::Down => [Dir::Right, Dir::Down, Dir::Left],
                    Dir::Left => [Dir::Down, Dir::Left, Dir::Up],
                    Dir::Right => [Dir::Up, Dir::Right, Dir::Down],
                }
            }
        }

        fn solve<const N: usize>(
            grid: &Grid,
            offset: usize,
            dirs: &[Dir; N],
            cache: &mut Vec<Option<usize>>,
        ) -> usize {
            if let Some(cache) = cache[offset] {
                return cache;
            }

            let value = grid.grid[offset];
            let mut sum = 0;

            for dir in dirs {
                let offset = match dir.apply(grid, offset) {
                    Some(offset) => offset,
                    None => continue,
                };

                let next = grid.grid[offset];

                if next != value + 1 {
                    continue;
                }

                if value == 8 {
                    if next == 9 {
                        sum += 1;
                    }
                } else {
                    sum += solve(grid, offset, &dir.next(), cache);
                }
            }

            cache[offset] = Some(sum);
            sum
        }

        let mut sum = 0;
        let mut cache = vec![None; self.grid.len()];
        for start in starts {
            sum += solve(
                self,
                *start,
                const { &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] },
                &mut cache,
            );
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let b = r"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "
        .trim()
        .replace(' ', "");

        let sum = process(b.as_bytes());

        assert_eq!(sum, 81);
    }
}

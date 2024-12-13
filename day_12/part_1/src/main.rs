use core::panic;
use std::{
    fs::File,
    io::{BufReader, Bytes, Read},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");

    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let count = process(reader);

    println!("Count: {count}");
}

fn process(reader: impl Read) -> usize {
    let bytes = reader.bytes().map(|byte| byte.expect("Cannot read byte"));
    let cap = bytes.size_hint().1.unwrap_or(bytes.size_hint().0);

    let mut columns = None;
    let mut grid = Vec::with_capacity(cap);

    for byte in bytes {
        match byte {
            b'A'..=b'Z' => {
                grid.push(byte);
            }

            b'\n' => {
                columns.get_or_insert(grid.len());
            }

            b'\r' => continue,

            _ => panic!("Invalid byte {byte}"),
        }
    }

    let columns = columns.expect("No columns found");
    let grid = Grid { columns, grid };
    grid.solve()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    const fn next(&self) -> [Self; 3] {
        match self {
            Dir::Up => [Dir::Left, Dir::Up, Dir::Right],
            Dir::Down => [Dir::Right, Dir::Down, Dir::Left],
            Dir::Left => [Dir::Down, Dir::Left, Dir::Up],
            Dir::Right => [Dir::Up, Dir::Right, Dir::Down],
        }
    }
}

struct Grid {
    columns: usize,
    grid: Vec<u8>,
}

impl Grid {
    fn _offset_to_point(&self, offset: usize) -> (usize, usize) {
        (offset % self.columns, offset / self.columns)
    }

    fn _point_to_offset(&self, point: (usize, usize)) -> usize {
        point.1 * self.columns + point.0
    }

    fn rows(&self) -> usize {
        self.grid.len() / self.columns
    }

    fn next(&self, pos: usize, dir: Dir) -> Option<usize> {
        let (x, y) = self._offset_to_point(pos);

        let (x, y) = match (dir, x, y) {
            (Dir::Up, x, y) if y > 0 => (x, y - 1),
            (Dir::Down, x, y) if y < self.rows() - 1 => (x, y + 1),
            (Dir::Left, x, y) if x > 0 => (x - 1, y),
            (Dir::Right, x, y) if x < self.columns - 1 => (x + 1, y),
            _ => return None,
        };

        Some(self._point_to_offset((x, y)))
    }

    fn solve(&self) -> usize {
        let mut visited = vec![false; self.grid.len()];
        let mut origin = 0;

        let mut region = Region::origin();
        self._solve(origin, [Dir::Right, Dir::Down], &mut region, &mut visited);
        let mut total = region.cost();

        while origin < self.grid.len() {
            if visited[origin] {
                origin += 1;
                continue;
            }

            let mut region = Region::new();
            self._solve(
                origin,
                [Dir::Right, Dir::Down, Dir::Left, Dir::Up],
                &mut region,
                &mut visited,
            );

            total += region.cost();
        }

        total
    }

    fn _solve<const N: usize>(
        &self,
        offset: usize,
        next: [Dir; N],
        region: &mut Region,
        visited: &mut Vec<bool>,
    ) {
        visited[offset] = true;
        region.area += 1;

        let value = self.grid[offset];

        for dir in next.iter() {
            match self.next(offset, *dir) {
                None => {
                    region.perimeter += 1;
                }
                Some(p) if self.grid[p] != value => {
                    region.perimeter += 1;
                }
                Some(p) if visited[p] => {}
                Some(p) => {
                    self._solve(p, dir.next(), region, visited);
                }
            }
        }
    }
}

struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn new() -> Self {
        Self {
            area: 0,
            perimeter: 0,
        }
    }

    fn origin() -> Self {
        Self {
            area: 0,
            perimeter: 2,
        }
    }

    fn cost(&self) -> usize {
        self.area * self.perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = r"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
        "
        .trim()
        .replace(' ', "");

        let result = process(test.as_bytes());

        assert_eq!(result, 1930);
    }
}

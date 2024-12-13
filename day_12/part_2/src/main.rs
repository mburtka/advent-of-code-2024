use core::panic;
use std::{
    fs::File,
    io::{BufReader, Bytes, Read, Write},
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
        let mut offset = 0;
        let mut visited = vec![false; self.grid.len()];
        let mut interior = vec![false; self.grid.len()];
        let mut total = 0;

        let mut stack = Vec::new();

        while offset < self.grid.len() {
            if visited[offset] {
                offset += 1;
                continue;
            }

            let mut region = Region {
                area: 0,
                vertices: 0,
            };
            stack.push(offset);

            while let Some(offset) = stack.pop() {
                visited[offset] = true;
                region.area += 1;

                let value = self.grid[offset];

                let left = self.next(offset, Dir::Left);
                let up_left = left.and_then(|offset| self.next(offset, Dir::Up));
                let up = self.next(offset, Dir::Up);
                let up_right = up.and_then(|offset| self.next(offset, Dir::Right));
                let right = self.next(offset, Dir::Right);
                let down_right = right.and_then(|offset| self.next(offset, Dir::Down));
                let down = self.next(offset, Dir::Down);
                let down_left = down.and_then(|offset| self.next(offset, Dir::Left));

                for (x, y, diag) in [
                    (left, up, up_left),
                    (up, right, up_right),
                    (right, down, down_right),
                    (down, left, down_left),
                ] {
                    match (x, y) {
                        (Some(x), Some(y)) if self.grid[x] == value && self.grid[y] == value => {
                            if !visited[x] {
                                if x == 0 {
                                    println!("why");
                                }
                                visited[x] = true;
                                stack.push(x);
                            }

                            // interior vertex
                            if let Some(diag) = diag {
                                if self.grid[diag] != value {
                                    region.vertices += 1;
                                    interior[diag] = true;
                                }
                            }
                        }

                        (Some(x), Some(y)) if self.grid[x] != value && self.grid[y] != value => {
                            region.vertices += 1;
                        }

                        (Some(x), None) | (None, Some(x)) if self.grid[x] != value => {
                            region.vertices += 1;
                        }

                        (Some(x), _) if self.grid[x] == value => {
                            if !visited[x] {
                                visited[x] = true;
                                if x == 0 {
                                    println!("why");
                                }
                                stack.push(x);
                            }
                        }

                        (None, Some(_)) | (Some(_), _) => {
                            // y gets pushed onto the stack when it "comes around"
                        }

                        // edge of the entire grid
                        (None, None) => {
                            region.vertices += 1;
                        }
                    }
                }
            }

            total += region.cost();
            stack.clear();
        }

        total
    }
}

struct Region {
    area: usize,
    vertices: usize,
}

impl Region {
    fn cost(&self) -> usize {
        self.area * self.vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = r"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        "
        .trim()
        .replace(' ', "");

        let result = process(test.as_bytes());

        assert_eq!(result, 436);
    }
}

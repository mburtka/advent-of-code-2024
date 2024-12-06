use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No input file path provided");
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut pos = None;
    let mut first_newline = None;
    let mut grid = Vec::new();
    let mut len = 0;

    for byte in reader.bytes().map(|byte| byte.expect("Cannot read byte")) {
        let obstacle = match (Dir::try_from(byte), byte) {
            (Some(dir), _) => {
                if let Some(_) = pos.replace(Pos { offset: len, dir }) {
                    panic!("Multiple starting positions");
                }
                false
            }

            (_, b'#') => true,

            (_, b'.') => false,

            (_, b'\r') => continue,

            (_, b'\n') => {
                first_newline.get_or_insert(len);
                continue;
            }

            _ => panic!("Invalid character"),
        };
        len += 1;
        grid.push(obstacle);
    }

    let pos = pos.expect("No starting position");
    let cols = first_newline.expect("No newline found");

    let sum = Grid::new(grid, cols, pos).find_loops();
    println!("Sum: {}", sum);
}

struct Grid {
    grid: Vec<bool>,
    cols: usize,
    pos: Pos,
}

impl Grid {
    fn new(grid: Vec<bool>, cols: usize, pos: Pos) -> Self {
        Self { grid, cols, pos }
    }

    fn pos(&self) -> (usize, usize) {
        let y = self.pos.offset / self.cols;
        let x = self.pos.offset % self.cols;
        (x, y)
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.pos.offset = y * self.cols + x;
    }

    fn set_obstacle(&mut self, x: usize, y: usize, value: bool) {
        self.grid[y * self.cols + x] = value;
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.grid.len() / self.cols || x >= self.cols {
            return None;
        }

        Some(self.grid[y * self.cols + x])
    }

    fn peek_next(&self) -> Move {
        let (x, y) = self.pos();

        let (x, y) = match self.pos.dir {
            Dir::Up => (x, if y == 0 { return Move::Done } else { y - 1 }),
            Dir::Down => (x, y + 1),
            Dir::Left => (if x == 0 { return Move::Done } else { x - 1 }, y),
            Dir::Right => (x + 1, y),
        };

        match self.get(x, y) {
            None => Move::Done,
            Some(true) => Move::Turn(self.pos.dir.turn_right()),
            _ => Move::Progress(x, y),
        }
    }

    fn loops(&mut self) -> bool {
        let mut visited = std::collections::HashSet::new();
        visited.insert(self.pos);
        loop {
            match self.peek_next() {
                Move::Done => break false,
                Move::Progress(x, y) => {
                    self.set_pos(x, y);
                    if !visited.insert(self.pos) {
                        break true;
                    }
                }
                Move::Turn(dir) => {
                    self.pos.dir = dir;
                }
            }
        }
    }

    fn find_loops(&mut self) -> usize {
        let mut sum = 0;
        let mut visited = HashSet::new();
        loop {
            match self.peek_next() {
                Move::Done => break sum,
                Move::Progress(x, y) => {
                    let dir = self.pos.dir;
                    if visited.insert((x, y)) {
                        self.set_obstacle(x, y, true);
                        if self.loops() {
                            sum += 1;
                        }
                    }
                    self.set_obstacle(x, y, false);
                    self.pos.dir = dir;
                    self.set_pos(x, y);
                }
                Move::Turn(dir) => {
                    self.pos.dir = dir;
                }
            }
        }
    }
}

enum Move {
    Turn(Dir),
    Progress(usize, usize),
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn try_from(b: u8) -> Option<Self> {
        match b {
            b'^' => Some(Self::Up),
            b'v' => Some(Self::Down),
            b'<' => Some(Self::Left),
            b'>' => Some(Self::Right),
            _ => None,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    offset: usize,
    dir: Dir,
}

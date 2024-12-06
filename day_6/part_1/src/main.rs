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

    let mut grid = Grid::new(grid, cols, pos);
    println!("Sum: {}", grid.solve());
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

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.grid.len() / self.cols || x >= self.cols {
            return None;
        }

        Some(self.grid[y * self.cols + x])
    }

    fn move_next(&mut self) -> Move {
        let (x, y) = self.pos();

        let (x, y) = match self.pos.dir {
            Dir::Up => (x, if y == 0 { return Move::Done } else { y - 1 }),
            Dir::Down => (x, y + 1),
            Dir::Left => (if x == 0 { return Move::Done } else { x - 1 }, y),
            Dir::Right => (x + 1, y),
        };

        match self.get(x, y) {
            None => Move::Done,
            Some(true) => {
                self.pos.dir = self.pos.dir.turn_right();
                Move::Turn
            }
            _ => {
                self.set_pos(x, y);
                Move::Progress
            }
        }
    }

    fn solve(&mut self) -> usize {
        let mut visited = vec![0; self.grid.len()];
        visited[self.pos.offset] = 1;
        loop {
            match self.move_next() {
                Move::Done => break visited.iter().sum(),
                Move::Progress => {
                    visited[self.pos.offset] = 1;
                }
                Move::Turn => {}
            }
        }
    }
}

enum Move {
    Turn,
    Progress,
    Done,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct Pos {
    offset: usize,
    dir: Dir,
}

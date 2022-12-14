use std::cmp;
use std::fs;

#[derive(PartialEq)]
enum Block {
    Wall,
    Sand { rest: bool },
    Air,
}

#[derive(PartialEq, Debug)]
enum State {
    Moving,
    Resting,
    OutOfBounds,
}

#[derive(PartialEq, Clone, Copy)]
struct Position(i32, i32);

struct Board {
    vec: Vec<Block>,
    width: usize,
    height: usize,
    sands: Vec<Position>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut vec = Vec::new();

        for _ in 0..(width * height) {
            vec.push(Block::Air);
        }

        Self {
            vec,
            width,
            height,
            sands: Vec::new(),
        }
    }

    fn get_block(&self, position: &Position) -> Option<&Block> {
        let Position(x, y) = *position;

        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let i = self.width * y as usize;
        Some(&self.vec[i + x as usize])
    }

    fn set_block(&mut self, position: &Position, new_block: Block) {
        let Position(x, y) = position;

        let current_block = self.get_block(position).unwrap();

        if *current_block == (Block::Sand { rest: false }) && new_block == Block::Air {
            let position = self.sands.iter().position(|p| p == position).unwrap();

            self.sands.remove(position);
        } else if *current_block == Block::Air && new_block == (Block::Sand { rest: false }) {
            self.sands.push(Position(*x, *y));
        }

        let i = self.width * *y as usize;
        self.vec[i + *x as usize] = new_block;
    }

    pub fn add_line(&mut self, start: Position, end: Position) {
        let Position(sx, sy) = start;
        let Position(ex, ey) = end;

        if sx == ex {
            for y in cmp::min(sy, ey)..cmp::max(sy, ey) + 1 {
                let i = self.width * y as usize;
                self.vec[i + sx as usize] = Block::Wall;
            }
        } else {
            for x in cmp::min(ex, sx)..cmp::max(ex, sx) + 1 {
                let i = self.width * ey as usize;
                self.vec[i + x as usize] = Block::Wall;
            }
        }
    }

    // pub fn draw(&self) {
    //     for row in 0..self.height {
    //         for col in 0..self.width {
    //             if let Some(block) = self.get_block(&Position(col as i32, row as i32)) {
    //                 match block {
    //                     Block::Wall => print!("#"),
    //                     Block::Sand { rest: false } => print!("O"),
    //                     Block::Sand { rest: true } => print!("o"),
    //                     Block::Air => print!("."),
    //                 }
    //             }
    //         }
    //         println!("");
    //     }
    // }

    pub fn add_sand(&mut self, position: &Position) {
        self.set_block(&position, Block::Sand { rest: false });
    }

    fn is_static(&self, block: &Block) -> bool {
        match block {
            Block::Wall => true,
            Block::Sand { rest: true } => true,
            _ => false,
        }
    }

    pub fn tick_sand(&mut self, initial_position: &Position) -> State {
        let mut position = initial_position.clone();

        self.set_block(&position, Block::Air);

        if let Some(block_below) = self.get_block(&Position(position.0, position.1 + 1)) {
            if !self.is_static(&block_below) {
                position.1 += 1;
            } else {
                let block_left = self
                    .get_block(&Position(position.0 - 1, position.1 + 1))
                    .unwrap();
                let block_right = self
                    .get_block(&Position(position.0 + 1, position.1 + 1))
                    .unwrap();

                if !self.is_static(&block_left) {
                    position.0 -= 1;
                    position.1 += 1;
                } else if !self.is_static(&block_right) {
                    position.0 += 1;
                    position.1 += 1;
                } else {
                    self.set_block(&position, Block::Sand { rest: true });

                    if position == Position(500, 0) {
                        return State::OutOfBounds;
                    }

                    return State::Resting;
                }
            }

            self.set_block(&position, Block::Sand { rest: false });

            return State::Moving;
        } else {
            self.set_block(&position, Block::Sand { rest: true });

            return State::Resting;
        }
    }

    pub fn tick(&mut self) -> State {
        let mut all_state = State::Resting;

        for position in self.sands.clone().iter() {
            let state = self.tick_sand(&position);

            if state == State::OutOfBounds {
                return State::OutOfBounds;
            }

            if state == State::Moving {
                all_state = State::Moving;
            }
        }

        all_state
    }
}

fn parse_position(str: &str) -> Position {
    let mut split = str.split(",");
    let x = split.next().unwrap().parse::<i32>().unwrap();
    let y = split.next().unwrap().parse::<i32>().unwrap();

    Position(x, y)
}

pub fn main() {
    let input = fs::read_to_string("src/14/input.txt").expect("File not found");

    let mut board = Board::new(10000, 10000);

    input.lines().for_each(|line| {
        for slice in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            let start = parse_position(slice[0]);
            let end = parse_position(slice[1]);

            board.add_line(start, end);
        }
    });

    let floor_at = board
        .vec
        .iter()
        .enumerate()
        .filter(|(_, block)| **block == Block::Wall)
        .map(|(i, _)| i / board.width)
        .max()
        .unwrap()
        + 2;

    board.add_line(Position(0, floor_at as i32), Position(999, floor_at as i32));

    let sand_position = parse_position("500,0");
    board.add_sand(&sand_position);

    let mut sands = 0;
    loop {
        let state = board.tick();
        if state == State::Resting {
            board.add_sand(&sand_position);
            sands += 1;
        }

        if state == State::OutOfBounds {
            break;
        }
    }

    // Last tick is always resting
    sands += 1;

    println!("Result b: {}", sands);
}

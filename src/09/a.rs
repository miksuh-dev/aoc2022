use std::fs;

struct Board {
    vec: Vec<Cell>,
    // row: usize,
    col: usize,
    pub tail: Pointer,
    pub head: Pointer,
}

struct Cell {
    // x: usize,
    // y: usize,
    visited: bool,
}

#[derive(Debug)]
struct Pointer {
    x: usize,
    y: usize,
    prev: Option<Box<Pointer>>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    U,
    UR,
    R,
    RD,
    D,
    DL,
    L,
    LU,
}

impl Pointer {
    fn new(x: usize, y: usize) -> Pointer {
        Pointer { x, y, prev: None }
    }

    pub fn move_to(&mut self, dir: Direction) -> &mut Pointer {
        self.prev = Some(Box::new(Pointer::new(self.x, self.y)));

        match dir {
            Direction::U => self.y -= 1,
            Direction::D => self.y += 1,
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
            Direction::UR => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::RD => {
                self.x += 1;
                self.y += 1;
            }
            Direction::DL => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::LU => {
                self.x -= 1;
                self.y -= 1;
            }
        }

        self
    }
}

impl Board {
    pub fn new(initial_vec: Vec<i32>, row: usize, col: usize) -> Self {
        assert!(initial_vec.len() == row * col);

        let mut vec = Vec::new();

        let start_x = row / 2;
        let start_y = col / 2;

        for (index, _) in initial_vec.iter().enumerate() {
            let x = index % col;
            let y = index / col;

            vec.push(Cell {
                // x,
                // y,
                visited: x == start_x && y == start_y,
            });
        }

        let tail = Pointer::new(start_x, start_y - 1);
        let head = Pointer::new(start_x, start_y - 1);

        Self {
            vec,
            // row,
            col,
            tail,
            head,
        }
    }

    // pub fn print(&self) {
    //     for (index, cell) in self.vec.iter().enumerate() {
    //         if index % self.col == 0 {
    //             println!();
    //         }
    //
    //         if (cell.x, cell.y) == (self.tail.x, self.tail.y) {
    //             print!("T");
    //         } else if (cell.x, cell.y) == (self.head.x, self.head.y) {
    //             print!("H");
    //         } else if cell.visited {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    fn get_distance(&self, p1: &Pointer, p2: &Pointer) -> usize {
        let x = (p1.x as i32 - p2.x as i32).abs() as usize;
        let y = (p1.y as i32 - p2.y as i32).abs() as usize;

        if x == 1 && y == 1 {
            1
        } else {
            x + y
        }
    }

    fn is_in_diagonal(&self, p1: &Pointer, p2: &Pointer) -> bool {
        let x = (p1.x as i32 - p2.x as i32).abs() as usize;
        let y = (p1.y as i32 - p2.y as i32).abs() as usize;

        x == 0 || y == 0
    }

    fn get_diagonal_move(&self, p1: &Pointer, p2: &Pointer) -> Option<Direction> {
        let x = (p1.x as i32 - p2.x as i32).abs() as usize;
        let y = (p1.y as i32 - p2.y as i32).abs() as usize;

        if x == 0 && y == 0 {
            return None;
        }

        if x == 0 {
            if p1.y < p2.y {
                return Some(Direction::D);
            } else {
                return Some(Direction::U);
            }
        }

        if y == 0 {
            if p1.x > p2.x {
                return Some(Direction::L);
            } else {
                return Some(Direction::R);
            }
        }

        None
    }

    fn get_direction_to_position(&self, p1: &Pointer, p2: &Pointer) -> Option<Direction> {
        if p1.x < p2.x {
            if p1.y < p2.y {
                Some(Direction::RD)
            } else if p1.y > p2.y {
                Some(Direction::UR)
            } else {
                Some(Direction::R)
            }
        } else if p1.x > p2.x {
            if p1.y < p2.y {
                Some(Direction::DL)
            } else if p1.y > p2.y {
                Some(Direction::LU)
            } else {
                Some(Direction::L)
            }
        } else {
            if p1.y < p2.y {
                Some(Direction::D)
            } else if p1.y > p2.y {
                Some(Direction::U)
            } else {
                None
            }
        }
    }

    fn is_touching_sides(&self, p1: &Pointer, p2: &Pointer) -> bool {
        let x = (p1.x as i32 - p2.x as i32).abs() as usize;
        let y = (p1.y as i32 - p2.y as i32).abs() as usize;

        x <= 1 && y <= 1 && x != y
    }

    pub fn move_tail(&mut self) {
        let distance = self.get_distance(&self.tail, &self.head);
        let touching = self.is_touching_sides(&self.tail, &self.head);

        if distance < 1 || touching {
            return;
        }

        let previous_head = self.head.prev.as_ref().unwrap();

        if self.is_in_diagonal(&self.tail, &self.head) {
            let direction = self.get_diagonal_move(&self.tail, &self.head).unwrap();

            self.tail.move_to(direction);
        } else {
            if distance == 1 {
                return;
            }

            let direction = self.get_direction_to_position(&self.tail, &previous_head);

            if let Some(dir) = direction {
                self.tail.move_to(dir);
            }
        }

        self.vec[self.tail.y * self.col + self.tail.x].visited = true;
    }

    pub fn count_visited(&self) -> usize {
        self.vec.iter().filter(|cell| cell.visited).count()
    }
}

pub fn main() {
    let input = fs::read_to_string("src/09/input.txt").expect("File not found");

    let moves = input
        .lines()
        .map(|line| {
            let (direction_raw, count) = line.split_once(" ").unwrap();

            let direction = match direction_raw {
                "R" => Direction::R,
                "U" => Direction::U,
                "L" => Direction::L,
                "D" => Direction::D,
                _ => panic!("Unknown input direction"),
            };

            (direction, count.parse::<usize>().expect("Not a number"))
        })
        .collect::<Vec<(Direction, usize)>>();

    let mut board = Board::new(vec![0; 1000000], 1000, 1000);

    moves.iter().for_each(|(direction, count)| {
        for _ in 0..*count {
            board.head.move_to(*direction);
            board.move_tail();
        }
    });

    let result = board.count_visited();
    println!("Result a: {}", result);
}

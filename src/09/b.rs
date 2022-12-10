use std::fs;

struct Board {
    vec: Vec<Cell>,
    // row: usize,
    col: usize,
    knots: Vec<Pointer>,
    head: Pointer,
}

struct Cell {
    // x: usize,
    // y: usize,
    visited: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Pointer {
    x: usize,
    y: usize,
    prev: Option<Box<Pointer>>,
    name: char,
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
    fn new(x: usize, y: usize, name: char) -> Pointer {
        Pointer {
            x,
            y,
            prev: None,
            name,
        }
    }

    pub fn move_to(&mut self, dir: Direction) -> &mut Pointer {
        self.prev = Some(Box::new(Pointer::new(self.x, self.y, self.name)));

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

        let start_x = col / 2;
        let start_y = row / 2;

        for (index, _) in initial_vec.iter().enumerate() {
            let x = index % col;
            let y = index / col;

            vec.push(Cell {
                // x,
                // y,
                visited: x == start_x && y == start_y,
            });
        }

        let head = Pointer::new(start_x, start_y, 'H');

        let mut knots = vec![];
        for i in 0..9 {
            knots.push(Pointer::new(start_x, start_y, i.into()));
        }

        Self {
            vec,
            // row,
            col,
            knots,
            head,
        }
    }

    // pub fn print(&self) {
    //     for (index, cell) in self.vec.iter().enumerate() {
    //         if index % self.col == 0 {
    //             println!();
    //         }
    //
    //         let rope_item = self.knots.iter().find(|k| k.x == cell.x && k.y == cell.y);
    //
    //         if let Some(_) = rope_item {
    //             print!("x");
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

        if x != 0 {
            if p1.x > p2.x {
                return Some(Direction::L);
            } else {
                return Some(Direction::R);
            }
        }

        if y != 0 {
            if p1.y < p2.y {
                return Some(Direction::D);
            } else {
                return Some(Direction::U);
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

    fn move_knots(&mut self) {
        let mut new_knots = vec![];

        let last_index = self.knots.len() - 1;

        for (index, knot) in self.knots.iter().enumerate() {
            let target = new_knots.iter().last().unwrap_or(&self.head);

            let knot_c = if let Some(direction) = self.get_pointer_move_direction(&knot, target) {
                let mut knot_c = knot.clone();
                knot_c.move_to(direction);

                if index == last_index {
                    self.vec[knot_c.y * self.col + knot_c.x].visited = true;
                }

                knot_c
            } else {
                knot.clone()
            };

            new_knots.push(knot_c);
        }

        self.knots = new_knots
    }

    fn get_pointer_move_direction(&self, knot: &Pointer, target: &Pointer) -> Option<Direction> {
        let distance = self.get_distance(&knot, &target);
        let touching = self.is_touching_sides(&knot, &target);

        if distance < 1 || touching {
            return None;
        }

        if self.is_in_diagonal(&knot, &target) {
            return self.get_diagonal_move(&knot, &target);
        } else if distance > 1 {
            return self.get_direction_to_position(&knot, &target);
        }

        return None;
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
            board.move_knots();
        }
    });

    let result = board.count_visited();
    println!("Result b: {}", result);
}

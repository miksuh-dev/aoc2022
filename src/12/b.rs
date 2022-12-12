use pathfinding::prelude::dijkstra;
use std::fs;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum PointType {
    End,
    Cell(i32),
}

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq)]
struct Point {
    position: (i32, i32),
    t: PointType,
}

struct Board {
    vec: Vec<Point>,
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(x: i32, y: i32, t: PointType) -> Point {
        Self {
            position: (x, y),
            t,
        }
    }

    fn get_char_height(char: &char) -> Option<usize> {
        let chars = "abcdefghijklmnopqrstuvwxyz";

        chars.chars().position(|c| c == *char)
    }
}

impl Board {
    pub fn new(initial_vec: Vec<char>, row: usize, col: usize) -> Self {
        assert!(initial_vec.len() == row * col);

        let mut vec = Vec::new();

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % col;
            let y = index / col;

            let t = match value {
                'E' => PointType::End,
                value => {
                    if value == &'S' {
                        PointType::Cell(0)
                    } else {
                        PointType::Cell(Point::get_char_height(value).unwrap() as i32)
                    }
                }
            };

            let point = Point::new(x as i32, y as i32, t);
            vec.push(point);
        }

        Self { vec, row, col }
    }

    fn get_starts(&self) -> Vec<&Point> {
        self.vec
            .iter()
            .filter(|point| point.t == PointType::Cell(0))
            .collect::<Vec<&Point>>()
    }

    fn get_end(&self) -> &Point {
        self.vec
            .iter()
            .find(|p| p.t == PointType::End)
            .expect("End not found")
    }

    fn index(&self, row: usize, col: usize) -> Option<&Point> {
        if (row >= self.row) || (col >= self.col) {
            return None;
        }

        let i = self.col * row;

        Some(&self.vec[i + col])
    }

    pub fn get_allowed_moves(&self, point: &(i32, i32)) -> Vec<((i32, i32), isize)> {
        let x = point.0;
        let y = point.1;

        let up = self.index((y - 1) as usize, x as usize);
        let down = self.index((y + 1) as usize, x as usize);
        let left = self.index(y as usize, (x - 1) as usize);
        let right = self.index(y as usize, (x + 1) as usize);

        let points = vec![up, down, left, right]
            .into_iter()
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .collect::<Vec<&Point>>();

        let possible_moves = points
            .to_vec()
            .into_iter()
            .filter(|p| {
                let point = self.vec.iter().find(|p2| p2.position == *point).unwrap();

                let current_height = match point.t {
                    PointType::Cell(height) => height,
                    PointType::End => Point::get_char_height(&'z').unwrap() as i32,
                };

                let next_height = match p.t {
                    PointType::Cell(height) => height,
                    PointType::End => Point::get_char_height(&'z').unwrap() as i32,
                };

                if next_height - current_height > 1 {
                    return false;
                }

                return true;
            })
            .map(|p| ((p.position.0, p.position.1), 1))
            .collect::<Vec<((i32, i32), isize)>>();

        possible_moves
    }
}

pub fn main() {
    let input = fs::read_to_string("src/12/input.txt").expect("File not found");

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let cells = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let board = Board::new(cells, rows, cols);

    let end = board.get_end();

    // :D
    let result = board
        .get_starts()
        .iter()
        .filter_map(|p| {
            let path = dijkstra(
                &p.position,
                |position| board.get_allowed_moves(position),
                |position| position.0 == end.position.0 && position.1 == end.position.1,
            );

            if path.is_some() {
                return Some(path.unwrap().1);
            }

            None
        })
        .min()
        .unwrap();

    println!("Result b: {:?}", result);
}

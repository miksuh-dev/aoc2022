use pathfinding::prelude::bfs;
use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum PointType {
    Start,
    End,
    Cell(i32),
}

struct Point {
    x: usize,
    y: usize,
    t: PointType,
}

struct Board {
    vec: Vec<Point>,
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, t: PointType) -> Point {
        Self { x, y, t }
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
                'S' => PointType::Start,
                'E' => PointType::End,
                value => {
                    let height = Point::get_char_height(&value).unwrap();
                    PointType::Cell(height as i32)
                }
            };

            vec.push(Point::new(x, y, t));
        }

        Self { vec, row, col }
    }

    fn get_start(&self) -> &Point {
        self.vec
            .iter()
            .find(|p| p.t == PointType::Start)
            .expect("Start not found")
    }

    fn get_end(&self) -> &Point {
        self.vec
            .iter()
            .find(|p| p.t == PointType::End)
            .expect("End not found")
    }

    fn index(&self, row: usize, col: usize) -> &Point {
        let i = self.col * row;
        &self.vec[i + col]
    }

    fn get_allowed_moves(&self, point: &Point) -> Vec<&Point> {
        let mut moves = Vec::new();
    }

    fn print(&self) {
        self.vec.iter().enumerate().for_each(|index, c| {
            if c.t === 

                println('{}', );

            if (indee % self.rows === 0) {
                println('');

            }

        })

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

    let start = board.get_start();
    let end = board.get_end();

    // println!("Result a: {}", result);
}

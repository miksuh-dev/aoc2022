use std::fs;

pub struct Forest {
    vec: Vec<Tree>,
    row: usize,
    col: usize,
}

pub struct Tree {
    x: usize,
    y: usize,
    value: i32,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Forest {
    pub fn new(initial_vec: Vec<i32>, row: usize, col: usize) -> Self {
        assert!(initial_vec.len() == row * col);

        let mut vec = Forest {
            vec: Vec::new(),
            row,
            col,
        };

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % col;
            let y = index / col;

            vec.vec.push(Tree {
                x,
                y,
                value: *value,
            });
        }

        Self {
            vec: vec.vec,
            row,
            col,
        }
    }

    pub fn index(&self, row: usize, col: usize) -> &Tree {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn is_border(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.col - 1 || y == self.row - 1
    }

    fn get_trees_in_dir(&self, tree: &Tree, dir: Direction) -> Vec<&Tree> {
        let mut trees = Vec::new();

        if dir == Direction::Up {
            for y in (0..tree.y).rev() {
                trees.push(self.index(y, tree.x));
            }
        }

        if dir == Direction::Right {
            for x in (tree.x + 1)..self.col {
                trees.push(self.index(tree.y, x));
            }
        }

        if dir == Direction::Down {
            for y in (tree.y + 1)..self.row {
                trees.push(self.index(y, tree.x));
            }
        }

        if dir == Direction::Left {
            for x in (0..tree.x).rev() {
                trees.push(self.index(tree.y, x));
            }
        }

        trees
    }

    pub fn is_visible(&self, tree: &Tree) -> bool {
        if self.is_border(tree.x, tree.y) {
            return true;
        }

        let dirs = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        for dir in dirs {
            let trees = self.get_trees_in_dir(tree, dir);

            let visible = trees.iter().filter(|t| t.value < tree.value).count() == trees.len();

            if visible {
                return true;
            }
        }

        return false;
    }
}

pub fn main() {
    let input = fs::read_to_string("src/08/input.txt").expect("File not found");

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let cells = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect::<Vec<i32>>();

    let forest = Forest::new(cells, rows, cols);

    let result = forest
        .vec
        .iter()
        .filter(|tree| forest.is_visible(&tree))
        .count();

    println!("Result a: {}", result);
}

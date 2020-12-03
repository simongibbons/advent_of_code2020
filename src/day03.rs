#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Square {
    EMPTY,
    TREE,
}

pub struct Map {
    squares: Vec<Vec<Square>>
}

impl Map {
    pub fn get_square(&self, x: usize, y: usize) -> Option<Square> {
        let xrow = self.squares.get(y)?;
        let xfolded = x % xrow.len();
        Some(xrow[xfolded])
    }
}

#[aoc_generator(day3)]
pub fn parse_map(input: &str) -> Map {
    let squares = input.split("\n")
        .map(|l| parse_line(l))
        .collect();
    Map { squares }
}

fn parse_line(line: &str) -> Vec<Square> {
    let parse_square = |c| match c {
        '.' => Square::EMPTY,
        '#' => Square::TREE,
        _ => panic!("Unknown symbol")
    };

    line.chars().map(parse_square).collect()
}

fn tree_count(map: &Map, dx: usize, dy: usize) -> usize{
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    loop {
        x += dx;
        y += dy;

        let square = map.get_square(x, y);
        if square.is_none() {
            break;
        }

        if square.unwrap() == Square::TREE {
            count += 1;
        }
    }
    count
}

#[aoc(day3, part1)]
pub fn part1(map: &Map) -> usize {
    tree_count(map, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(map: &Map) -> usize {
    let to_try = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    to_try.iter()
        .map(|(dx, dy)| tree_count(map, *dx, *dy))
        .product()
}

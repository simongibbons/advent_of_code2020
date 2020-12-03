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

    pub fn get_squares(&self, dx: usize, dy: usize) -> Box<dyn Iterator<Item=Square> + '_> {
        let xs = (0..).step_by(dx);
        let ys = (0..).step_by(dy);

        let it = xs.zip(ys)
            .map(move |(x, y)| self.get_square(x, y))
            .take_while(|o| o.is_some())
            .map(|o| o.unwrap());

        Box::new(it)
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

fn tree_count_iterator(map: &Map, dx: usize, dy: usize) -> usize {
    map.get_squares(dx, dy)
        .filter(|&s| s == Square::TREE)
        .count()
}

fn tree_count_loop(map: &Map, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    loop {
        let maybe_square = map.get_square(x, y);
        if maybe_square.is_none() {
            break;
        }

        if maybe_square.unwrap() == Square::TREE {
            count += 1;
        }

        x += dx;
        y += dy;
    }
    count
}

#[aoc(day3, part1, Iterator)]
pub fn part1_iterator(map: &Map) -> usize {
    tree_count_iterator(map, 3, 1)
}

#[aoc(day3, part1, Loop)]
pub fn part1_loop(map: &Map) -> usize {
    tree_count_loop(map, 3, 1)
}

const SLOPES: [(usize, usize); 5] = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

#[aoc(day3, part2, Iterator)]
pub fn part2_iterator(map: &Map) -> usize {
    SLOPES.iter()
        .map(|(dx, dy)| tree_count_iterator(map, *dx, *dy))
        .product()
}

#[aoc(day3, part2, Loop)]
pub fn part2_loop(map: &Map) -> usize {
    SLOPES.iter()
        .map(|(dx, dy)| tree_count_loop(map, *dx, *dy))
        .product()
}

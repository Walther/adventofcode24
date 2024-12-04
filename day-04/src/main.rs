use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Maze;

fn parse(input: &str) -> ParsedData {
    let mut maze = HashMap::new();
    for (row, contents) in input.lines().enumerate() {
        for (column, character) in contents.chars().enumerate() {
            maze.insert((row, column), character);
        }
    }

    Maze { maze }
}

struct Maze {
    maze: HashMap<(usize, usize), char>,
}

impl Maze {
    fn find_all(&self, search: char) -> Vec<(&usize, &usize)> {
        self.maze
            .iter()
            .filter(|&((_, _), character)| character == &search)
            .map(|((x, y), _)| (x, y))
            .collect()
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.maze.get(&(x, y))
    }
}

#[derive(Clone, Copy)]
enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}
use Direction::{E, N, NE, NW, S, SE, SW, W};

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [NW, N, NE, W, E, SW, S, SE].iter().copied()
    }
}

struct Visitor<'a> {
    maze: &'a Maze,
    x: usize,
    y: usize,
}

impl<'a> Visitor<'a> {
    fn new(maze: &'a Maze, x: usize, y: usize) -> Self {
        Self { maze, x, y }
    }

    fn get(&self) -> Option<&char> {
        self.maze.get(self.x, self.y)
    }

    fn coordinate_in_direction(&self, direction: Direction) -> Option<(usize, usize)> {
        let x;
        let y;
        match direction {
            Direction::NW => {
                x = self.x.checked_sub(1)?;
                y = self.y.checked_sub(1)?;
            }
            Direction::N => {
                x = self.x;
                y = self.y.checked_sub(1)?;
            }
            Direction::NE => {
                x = self.x.checked_add(1)?;
                y = self.y.checked_sub(1)?;
            }
            Direction::W => {
                x = self.x.checked_sub(1)?;
                y = self.y;
            }
            Direction::E => {
                x = self.x.checked_add(1)?;
                y = self.y;
            }
            Direction::SW => {
                x = self.x.checked_sub(1)?;
                y = self.y.checked_add(1)?;
            }
            Direction::S => {
                x = self.x;
                y = self.y.checked_add(1)?;
            }
            Direction::SE => {
                x = self.x.checked_add(1)?;
                y = self.y.checked_add(1)?;
            }
        }

        Some((x, y))
    }

    fn peek(&self, direction: Direction) -> Option<&char> {
        let (x, y) = self.coordinate_in_direction(direction)?;
        self.maze.get(x, y)
    }

    fn step(&mut self, direction: Direction) -> Option<&char> {
        let (x, y) = self.coordinate_in_direction(direction)?;
        self.x = x;
        self.y = y;
        self.get()
    }

    fn collect(&mut self, max_length: usize, direction: Direction) -> Option<String> {
        let mut collection = String::new();
        let grab = self.get()?;
        collection.push(*grab);

        while collection.len() < max_length {
            match self.step(direction) {
                Some(c) => collection.push(*c),
                None => return Some(collection),
            }
        }

        Some(collection)
    }

    fn surroundings(&self) -> Option<[&char; 9]> {
        Some([
            self.peek(NW)?,
            self.peek(N)?,
            self.peek(NE)?,
            self.peek(W)?,
            self.get()?,
            self.peek(E)?,
            self.peek(SW)?,
            self.peek(S)?,
            self.peek(SE)?,
        ])
    }
}

fn part1(data: &ParsedData) -> usize {
    let mut xmas_count = 0;
    let letter_x_coordinates = data.find_all('X');
    for (&x, &y) in letter_x_coordinates {
        for direction in Direction::iter() {
            let mut visitor = Visitor::new(data, x, y);
            let collection = visitor.collect(4, direction);
            match collection {
                Some(string) => {
                    if string == *"XMAS" {
                        xmas_count += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    xmas_count
}

fn part2(data: &ParsedData) -> usize {
    let mut x_max_count = 0;
    for &(x, y) in data.maze.keys() {
        let visitor = Visitor::new(data, x, y);
        let surroundings = visitor.surroundings();
        match surroundings {
            Some(map) => {
                if is_x_mas(map) {
                    x_max_count += 1;
                }
            }
            _ => continue,
        }
    }

    x_max_count
}

#[rustfmt::skip]
fn is_x_mas(map: [&char; 9]) -> bool {
    matches!(map,
        [
            'M', _, 'M',
            _,  'A',  _,
            'S', _, 'S',
        ] | [
            'S', _, 'M',
            _,  'A',  _,
            'S', _, 'M',
        ] | [
            'M', _, 'S',
            _,  'A',  _,
            'M', _, 'S',
        ] | [
            'S', _, 'S',
            _,  'A',  _,
            'M', _, 'M',
        ])
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 18;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 9;
        assert_eq!(value, expected);
    }
}

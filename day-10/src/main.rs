use std::collections::HashSet;

use shared::maze::{
    Coordinate,
    Direction::{E, N, S, W},
    Maze, Visitor, VisitorOptions,
};

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
    input.parse().expect("Unable to parse maze")
}

fn climb(maze: &Maze, coordinate: Coordinate, summits: &mut HashSet<Coordinate>) {
    let visitor = Visitor::new(VisitorOptions::default(), maze, coordinate);
    let height: u32 = visitor
        .get()
        .expect("Visitor outside Maze")
        .to_digit(10)
        .expect("Unknown height");
    if height == 9 {
        summits.insert(visitor.position());
    }
    for direction in [N, W, E, S] {
        if let Some(h) = visitor.peek(direction) {
            if h.to_digit(10) == Some(height + 1) {
                let next = visitor
                    .coordinate_in_direction(direction)
                    .expect("Unable to get coordinate after peeking");
                climb(maze, next, summits);
            }
        }
    }
}

fn part1(data: &ParsedData) -> usize {
    let maze = data;
    let trailheads = data.find_all('0');
    let mut score = 0;
    for coordinate in trailheads {
        let mut summits: HashSet<Coordinate> = HashSet::new();
        climb(maze, coordinate, &mut summits);
        score += summits.len();
    }

    score
}

fn part2(_data: &ParsedData) -> usize {
    2
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 36;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 2;
        assert_eq!(value, expected);
    }
}

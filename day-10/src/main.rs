use std::collections::HashSet;

use itertools::Itertools;

use shared::{
    Coordinate,
    Direction::{E, N, S, W},
    Maze, Visitor,
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

fn climb(visitor: Visitor, summits: &mut HashSet<Coordinate>) -> Vec<Visitor> {
    let height: u32 = visitor
        .get()
        .expect("Visitor outside Maze")
        .to_digit(10)
        .expect("Unknown height");
    if height == 9 {
        summits.insert(visitor.position());
        return vec![visitor];
    }
    let mut climbers = Vec::new();
    for direction in [N, W, E, S] {
        if let Some(h) = visitor.peek(direction) {
            if h.to_digit(10) == Some(height + 1) {
                let mut clone = visitor.clone();
                clone.step(direction).expect("Unable to step after peek");
                let mut recurse = climb(clone, summits);
                climbers.append(&mut recurse);
            }
        }
    }
    climbers
}

fn part1(data: &ParsedData) -> usize {
    let maze = data.clone();
    let trailheads = maze.find_all('0');
    let mut score = 0;
    let maze = maze.make_shareable();
    for coordinate in trailheads {
        let mut summits: HashSet<Coordinate> = HashSet::new();
        let visitor = Visitor::new(&maze.clone(), coordinate);
        let _ = climb(visitor, &mut summits);
        score += summits.len();
    }

    score
}

fn part2(data: &ParsedData) -> usize {
    let maze = data.clone();
    let trailheads = maze.find_all('0');
    let mut paths = 0;
    let maze = maze.make_shareable();
    for coordinate in trailheads {
        let mut summits: HashSet<Coordinate> = HashSet::new();
        let visitor = Visitor::new(&maze.clone(), coordinate);
        let climbers = climb(visitor, &mut summits);
        paths += climbers.iter().map(Visitor::path).unique().count();
    }

    paths
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
        let expected = 81;
        assert_eq!(value, expected);
    }
}

use std::collections::HashSet;

use itertools::Itertools;
use shared::{Coordinate, Displacement, Maze};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type Frequencies = Vec<char>;

type ParsedData = (Maze, Frequencies);

fn parse(input: &str) -> ParsedData {
    let maze: Maze = input.parse().expect("Unable to parse input maze");
    let frequencies: Vec<char> = maze
        .all_values()
        .iter()
        .filter(|&c| *c != '.')
        .unique()
        .copied()
        .collect();

    (maze, frequencies)
}

fn part1(data: &ParsedData) -> usize {
    let (maze, frequencies) = data;
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for &frequency in frequencies {
        let antennae = maze.find_all(frequency);
        for (a, b) in antennae.iter().tuple_combinations() {
            let delta: Displacement = b - a;
            let antinode_a = a - delta;
            if maze.contains_coordinate(antinode_a) {
                antinodes.insert(antinode_a);
            }
            let antinode_b = b + delta;
            if maze.contains_coordinate(antinode_b) {
                antinodes.insert(antinode_b);
            }
        }
    }

    antinodes.len()
}

// FIXME: ugly repetition.
fn part2(data: &ParsedData) -> usize {
    let (maze, frequencies) = data;
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for &frequency in frequencies {
        let antennae = maze.find_all(frequency);
        for (&a, &b) in antennae.iter().tuple_combinations() {
            let delta: Displacement = b - a;
            let mut antinode_a = a;
            loop {
                if maze.contains_coordinate(antinode_a) {
                    antinodes.insert(antinode_a);
                    antinode_a -= delta;
                    continue;
                }
                break;
            }
            let mut antinode_b = b;
            loop {
                if maze.contains_coordinate(antinode_b) {
                    antinodes.insert(antinode_b);
                    antinode_b += delta;
                    continue;
                }
                break;
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 14;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 34;
        assert_eq!(value, expected);
    }
}

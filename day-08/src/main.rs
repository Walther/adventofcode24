use itertools::Itertools;
use shared::{Coordinate, Displacement, Maze};
use std::collections::HashSet;

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
        antennae.iter().permutations(2).for_each(|permutation| {
            let [a, b] = permutation[..] else {
                panic!("Unable to parse permutation");
            };
            let delta: Displacement = b - a;
            let antinode = a - delta;
            if maze.contains_coordinate(antinode) {
                antinodes.insert(antinode);
            }
        });
    }

    antinodes.len()
}

fn part2(data: &ParsedData) -> usize {
    let (maze, frequencies) = data;
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for &frequency in frequencies {
        let antennae = maze.find_all(frequency);
        antennae.iter().permutations(2).for_each(|permutation| {
            let [&a, &b] = permutation[..] else {
                panic!("Unable to parse permutation");
            };
            let delta: Displacement = b - a;
            let mut antinode = a;
            loop {
                if maze.contains_coordinate(antinode) {
                    antinodes.insert(antinode);
                    antinode -= delta;
                    continue;
                }
                break;
            }
        });
    }

    antinodes.len()
}

#[cfg(test)]
mod integration {
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

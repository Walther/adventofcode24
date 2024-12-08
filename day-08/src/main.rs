use std::collections::HashSet;

use itertools::Itertools;
use shared::Maze;

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

// FIXME: ugly casts. Not in a refactoring mood tonight.
#[allow(clippy::cast_possible_wrap)]
fn part1(data: &ParsedData) -> usize {
    let (maze, frequencies) = data;
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for &frequency in frequencies {
        let antennae = maze.find_all(frequency);
        for (a, b) in antennae.iter().tuple_combinations() {
            let dx: isize = (b.0 as isize) - (a.0 as isize);
            let dy: isize = (b.1 as isize) - (a.1 as isize);
            let antinode_a = (a.0 as isize - dx, a.1 as isize - dy);
            if maze.contains_coordinate_signed(antinode_a.0, antinode_a.1) {
                antinodes.insert(antinode_a);
            }
            let antinode_b = (b.0 as isize + dx, b.1 as isize + dy);
            if maze.contains_coordinate_signed(antinode_b.0, antinode_b.1) {
                antinodes.insert(antinode_b);
            }
        }
    }

    antinodes.len()
}

// FIXME: ugly repetition.
#[allow(clippy::cast_possible_wrap)]
fn part2(data: &ParsedData) -> usize {
    let (maze, frequencies) = data;
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for &frequency in frequencies {
        let antennae = maze.find_all(frequency);
        for (a, b) in antennae.iter().tuple_combinations() {
            let dx: isize = (b.0 as isize) - (a.0 as isize);
            let dy: isize = (b.1 as isize) - (a.1 as isize);
            let mut antinode_a = (a.0 as isize, a.1 as isize);
            loop {
                if maze.contains_coordinate_signed(antinode_a.0, antinode_a.1) {
                    antinodes.insert(antinode_a);
                    antinode_a.0 -= dx;
                    antinode_a.1 -= dy;
                    continue;
                }
                break;
            }
            let mut antinode_b = (b.0 as isize, b.1 as isize);
            loop {
                if maze.contains_coordinate_signed(antinode_b.0, antinode_b.1) {
                    antinodes.insert(antinode_b);
                    antinode_b.0 += dx;
                    antinode_b.1 += dy;
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

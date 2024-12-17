use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;

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
    input.parse().expect("Unable to parse input")
}

fn part1(data: &ParsedData) -> usize {
    let mut maze = data.clone();
    let start = maze
        .find_replace('^', '.')
        .expect("Unable to find guard in maze");
    let (steps, _has_looped) = guard_walk(maze, start);
    steps
}

fn part2(data: &ParsedData) -> usize {
    let mut maze = data.clone();
    let start = maze
        .find_replace('^', '.')
        .expect("Unable to find guard in maze");
    let style = ProgressStyle::default_bar()
        .template(
            "Elapsed:   {elapsed_precise}\nProgress:  {bar} {pos}/{len}\nRemaining: {eta_precise}",
        )
        .expect("Unable to create progress bar style");

    maze.all_coordinates()
        .into_par_iter()
        .progress_with_style(style)
        .filter(|&coordinate| {
            let mut obstructed_maze = maze.clone();
            obstructed_maze.upsert(coordinate, '#');
            let (_steps, has_looped) = guard_walk(obstructed_maze, start);
            has_looped
        })
        .count()
}

fn guard_walk(maze: Maze, coordinate: Coordinate) -> (usize, bool) {
    let maze = maze.make_shareable();
    let mut guard = Visitor::new(&maze.clone(), coordinate);
    let walk_directions = [N, E, S, W];
    let mut direction_index = 0;
    let mut direction = walk_directions[direction_index];
    let mut has_looped = false;
    while let Some(forward) = guard.peek(direction) {
        if guard.has_looped() {
            has_looped = true;
            break;
        }
        if forward == '#' {
            direction_index = (direction_index + 1) % walk_directions.len();
            direction = walk_directions[direction_index];
            continue;
        }
        guard.step(direction);
    }
    let steps = guard.visited_coordinates().len();
    (steps, has_looped)
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 41;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 6;
        assert_eq!(value, expected);
    }
}

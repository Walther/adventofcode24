use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;

use shared::Direction::{E, N, S, W};
use shared::{Maze, Visitor, VisitorOptions};

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
    let (maze, x, y) = remove_guard_marker(data);
    let (steps, _has_looped) = guard_walk(&maze, x, y);
    steps
}

fn part2(data: &ParsedData) -> usize {
    let (maze, x, y) = remove_guard_marker(data);
    let style = ProgressStyle::default_bar()
        .template(
            "Elapsed:   {elapsed_precise}\nProgress:  {bar} {pos}/{len}\nRemaining: {eta_precise}",
        )
        .expect("Unable to create progress bar style");
    let looped: Vec<bool> = maze
        .all_coordinates()
        .into_par_iter()
        .progress_with_style(style)
        .map(|&(obstruction_x, obstruction_y)| {
            let mut obstructed_maze = maze.clone();
            obstructed_maze.upsert(obstruction_x, obstruction_y, '#');
            let (_steps, has_looped) = guard_walk(&obstructed_maze, x, y);
            has_looped
        })
        .collect();

    looped.into_iter().filter(|&l| l).count()
}

fn remove_guard_marker(data: &Maze) -> (Maze, usize, usize) {
    let mut maze = data.clone();
    let (x, y) = maze.find('^').expect("Unable to find guard in the maze");
    maze.upsert(x, y, '.');
    (maze, x, y)
}

fn guard_walk(maze: &Maze, x: usize, y: usize) -> (usize, bool) {
    let visitor_options = VisitorOptions {
        record_visited: true,
        ..Default::default()
    };
    let mut guard = Visitor::new(visitor_options, maze, x, y);
    let walk_directions = [N, E, S, W];
    let mut direction_index = 0;
    let mut direction = walk_directions[direction_index];
    let mut has_looped = false;
    while let Some(forward) = guard.peek(direction) {
        if guard.deja_vu(direction) {
            has_looped = true;
            break;
        }
        if *forward == '#' {
            direction_index = (direction_index + 1) % walk_directions.len();
            direction = walk_directions[direction_index];
            continue;
        }
        guard.step(direction);
    }
    let steps = guard
        .unique_visited()
        .expect("Guard path not recorded")
        .len();
    (steps, has_looped)
}

#[cfg(test)]
mod tests {
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

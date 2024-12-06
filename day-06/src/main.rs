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
    let mut maze = data.clone();
    let (x, y) = maze.find('^').expect("Unable to find guard in the maze");
    maze.upsert(x, y, '.');
    let visitor_options = VisitorOptions {
        record_visited: true,
        ..Default::default()
    };
    let mut guard = Visitor::new(visitor_options, &maze, x, y);
    let walk_directions = [N, E, S, W];
    let mut direction_index = 0;
    let mut direction = walk_directions[direction_index];
    while let Some(forward) = guard.peek(direction) {
        if *forward == '#' {
            direction_index = (direction_index + 1) % walk_directions.len();
            direction = walk_directions[direction_index];
            continue;
        }
        guard.step(direction);
    }

    guard.visited().expect("Guard path not recorded").len()
}

fn part2(_data: &ParsedData) -> usize {
    2
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
        let expected = 2;
        assert_eq!(value, expected);
    }
}

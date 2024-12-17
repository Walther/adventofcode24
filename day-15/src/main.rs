pub mod robot;

use robot::Robot;

use shared::{Direction, Maze, Visitor};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type Movements = Vec<Direction>;
type ParsedData = (Maze, Movements);

fn parse(input: &str) -> ParsedData {
    let (maze, movements) = input.split_once("\n\n").expect("Unable to split input");
    let maze: Maze = maze.parse().expect("Unable to parse maze");
    let movements = movements
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| Direction::try_from(c).expect("Unable to parse movements"))
        .collect();

    (maze, movements)
}

fn part1(data: &ParsedData) -> usize {
    let mut maze = data.0.clone();
    let movements = data.1.clone();
    let robot_position = maze
        .find_replace('@', '.')
        .expect("Unable to find robot in maze");
    let maze = maze.make_shareable();
    let visitor = Visitor::new(&maze, robot_position);
    let mut robot = Robot::new(visitor);
    for direction in movements {
        robot.r#move(direction);
    }

    robot.gps_sum()
}

fn part2(_data: &ParsedData) -> usize {
    0
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 10092;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 0;
        assert_eq!(value, expected);
    }
}

#[cfg(test)]
mod unit {
    #[test]
    fn unit() {
        let value = 0;
        let expected = 0;
        assert_eq!(value, expected);
    }
}

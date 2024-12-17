pub mod robot;
use robot::{looks_like_a_tree, safety_score, Robot, BATHROOM_HEIGHT, BATHROOM_WIDTH};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<Robot>;

fn parse(input: &str) -> ParsedData {
    input
        .lines()
        .map(|line| Robot::try_from(line).expect("Robot parse error"))
        .collect()
}

fn part1(data: &ParsedData) -> usize {
    let mut robots = data.clone();
    robots.iter_mut().for_each(|r| r.step_n(100));
    safety_score(&robots)
}

fn part2(data: &ParsedData) -> usize {
    let mut robots = data.clone();
    for step in 0..(BATHROOM_WIDTH * BATHROOM_HEIGHT) {
        if looks_like_a_tree(&robots) {
            // robot::print_bathroom(&robots);
            #[allow(clippy::cast_sign_loss)]
            return step as usize;
        };
        robots.iter_mut().for_each(Robot::step);
    }

    0
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 12;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 0; // NOTE: no easter egg in test input
        assert_eq!(value, expected);
    }
}

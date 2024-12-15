use arcade::ClawMachine;

pub mod arcade;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<ClawMachine>;

fn parse(input: &str) -> ParsedData {
    input
        .split("\n\n")
        .map(|machine| machine.parse().expect("Unable to parse machine"))
        .collect()
}

fn part1(data: &ParsedData) -> usize {
    let machines = data;
    let cost: usize = machines
        .iter()
        .filter_map(ClawMachine::minimal_solve_cost)
        .sum();

    cost
}

fn part2(_data: &ParsedData) -> usize {
    2
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 480;
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

use computer::Computer;

pub mod computer;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Computer;

fn parse(input: &str) -> ParsedData {
    input.parse().expect("Unable to parse computer")
}

fn part1(data: &ParsedData) -> String {
    let mut computer = data.clone();
    computer.run(false).expect("Program failed");
    computer.print()
}

fn part2(_data: &ParsedData) -> usize {
    // FIXME: brute force clearly isn't going to work
    /* let mut computer = data.clone();
    let target = computer.print_program();
    let mut initial_a = 0;
    loop {
        computer.set_register('a', initial_a as u64);
        if computer.run(false).is_ok() && computer.print() == target {
            break;
        }
        initial_a += 1;
    }
    initial_a */
    0
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = "4,6,3,5,6,3,5,2,1,0".to_string();
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

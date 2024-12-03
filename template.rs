fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = method_1(&parsed);
    println!("Part 1: {value}");

    let value = method_2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<()>;

fn parse(input: &str) -> ParsedData {}

fn method_1(data: &ParsedData) -> u32 {
    1
}

fn method_2(data: &ParsedData) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = r"";

    #[test]
    fn part1() {
        let parsed = parse(INPUT);
        let value = method_1(&parsed);
        let expected = 1;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = parse(INPUT);
        let value = method_2(&parsed);
        let expected = 2;
        assert_eq!(value, expected);
    }
}
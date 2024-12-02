fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(&input);

    let value = method_1(&parsed);
    println!("Part 1: {value}");

    let value = method_2(&parsed);
    println!("Part 2: {value}");
}

fn parse(input: &str) {}

fn method_1() {}

fn method_2() {}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = r"";

    #[test]
    fn part1() {
        let parsed = parse(INPUT);
        let value = method_1(parsed);
        assert_eq!(value, 1);
    }

    #[test]
    fn part2() {
        let parsed = parse(INPUT);
        let value = method_2(parsed);
        assert_eq!(value, 2);
    }
}

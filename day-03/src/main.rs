use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = method_1(&parsed);
    println!("Part 1: {value}");

    let value = method_2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<(u32, u32)>;

fn parse(input: &str) -> ParsedData {
    let mut muls = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to construct a regular expression");
    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let l: u32 = left.parse().expect("Unable to parse number");
        let r: u32 = right.parse().expect("Unable to parse number");
        muls.push((l, r));
    }

    muls
}

fn method_1(data: &ParsedData) -> u32 {
    data.iter().map(|(l, r)| l * r).sum()
}

fn method_2(data: &ParsedData) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1() {
        let parsed = parse(INPUT);
        let value = method_1(&parsed);
        let expected = 161;
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

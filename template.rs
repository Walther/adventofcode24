fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<()>;

fn parse(_input: &str) -> ParsedData {
    Vec::new()
}

fn part1(_data: &ParsedData) -> usize {
    1
}

fn part2(_data: &ParsedData) -> usize {
    2
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 1;
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

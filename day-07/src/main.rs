use std::collections::VecDeque;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<(usize, Vec<usize>)>;

fn parse(input: &str) -> ParsedData {
    let mut equations = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").expect("Unable to parse line");
        let test_value: usize = left.parse().expect("Unable to parse test value");
        let numbers: Vec<usize> = right
            .split(' ')
            .map(|n| n.parse().expect("Unable to parse number"))
            .collect();
        equations.push((test_value, numbers));
    }

    equations
}

fn part1(data: &ParsedData) -> usize {
    total_calibration(data, false)
}

fn part2(data: &ParsedData) -> usize {
    total_calibration(data, true)
}

fn total_calibration(data: &ParsedData, concats: bool) -> usize {
    let mut total_calibration_result = 0;
    for (test_value, numbers) in data {
        let mut numbers: VecDeque<usize> = numbers.clone().into();
        let mut possible_totals = vec![numbers.pop_front().expect("Missing numbers in equation")];

        while let Some(n) = numbers.pop_front() {
            let sums: Vec<usize> = possible_totals.iter().map(|t| t + n).collect();
            let products: Vec<usize> = possible_totals.iter().map(|t| t * n).collect();
            if concats {
                let concats: Vec<usize> = possible_totals
                    .iter()
                    .map(|t| t * 10_usize.pow(n.ilog10() + 1) + n)
                    .collect();

                possible_totals = [sums, products, concats].concat();
            } else {
                possible_totals = [sums, products].concat();
            }
        }

        if possible_totals.contains(test_value) {
            total_calibration_result += test_value;
        }
    }

    total_calibration_result
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 3749;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 11387;
        assert_eq!(value, expected);
    }
}

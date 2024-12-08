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
    total_calibration(data, &[&operator_sum, &operator_product])
}

fn part2(data: &ParsedData) -> usize {
    total_calibration(data, &[&operator_sum, &operator_product, &operator_concat])
}

fn total_calibration(data: &ParsedData, operators: &[&dyn Fn(usize, usize) -> usize]) -> usize {
    let mut total_calibration_result = 0;
    for (test_value, numbers) in data {
        let mut numbers: VecDeque<usize> = numbers.clone().into();
        let mut possible_totals = vec![numbers.pop_front().expect("Missing numbers in equation")];

        while let Some(n) = numbers.pop_front() {
            let mut new_totals = Vec::new();
            for &operator in operators {
                let mut results: Vec<usize> =
                    possible_totals.iter().map(|&t| operator(t, n)).collect();
                new_totals.append(&mut results);
            }
            possible_totals = new_totals;
        }

        if possible_totals.contains(test_value) {
            total_calibration_result += test_value;
        }
    }

    total_calibration_result
}

fn operator_sum(a: usize, b: usize) -> usize {
    a + b
}

fn operator_product(a: usize, b: usize) -> usize {
    a * b
}

fn operator_concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod tests {
    use crate::operator_concat;

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

    #[test]
    fn concatenation() {
        let a = 123;
        let b = 456;
        let concat = operator_concat(a, b);
        let expected = 123_456;
        assert_eq!(concat, expected);
    }
}

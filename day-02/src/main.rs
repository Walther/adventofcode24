use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type Report = Vec<usize>;
type ParsedData = Vec<Report>;

fn parse(input: &str) -> ParsedData {
    let mut reports = Vec::new();
    for line in input.lines() {
        let mut report = Vec::new();
        for level in line.split(' ') {
            let number: usize = level.parse().expect("Unable to parse level as an integer");
            report.push(number);
        }
        reports.push(report);
    }
    reports
}

fn part1(data: &ParsedData) -> usize {
    data.iter().filter(is_safe_report).count()
}

// TODO: how to make clippy happier while keeping the filter above neat?
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_safe_report(report: &&Report) -> bool {
    let is_strictly_monotone =
        report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b);
    let is_gradual = report
        .iter()
        .tuple_windows::<(_, _)>()
        .all(|(&a, &b)| a.abs_diff(b) <= 3);

    is_strictly_monotone && is_gradual
}

fn part2(data: &ParsedData) -> usize {
    data.iter()
        .filter(|&report| {
            // NOTE: brute force solution
            for i in 0..(report.len()) {
                let mut dampened_report = report.clone();
                dampened_report.remove(i);
                if is_safe_report(&&dampened_report) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        assert_eq!(value, 2);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        assert_eq!(value, 4);
    }
}

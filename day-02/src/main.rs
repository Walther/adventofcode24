use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = count_safe_reports(&parsed);
    println!("Part 1: {value}");

    let value = count_dampenable_reports(&parsed);
    println!("Part 2: {value}");
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let mut report = Vec::new();
        for level in line.split(' ') {
            let number: u32 = level.parse().expect("Unable to parse level as an integer");
            report.push(number);
        }
        reports.push(report);
    }
    reports
}

fn count_safe_reports(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .map(|report| is_safe_report(report))
        .filter(|&bool| bool)
        .count()
}

fn is_safe_report(report: &[u32]) -> bool {
    let is_strictly_monotone =
        report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b);
    let is_gradual = report
        .iter()
        .tuple_windows::<(_, _)>()
        .all(|(&a, &b)| a.abs_diff(b) <= 3);

    is_strictly_monotone && is_gradual
}

fn count_dampenable_reports(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .filter(|&report| {
            // NOTE: brute force solution
            for i in 0..(report.len()) {
                let mut dampened_report = report.clone();
                dampened_report.remove(i);
                if is_safe_report(&dampened_report) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part1() {
        let parsed = parse(INPUT);
        let value = count_safe_reports(&parsed);
        assert_eq!(value, 2);
    }

    #[test]
    fn part2() {
        let parsed = parse(INPUT);
        let value = count_dampenable_reports(&parsed);
        assert_eq!(value, 4);
    }
}

use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let (left, right) = parse(INPUT);

    let value = total_distance(&left, &right);
    println!("Part 1: {value}");

    let value = similarity_score(&left, &right);
    println!("Part 2: {value}");
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (l, r) = line
            .split_once("   ")
            .expect("Unable to split row into two numbers");
        let l: u32 = l.parse().expect("Unable to parse left number");
        let r: u32 = r.parse().expect("Unable to parse right number");
        left.push(l);
        right.push(r);
    }
    // NOTE: load-bearing sort for Part 1
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

#[must_use]
/// Given two lists of numbers, calculate pair-wise absolute differences, and return the sum of those differences.
fn total_distance(left: &[u32], right: &[u32]) -> u32 {
    left.iter().zip(right).map(|(&l, &r)| l.abs_diff(r)).sum()
}

/// For each number in the left list, multiply it by the count of occurrences of itself in the right list, and return the sum of those products.
fn similarity_score(left: &[u32], right: &[u32]) -> u32 {
    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    for &id in right {
        *right_counts.entry(id).or_insert(0) += 1;
    }
    left.iter()
        .map(|l| l * right_counts.get(l).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn part1() {
        let (left, right) = parse(INPUT);
        let value = total_distance(&left, &right);
        assert_eq!(value, 11);
    }

    #[test]
    fn part2() {
        let (left, right) = parse(INPUT);
        let value = similarity_score(&left, &right);
        assert_eq!(value, 31);
    }
}
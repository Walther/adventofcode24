use std::cmp::Ordering;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = PrintJob;

struct PrintJob {
    ordering_rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn parse(input: &str) -> ParsedData {
    let mut ordering_rules = Vec::new();
    let mut updates = Vec::new();

    let (rules_section, update_section) = input
        .split_once("\n\n")
        .expect("Could not split input by two newlines");

    for line in rules_section.lines() {
        let (before, after) = line
            .split_once('|')
            .expect("Could not split rule line by pipe");
        let before: usize = before.parse().expect("Unable to parse page number");
        let after: usize = after.parse().expect("Unable to parse page number");
        ordering_rules.push((before, after));
    }

    for line in update_section.lines() {
        let mut update = Vec::new();
        for page in line.split(',') {
            let page: usize = page.parse().expect("Unable to parse page number");
            update.push(page);
        }
        updates.push(update);
    }

    PrintJob {
        ordering_rules,
        updates,
    }
}

fn is_ordered(update: &Vec<usize>, rules: &[(usize, usize)]) -> bool {
    let mut sorted: Vec<usize> = update.clone();
    sorted.sort_by(|&a, &b| page_sort(a, b, rules));
    sorted == *update
}

fn page_sort(a: usize, b: usize, rules: &[(usize, usize)]) -> Ordering {
    if rules.contains(&(a, b)) {
        return std::cmp::Ordering::Less;
    }
    if rules.contains(&(b, a)) {
        return std::cmp::Ordering::Greater;
    }
    std::cmp::Ordering::Equal
}

fn part1(data: &ParsedData) -> usize {
    let ordered = data
        .updates
        .iter()
        .filter(|update| is_ordered(update, &data.ordering_rules));
    let middle_pages = ordered.map(|update| {
        update
            .get(update.len() / 2)
            .expect("Cannot get the middle page of an update")
    });
    middle_pages.sum()
}

fn part2(data: &ParsedData) -> usize {
    let incorrectly_ordered = data
        .updates
        .iter()
        .filter(|update| !is_ordered(update, &data.ordering_rules));
    let mut fixed: Vec<_> = incorrectly_ordered.cloned().collect();
    fixed
        .iter_mut()
        .for_each(|update| update.sort_by(|&a, &b| page_sort(a, b, &data.ordering_rules)));
    let middle_pages = fixed.iter().map(|update| {
        update
            .get(update.len() / 2)
            .expect("Cannot get the middle page of an update")
    });
    middle_pages.sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 143;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 123;
        assert_eq!(value, expected);
    }
}

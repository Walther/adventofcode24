use indicatif::ProgressIterator;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type Stone = u128;
type ParsedData = Vec<Stone>;

fn parse(input: &str) -> ParsedData {
    let stones = input
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();
    stones
}

fn evolve_stone(stone: Stone) -> Vec<Stone> {
    match stone {
        0 => vec![1],
        stone if (stone.ilog10() + 1) % 2 == 0 => {
            let stone_string = stone.to_string();
            let (l, r) = stone_string.split_at(stone_string.len() / 2);
            let l = l.parse().expect("Unable to create new stone from split");
            let r = r.parse().expect("Unable to create new stone from split");
            vec![l, r]
        }
        other => vec![other * 2024],
    }
}

fn step(stones: Vec<Stone>) -> Vec<Stone> {
    let mut new_stones = Vec::new();
    for stone in stones {
        new_stones.append(&mut evolve_stone(stone));
    }

    new_stones
}

fn simulate(mut stones: Vec<Stone>, steps: u8) -> Vec<Stone> {
    for _ in (0..steps).progress() {
        stones = step(stones);
    }

    stones
}

fn part1(data: &ParsedData) -> usize {
    let stones = data.clone();
    let stones = simulate(stones, 25);
    stones.len()
}

fn part2(_data: &ParsedData) -> usize {
    0
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"125 17";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 55312;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 0;
        assert_eq!(value, expected);
    }
}

#[cfg(test)]
mod unit {
    use crate::evolve_stone;

    #[test]
    fn evolve_0() {
        let stone = evolve_stone(0);
        let expected = vec![1];
        assert_eq!(stone, expected);
    }

    #[test]
    fn evolve_1() {
        let stone = evolve_stone(1);
        let expected = vec![2024];
        assert_eq!(stone, expected);
    }

    #[test]
    fn evolve_11() {
        let stone = evolve_stone(11);
        let expected = vec![1, 1];
        assert_eq!(stone, expected);
    }

    #[test]
    fn evolve_100() {
        let stone = evolve_stone(100);
        let expected = vec![202_400];
        assert_eq!(stone, expected);
    }

    #[test]
    fn evolve_1000() {
        let stone = evolve_stone(1000);
        let expected = vec![10, 0];
        assert_eq!(stone, expected);
    }

    #[test]
    fn evolve_1001() {
        let stone = evolve_stone(1001);
        let expected = vec![10, 1];
        assert_eq!(stone, expected);
    }
}

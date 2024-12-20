pub mod memory;
use memory::Memory;
use shared::Coordinate;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<Coordinate>;

fn parse(input: &str) -> ParsedData {
    input
        .lines()
        .map(|l| {
            let e = "Unable to parse coordinate";
            let (x, y) = l.split_once(',').expect(e);
            let x = x.parse().expect(e);
            let y = y.parse().expect(e);
            Coordinate::new(x, y)
        })
        .collect()
}

#[cfg(not(test))]
const PART1_FALL_COUNT: usize = 1024;
#[cfg(test)]
const PART1_FALL_COUNT: usize = 12;

fn part1(data: &ParsedData) -> usize {
    let mut memory = Memory::new();
    let bytes: Vec<Coordinate> = data.clone().into_iter().take(PART1_FALL_COUNT).collect();
    memory.add_bytes(&bytes);

    let path = memory.shortest_path().expect("No path found");
    // memory.add_path(&path);
    // memory.print();

    path.len() - 1
}

fn part2(data: &ParsedData) -> String {
    let index = (0..data.len())
        .collect::<Vec<usize>>()
        .partition_point(|&n| Memory::has_path_after_n_bytes(n, data));
    let byte = data[index];
    format!("{},{}", byte.x, byte.y)
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 22;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = "6,1";
        assert_eq!(value, expected);
    }
}

#[cfg(test)]
mod unit {
    #[test]
    fn unit() {
        let value = 0;
        let expected = 0;
        assert_eq!(value, expected);
    }
}

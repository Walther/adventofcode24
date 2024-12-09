use shared::maze::{Coordinate, Direction, Maze, Visitor, VisitorOptions};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Maze;

fn parse(input: &str) -> ParsedData {
    input.parse().expect("Unable to parse input")
}

fn part1(data: &ParsedData) -> usize {
    let mut xmas_count = 0;
    let letter_x_coordinates: Vec<Coordinate> = data.find_all('X');
    for coordinate in letter_x_coordinates {
        for direction in Direction::iter() {
            let mut visitor = Visitor::new(
                VisitorOptions {
                    has_pockets: true,
                    ..Default::default()
                },
                data,
                coordinate,
            );
            let collection = visitor.collect(4, direction);
            match collection {
                Some(collection) => {
                    if *collection.iter().collect::<String>() == *"XMAS" {
                        xmas_count += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    xmas_count
}

fn part2(data: &ParsedData) -> usize {
    let mut x_max_count = 0;
    for &coordinate in data.all_coordinates() {
        let visitor = Visitor::new(VisitorOptions::default(), data, coordinate);
        let surroundings = visitor.surroundings();
        match surroundings {
            Some(map) => {
                if is_x_mas(map) {
                    x_max_count += 1;
                }
            }
            _ => continue,
        }
    }

    x_max_count
}

#[rustfmt::skip]
fn is_x_mas(map: [&char; 9]) -> bool {
    matches!(map,
        [
            'M', _, 'M',
            _,  'A',  _,
            'S', _, 'S',
        ] | [
            'S', _, 'M',
            _,  'A',  _,
            'S', _, 'M',
        ] | [
            'M', _, 'S',
            _,  'A',  _,
            'M', _, 'S',
        ] | [
            'S', _, 'S',
            _,  'A',  _,
            'M', _, 'M',
        ])
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 18;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 9;
        assert_eq!(value, expected);
    }
}

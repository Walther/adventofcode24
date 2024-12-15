use std::collections::HashSet;

use shared::maze::{Coordinate, Maze, Visitor, VisitorOptions};

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
    input.parse().expect("Unable to parse maze")
}

struct Plot<'farm> {
    coordinates: HashSet<Coordinate>,
    farm: &'farm Maze,
}

impl Plot<'_> {
    fn contains(&self, coordinate: &Coordinate) -> bool {
        self.coordinates.contains(coordinate)
    }

    fn area(&self) -> usize {
        self.coordinates.len()
    }

    fn perimeter(&self) -> usize {
        self.coordinates
            .iter()
            .map(|&coordinate| {
                let visitor = Visitor::new(VisitorOptions::default(), self.farm, coordinate);
                let plant = visitor.get().expect("Unknown plant");
                visitor
                    .surroundings_nwes()
                    .iter()
                    .filter(|&neighbor| neighbor != &Some(plant))
                    .count()
            })
            .sum()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

fn part1(data: &ParsedData) -> usize {
    let farm = data;
    let mut plots: Vec<Plot> = Vec::new();
    for coordinate in farm.all_coordinates() {
        if plots.iter().any(|plot| plot.contains(coordinate)) {
            continue;
        }

        let visitor = Visitor::new(VisitorOptions::default(), farm, *coordinate);
        let coordinates = visitor
            .flood_nwes()
            .expect("Unable to flood fill garden plot");
        let plot = Plot { coordinates, farm };
        plots.push(plot);
    }

    let price = plots.iter().map(Plot::price).sum();

    price
}

fn part2(_data: &ParsedData) -> usize {
    2
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 1930;
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

#[cfg(test)]
mod unit {
    use std::collections::HashSet;

    use shared::maze::Coordinate;

    use crate::Plot;

    const INPUT: &str = r"AAAA
BBCD
BBCC
EEEC
";

    #[test]
    fn area() {
        let farm = crate::parse(INPUT);
        let plot = Plot {
            coordinates: HashSet::from([
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0),
                Coordinate::new(3, 0),
            ]),
            farm: &farm,
        };

        let area = plot.area();
        let expected = 4;
        assert_eq!(area, expected);
    }

    #[test]
    fn perimeter() {
        let farm = crate::parse(INPUT);
        let plot = Plot {
            coordinates: HashSet::from([
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0),
                Coordinate::new(3, 0),
            ]),
            farm: &farm,
        };

        let perimeter = plot.perimeter();
        let expected = 10;
        assert_eq!(perimeter, expected);
    }
}

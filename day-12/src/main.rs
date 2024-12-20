use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use shared::{Coordinate, Maze, Visitor};

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
    farm: &'farm Arc<Mutex<Maze>>,
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
                let visitor = Visitor::new(self.farm, coordinate);
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
    let farm = data.clone();
    let mut plots: Vec<Plot> = Vec::new();
    let all_coordinates = farm.all_coordinates();
    let farm = farm.make_shareable();
    for coordinate in all_coordinates {
        if plots.iter().any(|plot| plot.contains(&coordinate)) {
            continue;
        }

        let visitor = Visitor::new(&farm.clone(), coordinate);
        let coordinates = visitor
            .flood_nwes()
            .expect("Unable to flood fill garden plot");
        let plot = Plot {
            coordinates,
            farm: &farm,
        };
        plots.push(plot);
    }

    let price = plots.iter().map(Plot::price).sum();

    price
}

fn part2(_data: &ParsedData) -> usize {
    0
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
        let expected = 0;
        assert_eq!(value, expected);
    }
}

#[cfg(test)]
mod unit {
    use std::collections::HashSet;

    use shared::Coordinate;

    use crate::Plot;

    const INPUT: &str = r"AAAA
BBCD
BBCC
EEEC
";

    #[test]
    fn area() {
        let farm = crate::parse(INPUT);
        let farm = farm.make_shareable();
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
        let farm = farm.make_shareable();
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

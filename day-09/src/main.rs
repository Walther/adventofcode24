mod disk;
use disk::Disk;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Disk;

fn parse(input: &str) -> ParsedData {
    let mut disk = Disk::default();
    for (diskmap_position, num) in input.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        let block = match diskmap_position % 2 {
            // even position diskmap numbers are files, running numbering
            0 => {
                #[allow(clippy::cast_possible_truncation)]
                let file_id = (diskmap_position as u64) / 2;
                Some(file_id)
            }
            // odd position diskmap numbers are empty space
            1 => None,
            _ => unreachable!(),
        };
        for _block_number in 0..num {
            disk.append(block);
        }
    }

    disk
}

fn part1(data: &ParsedData) -> usize {
    let mut disk = data.clone();
    disk.compact_fragmented().expect("Compacting failed");
    disk.checksum().expect("Checksum failed")
}

fn part2(data: &ParsedData) -> usize {
    let mut disk = data.clone();
    disk.compact_non_fragmented().expect("Compacting failed");
    disk.checksum().expect("Checksum failed")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT);
        let value = crate::part1(&parsed);
        let expected = 1928;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT);
        let value = crate::part2(&parsed);
        let expected = 2858;
        assert_eq!(value, expected);
    }
}

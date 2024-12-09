use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

/// A single block on a disk
/// - `None` = empty space
/// - `Some(u64)` = file id
pub type Block = Option<u64>;

pub type Position = u64;

pub type FileId = u64;

#[derive(Default, Clone)]
pub struct Disk {
    pub contents: HashMap<Position, Block>,
    pub written: u64,
}

impl Disk {
    /// Write a new block at the end of the disk, expanding the written size by one.
    pub fn append(&mut self, block: Block) {
        self.contents.insert(self.written, block);
        self.written += 1;
    }

    /// Get the block at the given position. Returns None if the position is outside the disk.
    pub fn get(&self, position: u64) -> Option<Block> {
        self.contents.get(&position).copied()
    }

    /// Remove all blocks with the given `FileId`
    pub fn remove_file(&mut self, file_id: FileId) {
        self.contents.iter_mut().for_each(|(_position, id)| {
            if *id == Some(file_id) {
                *id = None;
            }
        });
    }

    /// Swap the values at the given two positions, if they exist on the disk.
    // FIXME: should probably return Result instead, but the ?s become annoying
    pub fn swap(&mut self, position_a: Position, position_b: Position) -> Option<()> {
        let &block_a = self.contents.get(&position_a)?;
        let &block_b = self.contents.get(&position_b)?;

        self.contents.insert(position_a, block_b)?;
        self.contents.insert(position_b, block_a)?;

        Some(())
    }

    /// Find the next empty position on the disk from the starting position `from`, inclusive.
    pub fn next_empty(&self, from: Position) -> Option<Position> {
        for position in from..self.written {
            match self.contents.get(&position) {
                Some(None) => return Some(position),
                Some(Some(_file_id)) => continue,
                None => unreachable!(),
            }
        }
        None
    }

    /// Find the next empty position on the disk with the contiguous length of `length` from the starting position `from`, inclusive.
    pub fn next_empty_contiguous(&self, from: Position, length: u64) -> Option<Position> {
        let mut offset = from;
        loop {
            offset = self.next_empty(offset)?;
            let mut contiguous = true;
            'inner: for position in offset..(offset + length) {
                if let Some(Some(_nonempty)) = self.get(position) {
                    contiguous = false;
                    offset = position;
                    break 'inner;
                }
            }

            if contiguous {
                return Some(offset);
            }
        }
    }

    /// Find the last non-empty block on the disk.
    pub fn last_nonempty(&self) -> Option<Position> {
        for position in (0..self.written).rev() {
            match self.contents.get(&position) {
                Some(Some(_file_id)) => return Some(position),
                Some(None) => continue,
                None => unreachable!(),
            }
        }
        None
    }

    /// Get the file length for the file with the specified `FileId`
    pub fn file_length(&self, file_id: FileId) -> u64 {
        self.contents
            .values()
            .filter(|&v| *v == Some(file_id))
            .count() as u64
    }

    /// Find the `Position` of the first block of the file with the specified `FileId`
    pub fn file_start(&self, file_id: FileId) -> Option<Position> {
        for position in 0..self.written {
            match self.contents.get(&position) {
                Some(Some(id)) => {
                    if *id == file_id {
                        return Some(position);
                    }
                }
                Some(None) => continue,
                None => unreachable!(),
            }
        }
        None
    }

    /// Compact the disk by moving the last non-empty block into the first empty block,
    /// looping until all empty blocks are at the end of the disk.
    pub fn compact_fragmented(&mut self) {
        loop {
            let Some(first_empty) = self.next_empty(0) else {
                break;
            };
            let Some(last_nonempty) = self.last_nonempty() else {
                break;
            };
            if first_empty < last_nonempty {
                self.swap(first_empty, last_nonempty);
            } else {
                break;
            }
        }
    }

    /// Compact the disk by moving all files, starting from the largest `FileId`,
    /// to the first available contiguous space with sufficient length to store the full file.
    pub fn compact_non_fragmented(&mut self) {
        let mut files: Vec<u64> = self
            .contents
            .values()
            .filter_map(|&block| block)
            .unique()
            .collect();
        files.sort_unstable();
        files.reverse();
        for file_id in files {
            let file_start = self.file_start(file_id).expect("Unable to find file start");
            let length = self.file_length(file_id);
            if let Some(space) = self.next_empty_contiguous(0, length) {
                if space < file_start {
                    self.remove_file(file_id);
                    for position in space..(space + length) {
                        self.contents.insert(position, Some(file_id));
                    }
                }
            }
        }
    }

    /// Calculates the disk checksum based on the positions and `FileId`s.
    #[allow(clippy::cast_possible_truncation)]
    pub fn checksum(&self) -> usize {
        let mut checksum = 0;
        for position in 0..self.written {
            match self.get(position) {
                Some(Some(file_id)) => checksum += position * file_id,
                Some(None) => continue,
                None => unreachable!(),
            }
        }
        checksum as usize
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        // FIXME: this becomes messy for values over 9
        for n in 0..(self.written) {
            match self.get(n) {
                Some(Some(id)) => string.push_str(&id.to_string()),
                Some(None) => string.push('.'),
                None => panic!("Attempted to read uninitialized part of disk"),
            }
        }
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn display() {
        let parsed = crate::parse(INPUT);
        let disk_string = parsed.to_string();
        let expected = "00...111...2...333.44.5555.6666.777.888899".to_string();
        assert_eq!(disk_string, expected);
    }

    #[test]
    fn next_empty_0() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty(0);
        let expected = Some(2);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn next_empty_5() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty(5);
        let expected = Some(8);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn next_empty_contiguous_0_3() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty_contiguous(0, 3);
        let expected = Some(2);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn next_empty_contiguous_0_4() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty_contiguous(0, 4);
        let expected = None;
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn last_nonempty() {
        let parsed = crate::parse(INPUT);
        let last_nonempty = parsed.last_nonempty();
        let expected = Some(41);
        assert_eq!(last_nonempty, expected);
    }
}

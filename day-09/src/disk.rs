use std::fmt::Display;

use itertools::Itertools;

/// A single block on a disk
/// - `None` = empty space
/// - `Some(u64)` = file id
pub type Block = Option<u64>;

pub type Position = usize;

pub type FileId = u64;

#[derive(Default, Clone)]
pub struct Disk {
    contents: Vec<Block>,
}

impl Disk {
    /// Write a new block at the end of the disk, expanding the written size by one.
    pub fn append(&mut self, block: Block) {
        self.contents.push(block);
    }

    /// Get the block at the given position. Returns None if the position is outside the disk.
    pub fn get(&self, position: Position) -> Result<Block, Error> {
        self.contents.get(position).ok_or(PositionNotFound).copied()
    }

    pub fn write(&mut self, position: Position, block: Block) -> Result<Block, Error> {
        self.contents
            .get_mut(position)
            .ok_or(WriteFailed)
            .map(|p| *p = block)?;
        Ok(block)
    }

    /// Remove all blocks with the given `FileId`
    pub fn remove_file(&mut self, file_id: FileId) {
        self.contents.iter_mut().for_each(|id| {
            if *id == Some(file_id) {
                *id = None;
            }
        });
    }

    /// Swap the values at the given two positions, if they exist on the disk.
    pub fn swap(&mut self, position_a: Position, position_b: Position) -> Result<(), Error> {
        let block_a = self.get(position_a)?;
        let block_b = self.get(position_b)?;

        self.write(position_a, block_b)?;
        self.write(position_b, block_a)?;

        Ok(())
    }

    /// Find the next empty position on the disk from the starting position `from`, inclusive.
    pub fn next_empty(&self, from: Position) -> Result<Position, Error> {
        for position in from..self.contents.len() {
            match self.get(position)? {
                None => return Ok(position),
                Some(_file_id) => continue,
            }
        }
        Err(EmptyNotFound)
    }

    /// Find the next empty position on the disk with the contiguous length of `length` from the starting position `from`, inclusive.
    pub fn next_empty_contiguous(&self, from: Position, length: usize) -> Result<Position, Error> {
        let mut offset = from;
        loop {
            offset = self.next_empty(offset)?;
            let mut contiguous = true;
            'inner: for position in offset..(offset + length) {
                if let Some(_nonempty) = self.get(position)? {
                    contiguous = false;
                    offset = position;
                    break 'inner;
                }
            }

            if contiguous {
                return Ok(offset);
            }
        }
    }

    /// Find the last non-empty block on the disk.
    pub fn last_nonempty(&self) -> Result<Position, Error> {
        for position in (0..self.contents.len()).rev() {
            match self.get(position)? {
                Some(_file_id) => return Ok(position),
                None => continue,
            }
        }
        Err(EmptyNotFound)
    }

    /// Get the file length for the file with the specified `FileId`
    pub fn file_length(&self, file_id: FileId) -> usize {
        self.contents
            .iter()
            .filter(|&v| *v == Some(file_id))
            .count()
    }

    /// Find the `Position` of the first block of the file with the specified `FileId`
    pub fn file_start(&self, file_id: FileId) -> Result<Position, Error> {
        self.contents
            .iter()
            .position(|id| *id == Some(file_id))
            .ok_or(FileNotFound)
    }

    /// Compact the disk by moving the last non-empty block into the first empty block,
    /// looping until all empty blocks are at the end of the disk.
    pub fn compact_fragmented(&mut self) -> Result<(), Error> {
        loop {
            let first_empty = self.next_empty(0)?;
            let last_nonempty = self.last_nonempty()?;
            if first_empty < last_nonempty {
                self.swap(first_empty, last_nonempty)?;
            } else {
                break Ok(());
            }
        }
    }

    /// Compact the disk by moving all files, starting from the largest `FileId`,
    /// to the first available contiguous space with sufficient length to store the full file.
    pub fn compact_non_fragmented(&mut self) -> Result<(), Error> {
        let mut files: Vec<u64> = self
            .contents
            .iter()
            .filter_map(|&block| block)
            .unique()
            .collect();
        files.sort_unstable();
        files.reverse();
        for file_id in files {
            let file_start = self.file_start(file_id)?;
            let length = self.file_length(file_id);
            if let Ok(space) = self.next_empty_contiguous(0, length) {
                if space < file_start {
                    self.remove_file(file_id);
                    for position in space..(space + length) {
                        self.write(position, Some(file_id))?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Calculates the disk checksum based on the positions and `FileId`s.
    #[allow(clippy::cast_possible_truncation)]
    pub fn checksum(&self) -> usize {
        let mut checksum = 0;
        for (position, block) in self.contents.iter().enumerate() {
            match block {
                Some(file_id) => checksum += position * (*file_id as usize),
                None => continue,
            }
        }
        checksum
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: this becomes messy for values over 9
        let string: String = self
            .contents
            .iter()
            .map(|block| match block {
                Some(file_id) => file_id.to_string(),
                None => ".".to_string(),
            })
            .collect();
        write!(f, "{string}")
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Position not found on disk")]
    PositionNotFound,
    #[error("File not found on disk")]
    FileNotFound,
    #[error("Empty space not found on disk")]
    EmptyNotFound,
    #[error("Write to disk failed")]
    WriteFailed,
}
#[allow(clippy::enum_glob_use)]
use Error::*;

#[cfg(test)]
mod unit {
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
        let expected = Ok(2);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn next_empty_5() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty(5);
        let expected = Ok(8);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn next_empty_contiguous_0_3() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty_contiguous(0, 3);
        let expected = Ok(2);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn next_empty_contiguous_0_4() {
        let parsed = crate::parse(INPUT);
        let next_empty = parsed.next_empty_contiguous(0, 4);
        let expected = Err(crate::disk::Error::EmptyNotFound);
        assert_eq!(next_empty, expected);
    }

    #[test]
    fn last_nonempty() {
        let parsed = crate::parse(INPUT);
        let last_nonempty = parsed.last_nonempty();
        let expected = Ok(41);
        assert_eq!(last_nonempty, expected);
    }
}

#[cfg(test)]
mod integration {
    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn compact_fragmented() {
        let mut disk = crate::parse(INPUT);
        disk.compact_fragmented().expect("Compacting failed");
        let string = disk.to_string();
        let expected = "0099811188827773336446555566..............".to_string();
        assert_eq!(string, expected);
    }

    #[test]
    fn compact_non_fragmented() {
        let mut disk = crate::parse(INPUT);
        disk.compact_non_fragmented().expect("Compacting failed");
        let string = disk.to_string();
        let expected = "00992111777.44.333....5555.6666.....8888..".to_string();
        assert_eq!(string, expected);
    }
}

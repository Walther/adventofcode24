use std::collections::{HashSet, VecDeque};

use shared::{Coordinate, Maze, Visitor};

#[cfg(not(test))]
const MEMORY_WIDTH: usize = 71;
#[cfg(test)]
const MEMORY_WIDTH: usize = 7;

#[derive(Default)]
pub struct Memory {
    maze: Maze,
}

impl Memory {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new() -> Self {
        let mut maze = Maze::default();
        for y in 0..MEMORY_WIDTH {
            for x in 0..MEMORY_WIDTH {
                maze.upsert(Coordinate::new(x as isize, y as isize), '.');
            }
        }

        Memory { maze }
    }

    pub fn add_bytes(&mut self, bytes: &[Coordinate]) {
        for &byte in bytes {
            self.maze.upsert(byte, '#');
        }
    }

    pub fn add_path(&mut self, path: &[Coordinate]) {
        for &step in path {
            self.maze.upsert(step, 'O');
        }
    }

    /// Returns the shortest path of this [`Memory`].
    ///
    /// # Panics
    ///
    /// Panics if the lock acquisition for the inner maze fails.
    #[must_use]
    pub fn shortest_path(&self) -> Option<Vec<Coordinate>> {
        let maze = self.maze.clone().make_shareable();
        let start = Coordinate::new(0, 0);
        #[allow(clippy::cast_possible_wrap)]
        let e = (MEMORY_WIDTH - 1) as isize;
        let end = Coordinate::new(e, e);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(PathNode {
            coordinate: start,
            parent: None,
        });
        visited.insert(start);
        while let Some(node) = queue.pop_front() {
            let coordinate = node.coordinate;
            if coordinate == end {
                return Some(node.into());
            }

            let visitor = Visitor::new(&maze, coordinate);
            for next in visitor.coordinates_nwes() {
                let value = maze.lock().expect("Unable to acquire lock").get(next);
                if value == Some('.') && !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(PathNode {
                        coordinate: next,
                        parent: Some(Box::new(node.clone())),
                    });
                }
            }
        }
        None
    }

    #[must_use]
    pub fn has_path_after_n_bytes(n: usize, bytes: &[Coordinate]) -> bool {
        let mut memory = Memory::new();
        memory.add_bytes(&bytes[0..n + 1]);
        memory.shortest_path().is_some()
    }

    /// Returns the print of this [`Memory`].
    ///
    /// # Panics
    ///
    /// Panics if the value at some memory address in the maze cannot be read.
    pub fn print(&self) {
        for y in 0..MEMORY_WIDTH {
            for x in 0..MEMORY_WIDTH {
                #[allow(clippy::cast_possible_wrap)]
                let c = self
                    .maze
                    .get(Coordinate::new(x as isize, y as isize))
                    .expect("Memory print error");
                print!("{c}");
            }
            println!();
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathNode {
    coordinate: Coordinate,
    parent: Option<Box<PathNode>>,
}

impl PathNode {
    #[must_use]
    pub fn depth(&self) -> usize {
        match &self.parent {
            Some(parent) => 1 + parent.depth(),
            None => 0,
        }
    }
}

impl From<PathNode> for Vec<Coordinate> {
    fn from(val: PathNode) -> Self {
        match &val.parent {
            Some(parent) => {
                let mut p: Vec<Coordinate> = (*parent.clone()).into();
                p.push(val.coordinate);
                p
            }
            None => vec![val.coordinate],
        }
    }
}

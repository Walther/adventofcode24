#[rustfmt::skip]
use Direction::{NW, N, NE, W, E, SW, S, SE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(N),
            '>' => Ok(E),
            'v' => Ok(S),
            '<' => Ok(W),
            // NOTE: diagonals missing for now
            d => Err(format!("Unable to parse direction {d}")),
        }
    }
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [NW, N, NE, W, E, SW, S, SE].iter().copied()
    }
}

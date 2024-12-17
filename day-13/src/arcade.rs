use std::str::FromStr;

use nalgebra::{Matrix, Matrix2};
use shared::{Coordinate, Displacement};

struct Button {
    pub effect: Displacement,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Unable to parse button")]
    Button,
    #[error("Unable to parse prize")]
    Prize,
    #[error("Unable to parse machine")]
    Machine,
}

impl FromStr for Button {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_name, rest) = s.split_once(": ").ok_or(ParseError::Button)?;

        let Some((x, y)) = parse_xy(rest) else {
            return Err(ParseError::Button);
        };

        let effect = Displacement::new(x, y);

        Ok(Button { effect })
    }
}

// FIXME: ugly parsing :<
fn parse_xy(s: &str) -> Option<(isize, isize)> {
    let (mut x, mut y) = s.split_once(", ")?;
    (_, x) = x.split_once('X')?;
    (_, y) = y.split_once('Y')?;
    if x.starts_with('=') {
        x = &x[1..];
    };
    if y.starts_with('=') {
        y = &y[1..];
    };
    let Ok(x) = x.parse() else {
        return None;
    };
    let Ok(y) = y.parse() else {
        return None;
    };
    Some((x, y))
}

struct Prize {
    location: Coordinate,
}

impl FromStr for Prize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once(": ").ok_or(ParseError::Prize)?;
        if name != "Prize" {
            return Err(ParseError::Prize);
        };

        let Some((x, y)) = parse_xy(rest) else {
            return Err(ParseError::Prize);
        };
        let location = Coordinate::new(x, y);

        Ok(Prize { location })
    }
}

#[derive(Clone)]
pub struct ClawMachine {
    a: Displacement,
    b: Displacement,
    prize: Coordinate,
    claw: Coordinate,
    tokens: usize,
}

impl ClawMachine {
    pub fn press_a(&mut self) {
        self.tokens += 3;
        self.claw += self.a;
    }

    pub fn press_b(&mut self) {
        self.tokens += 1;
        self.claw += self.b;
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn multi_press_a(&mut self, n: usize) {
        self.tokens += n * 3;
        self.claw += (n as isize) * self.a;
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn multi_press_b(&mut self, n: usize) {
        self.tokens += n;
        self.claw += (n as isize) * self.b;
    }

    #[must_use]
    pub fn on_prize(&self) -> bool {
        self.claw.x == self.prize.x && self.claw.y == self.prize.y
    }

    #[must_use]
    pub fn missed_prize(&self) -> bool {
        self.claw.x > self.prize.x || self.claw.y > self.prize.y
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    fn minimal_presses(&self) -> Option<(usize, usize)> {
        let transform: Matrix2<isize> = Matrix::from_columns(&[self.a, self.b]);
        let det = det(transform);
        let det_inv = det_times_inverse(transform);
        let det_point = det_inv * self.prize;
        let point = det_point.xy().map(|component| exact_div(component, det));
        Some((point.x? as usize, point.y? as usize))
    }

    #[must_use]
    pub fn minimal_solve_cost(&self) -> Option<usize> {
        self.minimal_presses().map(|(a, b)| 3 * a + b)
    }

    pub fn move_prize(&mut self, displacement: Displacement) {
        self.prize += displacement;
    }
}

fn det(m: Matrix2<isize>) -> isize {
    // determinant = ad - bc
    // a b
    // c d
    m.m11 * m.m22 - m.m12 * m.m21
}

/// Returns the almost-inverse of a matrix; still needs to be divided by the determinant
fn det_times_inverse(m: Matrix2<isize>) -> Matrix2<isize> {
    // 2x2 inverse = 1/det *
    // d -b
    // -c a
    Matrix2::new(m.m22, -m.m12, -m.m21, m.m11)
}

fn exact_div(a: isize, b: isize) -> Option<isize> {
    if a.rem_euclid(b) != 0 {
        return None;
    };
    Some(a / b)
}

impl FromStr for ClawMachine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.lines();
        let a: Button = match parts.next() {
            Some(it) => it.parse()?,
            None => return Err(ParseError::Machine),
        };
        let b: Button = match parts.next() {
            Some(it) => it.parse()?,
            None => return Err(ParseError::Machine),
        };
        let prize: Prize = match parts.next() {
            Some(it) => it.parse()?,
            None => return Err(ParseError::Machine),
        };

        Ok(ClawMachine {
            a: a.effect,
            b: b.effect,
            prize: prize.location,
            claw: Coordinate::new(0, 0),
            tokens: 0,
        })
    }
}

#[cfg(test)]
mod unit {
    use nalgebra::Matrix2;

    use super::{det, det_times_inverse};

    #[test]
    fn determinant() {
        let m = Matrix2::from([[2, 3], [1, 2]]);
        let det = det(m);
        let expected = 1;
        assert_eq!(det, expected);
    }

    #[test]
    fn inverse() {
        let m = Matrix2::from([[2, 3], [1, 2]]);
        let inv = det_times_inverse(m);
        let expected = Matrix2::from([[2, -3], [-1, 2]]);
        assert_eq!(inv, expected);
    }
}

use nalgebra::{Point2, Vector2};

pub mod direction;
pub use direction::*;
pub mod maze;
pub use maze::*;
pub mod visitor;
pub use visitor::*;

pub type Coordinate = Point2<isize>;
pub type Displacement = Vector2<isize>;

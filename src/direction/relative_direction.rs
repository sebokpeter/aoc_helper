use super::{cardinal_direction::CardinalDirection, Direction};


/// Four main directions: Up, Right, Down, Left
/// These enums can be used in, for example, a 2D grid to get the positions of elements relative to each other (e.g. point (0,0) is Up from point (1,0)).
/// This enum behaves the same as the `CardinalDirection` enum (North == Up, East == Right, South == Down, West == Left). It exists because in some situations it makes more sense to refer to the position of other elements as e.g 'Up' instead of 'North'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelativeDirection {
    Up,
    Right,
    Down,
    Left
}

impl RelativeDirection {

    /// Transform [`self`] into a [`CardinalDirection`].
    /// 
    /// # Example:
    /// 
    /// ```
    /// 
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// use aoc_helper::direction::relative_direction::RelativeDirection;
    /// 
    /// let up = RelativeDirection::Up;
    /// assert_eq!(CardinalDirection::North, up.to_cardinal());
    /// 
    /// ```
    pub fn to_cardinal(&self) -> CardinalDirection {
        match self {
            RelativeDirection::Up => CardinalDirection::North,
            RelativeDirection::Right => CardinalDirection::East,
            RelativeDirection::Down => CardinalDirection::South,
            RelativeDirection::Left => CardinalDirection::West
        }
    }
}

impl Direction for RelativeDirection {
    fn get_horizontal() -> [Self; 2] where Self: Sized {
        [RelativeDirection::Right, RelativeDirection::Left]
    }

    fn from_offset(offset: &(i8, i8)) -> Self where Self:Sized {
        CardinalDirection::from_offset(offset).to_relative()
    }

    fn get_vertical() -> [Self; 2] where Self: Sized {
        [RelativeDirection::Up, RelativeDirection::Down]
    }

    fn all() -> [Self; 4] where Self: Sized {
        [RelativeDirection::Up, RelativeDirection::Right, RelativeDirection::Down, RelativeDirection::Left]
    }

    fn get_opposite(&self) -> Self where Self: Sized {
        match self {
            RelativeDirection::Up => RelativeDirection::Down,
            RelativeDirection::Right => RelativeDirection::Left,
            RelativeDirection::Down => RelativeDirection::Up,
            RelativeDirection::Left => RelativeDirection::Right,
        }
    }

    fn get_offset(&self) -> (i8, i8) {
        self.to_cardinal().get_offset()
    }

    fn get_right(&self) -> Self where Self: Sized {
        match self {
            RelativeDirection::Up => RelativeDirection::Right,
            RelativeDirection::Right => RelativeDirection::Down,
            RelativeDirection::Down => RelativeDirection::Left,
            RelativeDirection::Left => RelativeDirection::Up,
        }
    }

    fn get_left(&self) -> Self where Self: Sized {
        match self {
            RelativeDirection::Up => RelativeDirection::Left,
            RelativeDirection::Right => RelativeDirection::Up,
            RelativeDirection::Down => RelativeDirection::Right,
            RelativeDirection::Left => RelativeDirection::Down,
        }
    }
}
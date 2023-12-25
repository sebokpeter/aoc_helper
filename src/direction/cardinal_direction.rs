use super::{Direction, relative_direction::RelativeDirection};

/// Four main directions: North, East, South, West.
/// These enums can be used in, for example, a 2D grid to get the positions of elements relative to each other (e.g. point (0,0) is to the North of point (1,0)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl CardinalDirection {
    
    /// Convert [`self`] to a [`RelativeDirection`]
    pub fn to_relative(&self) -> RelativeDirection {
        match self {
            CardinalDirection::North => RelativeDirection::Up,
            CardinalDirection::East => RelativeDirection::Right,
            CardinalDirection::South => RelativeDirection::Down,
            CardinalDirection::West => RelativeDirection::Left,
        }
    }
}

// Associated functions
impl Direction for CardinalDirection {
    /// Return the horizontal directions (West and East) as an array:
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    ///  let horizontal_directions = [CardinalDirection::West, CardinalDirection::East];
    ///  assert_eq!(horizontal_directions, Direction::get_horizontal());
    /// ```
     fn get_horizontal() -> [CardinalDirection; 2] {
        [CardinalDirection::West, CardinalDirection::East]
    }

    /// Get a direction that will correspond to the given offset in a 2D grid. The offset should be in the format (row_offset, col_offset). The offset values should be one of -1, 0, or 1.
    /// Note that (0, 0) is not a valid offset value, as that represents the current location.
    /// 
    /// # Examples:
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let north = CardinalDirection::North;
    /// assert_eq!(CardinalDirection::South, north.get_opposite());
    /// 
    /// let east = CardinalDirection::East;
    /// assert_eq!(CardinalDirection::West, east.get_opposite());
    /// 
    /// let south = CardinalDirection::South;
    /// assert_eq!(CardinalDirection::North, south.get_opposite());
    /// 
    /// let west = CardinalDirection::West;
    /// assert_eq!(CardinalDirection::East, west.get_opposite());
    /// 
    /// // These will panic:
    /// // let same = (0, 0);
    /// // let _ = Direction::get_dir_from_offset(&same);
    /// 
    ///  // let inv = (2, 0);
    ///  // let _ = Direction::get_dir_from_offset(&inv);
    /// ``` 
     fn from_offset(offset: &(i8, i8)) -> CardinalDirection {
        match offset {
            (-1, 0) => CardinalDirection::North,
            (1, 0) => CardinalDirection::South,
            (0, 1) => CardinalDirection::East,
            (0, -1) => CardinalDirection::West, 
            (0, 0) => panic!("(0, 0) is not a valid offset, as it represents the current position."),
            _ => panic!("Invalid format! The offset should be in the format (row_offset, col_offset), where both values must be either -1, 0, or 1.")
        }
    }

    /// Return the vertical directions (North and South) as an array:
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let vertical_directions = [CardinalDirection::North, CardinalDirection::South];
    /// assert_eq!(vertical_directions, Direction::get_vertical());    
    /// ```
     fn get_vertical() -> [CardinalDirection; 2] {
        [CardinalDirection::North, CardinalDirection::South]
    }

    /// Iterate over all [`CardinalDirection`] variants. The iterator starts at [`CardinalDirection::North`], and moves clockwise.
    /// 
    /// # Example: 
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let expected = vec![CardinalDirection::North, CardinalDirection::East, CardinalDirection::South, CardinalDirection::West];
    /// assert_eq!(expected, CardinalDirection::all().into_iter().collect::<Vec<_>>());
    /// ```
     fn all() -> [CardinalDirection; 4] {
        [CardinalDirection::North, CardinalDirection::East, CardinalDirection::South, CardinalDirection::West]
    }

     /// Returns [`CardinalDirection`] that is opposite of this [`CardinalDirection`] ([`CardinalDirection::West`] <-> [`CardinalDirection::East`] and [`CardinalDirection::North`] <-> [`CardinalDirection::South`]). 
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let north = CardinalDirection::North;
    /// assert_eq!(CardinalDirection::South, north.get_opposite());
    ///
    /// let east = CardinalDirection::East;
    /// assert_eq!(CardinalDirection::West, east.get_opposite());
    ///
    /// let south = CardinalDirection::South;
    /// assert_eq!(CardinalDirection::North, south.get_opposite());
    ///
    /// let west = CardinalDirection::West;
    /// assert_eq!(CardinalDirection::East, west.get_opposite());
    /// ```
    fn get_opposite(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }


    /// Get an offset that will correspond to this [`CardinalDirection`] in a 2D grid.
    /// 
    /// # Examples:
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let north = CardinalDirection::North;
    /// assert_eq!((-1, 0), north.get_offset());
    ///
    /// let east = CardinalDirection::East;
    /// assert_eq!((0, 1), east.get_offset());
    ///
    /// let south = CardinalDirection::South;
    /// assert_eq!((1, 0), south.get_offset());
    ///
    /// let west = CardinalDirection::West;
    /// assert_eq!((0, -1), west.get_offset()); 
    /// ```
     fn get_offset(&self) -> (i8, i8) {
        match self {
            CardinalDirection::North => (-1, 0),
            CardinalDirection::East => (0, 1),
            CardinalDirection::South => (1, 0),
            CardinalDirection::West => (0, -1),
        }
    }


    /// Returns the [`CardinalDirection`] to the right of [`self`].
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let north = CardinalDirection::North;
    /// assert_eq!(CardinalDirection::East, north.get_right());
    ///
    /// let east = CardinalDirection::East;
    /// assert_eq!(CardinalDirection::South, east.get_right());
    ///
    /// let south = CardinalDirection::South;
    /// assert_eq!(CardinalDirection::West, south.get_right());
    ///
    /// let west = CardinalDirection::West;
    /// assert_eq!(CardinalDirection::North, west.get_right());
    /// ```
     fn get_right(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    /// Returns the [`CardinalDirection`] to the left of [`self`].
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let north = CardinalDirection::North;
    /// assert_eq!(CardinalDirection::West, north.get_left());
    ///
    /// let east = CardinalDirection::East;
    /// assert_eq!(CardinalDirection::North, east.get_left());
    ///
    /// let south = CardinalDirection::South;
    /// assert_eq!(CardinalDirection::East, south.get_left());
    ///
    /// let west = CardinalDirection::West;
    /// assert_eq!(CardinalDirection::South, west.get_left());
    /// ```
     fn get_left(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[test]
    fn horizontal_works() {
        let horizontal_directions = [CardinalDirection::West, CardinalDirection::East];
        assert_eq!(horizontal_directions, CardinalDirection::get_horizontal());
    }

    #[test]
    fn vertical_works() {
        let vertical_directions = [CardinalDirection::North, CardinalDirection::South];
        assert_eq!(vertical_directions, CardinalDirection::get_vertical());
    }

    #[test]
    fn get_opposite_works() {
        let north = CardinalDirection::North;
        assert_eq!(CardinalDirection::South, north.get_opposite());

        let east = CardinalDirection::East;
        assert_eq!(CardinalDirection::West, east.get_opposite());

        let south = CardinalDirection::South;
        assert_eq!(CardinalDirection::North, south.get_opposite());

        let west = CardinalDirection::West;
        assert_eq!(CardinalDirection::East, west.get_opposite());
    }

    #[test]
    fn get_offset_works() {
        let north = CardinalDirection::North;
        assert_eq!((-1, 0), north.get_offset());

        let east = CardinalDirection::East;
        assert_eq!((0, 1), east.get_offset());

        let south = CardinalDirection::South;
        assert_eq!((1, 0), south.get_offset());

        let west = CardinalDirection::West;
        assert_eq!((0, -1), west.get_offset()); 
    }

    #[test]
    fn iterator_works() {
        let expected = vec![CardinalDirection::North, CardinalDirection::East, CardinalDirection::South, CardinalDirection::West];
        assert_eq!(expected, CardinalDirection::all().into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn get_direction_from_offset_works() {
        let north = (-1, 0);
        assert_eq!(CardinalDirection::North, CardinalDirection::from_offset(&north));

        let east = (0, 1);
        assert_eq!(CardinalDirection::East, CardinalDirection::from_offset(&east));

        let south = (1, 0);
        assert_eq!(CardinalDirection::South, CardinalDirection::from_offset(&south));

        let west = (0, -1);
        assert_eq!(CardinalDirection::West, CardinalDirection::from_offset(&west));
    }

    #[test]
    #[should_panic]
    fn get_direction_should_panic_on_invalid_input_input_is_same() {
        let same = (0, 0);
        let _ = CardinalDirection::from_offset(&same);
    }


    #[test]
    #[should_panic]
    fn get_direction_should_panic_on_invalid_input_input_is_offset_more_than_one() {
        let same = (2, 0);
        let _ = CardinalDirection::from_offset(&same);
    }

    #[test]
    fn get_right_works() {
        let north = CardinalDirection::North;
        assert_eq!(CardinalDirection::East, north.get_right());

        let east = CardinalDirection::East;
        assert_eq!(CardinalDirection::South, east.get_right());

        let south = CardinalDirection::South;
        assert_eq!(CardinalDirection::West, south.get_right());

        let west = CardinalDirection::West;
        assert_eq!(CardinalDirection::North, west.get_right());
    }

    #[test]
    fn get_left_works() {
        let north = CardinalDirection::North;
        assert_eq!(CardinalDirection::West, north.get_left());
    
        let east = CardinalDirection::East;
        assert_eq!(CardinalDirection::North, east.get_left());
    
        let south = CardinalDirection::South;
        assert_eq!(CardinalDirection::East, south.get_left());
    
        let west = CardinalDirection::West;
        assert_eq!(CardinalDirection::South, west.get_left());
    }
}

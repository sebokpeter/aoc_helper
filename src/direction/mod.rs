
/// Trait that can be implemented by types that indicate direction (e.g. North, East, Up, Right, etc.).
pub trait Direction {
    /// Return the horizontal directions (e.g West and East) as an array:
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    ///  let horizontal_directions = [CardinalDirection::West, CardinalDirection::East];
    ///  assert_eq!(horizontal_directions, Direction::get_horizontal());
    /// ```
     fn get_horizontal() -> [Self; 2] where Self: Sized;

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
    /// // let _ = CardinalDirection::get_dir_from_offset(&same);
    /// 
    ///  // let inv = (2, 0);
    ///  // let _ = CardinalDirection::get_dir_from_offset(&inv);
    /// ``` 
     fn from_offset(offset: &(i8, i8)) -> Self where Self:Sized;

    /// Return the vertical directions (e.g. North and South) as an array:
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let vertical_directions = [CardinalDirection::North, CardinalDirection::South];
    /// assert_eq!(vertical_directions, Direction::get_vertical());    
    /// ```
     fn get_vertical() -> [Self; 2] where Self: Sized;

    /// An array of all [`Direction`] variants.
    /// 
    /// # Example: 
    /// ```
    /// use crate::aoc_helper::direction::Direction;
    /// use aoc_helper::direction::cardinal_direction::CardinalDirection;
    /// 
    /// let expected = vec![CardinalDirection::North, CardinalDirection::East, CardinalDirection::South, CardinalDirection::West];
    /// assert_eq!(expected, CardinalDirection::all().into_iter().collect::<Vec<_>>());
    /// ```
    fn all() -> [Self; 4] where Self: Sized;

    /// Returns [`Direction`] that is opposite of this [`Direction`] ([`CardinalDirection::West`] <-> [`CardinalDirection::East`] and [`CardinalDirection::North`] <-> [`CardinalDirection::South`]). 
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
    fn get_opposite(&self) -> Self where Self: Sized;


    /// Get an offset that will correspond to this [`Direction`] in a 2D grid.
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
     fn get_offset(&self) -> (i8, i8);


    /// Returns the [`Direction`] to the right of [`self`].
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
     fn get_right(&self) -> Self where Self: Sized;

    /// Returns the [`Direction`] to the left of [`self`].
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
    fn get_left(&self) -> Self where Self: Sized;
}


pub mod cardinal_direction;
pub mod relative_direction;

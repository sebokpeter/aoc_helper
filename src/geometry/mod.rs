#![allow(dead_code)]
use num_traits::{Num, Pow, PrimInt, Signed};

/// Represents a point in two dimensional space. The type of T indicates the type of the x and y coordinates. 
/// If all the points are located at integer coordinates, T will most likely be an integer type (e.g. [`i32`], [`isize`], [`u32`] or [`usize`]).
/// 
/// # Example:
///  
/// ```
/// use aoc_helper::geometry::Point2D;
/// 
/// // Points with signed integer coordinates
/// let p1 = Point2D { x: 5, y: 6 };
///  
/// // Points with floating point coordinates
/// let p3 = Point2D {x: 5.0, y: 6.0};
/// 
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2D<T> 
{
    pub x: T,
    pub y: T
}

impl<T> Point2D<T> 
where
    T: Num
{
    /// Creates a new [`Point2D<T>`].
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    /// Measure the euclidean distance to the other [`Point2D`].
    /// The distance is calculated the following way:
    /// 
    /// d = sqrt((self.x-other.x)^2 + (self.y - other.y)^2)
    /// 
    /// # Arguments:
    /// 
    /// * 'other' - A reference to another [`Point2D`] struct.
    /// 
    /// # Example:
    /// ```
    /// use aoc_helper::geometry::Point2D;
    ///  
    /// let p1 = Point2D { x: 5, y: 6 };
    /// let p2 = Point2D { x: -7, y:11 };
    /// 
    /// assert_eq!(13.0, p1.euclidean_distance_to(&p2));
    /// ```
    pub fn euclidean_distance_to(&self, other: &Point2D<T>) -> f64 
    where
        T: Clone + Copy,
        f64: From::<T>
    {
        let lhs = f64::from(self.x - other.x).pow(2);
        let rhs = f64::from(self.y - other.y).pow(2);
        let res: f64 = lhs + rhs;
        res.sqrt()
    }

    /// Returns the distance to the other [`Point2D`] measured along axes at right angles.
    /// In other words, it returns the sum of distances between the x and y coordinates.
    /// T must be a signed type.
    /// 
    /// # Arguments:
    /// 
    /// * 'other' - A reference to another [`Point2D`] struct.
    /// 
    /// # Example:
    /// ```
    /// use aoc_helper::geometry::Point2D;
    /// 
    /// let p1: Point2D<isize> = Point2D {x: 1, y: 4};
    /// let p2: Point2D<isize> = Point2D {x: 4, y: 1};
    /// 
    /// assert_eq!(6, p1.manhattan_distance_to(&p2));
    /// ```
    pub fn manhattan_distance_to(&self, other: &Point2D<T>) -> T 
    where
        T: PrimInt + Signed
    {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn euclidean_distance_to_works() {
        let p1 = Point2D { x: 5, y: 6 };
        let p2 = Point2D { x: -7, y:11 };

        let expected = 13.0;
        let actual = p1.euclidean_distance_to(&p2);

        assert_eq!(expected, actual);
    } 

    #[test]
    fn manhattan_distance_to_works() {
        let p1: Point2D<isize> = Point2D {x: 1, y: 4};
        let p2: Point2D<isize> = Point2D {x: 4, y: 1};

        let expected_1 = 6;
        let actual_1 = p1.manhattan_distance_to(&p2);

        assert_eq!(expected_1, actual_1);

        let p3: Point2D<isize> = Point2D {x: 658, y: 974};
        let p4: Point2D<isize> = Point2D {x: 1001, y: 589};

        let expected_2 = 728;
        let actual_2 = p3.manhattan_distance_to(&p4);

        assert_eq!(expected_2, actual_2);
    }

    #[test]
    fn equal_works() {
        let p1 = Point2D {x: 42, y: 42};
        let p2 = Point2D { x: 42, y: 42};

        assert_eq!(p1, p2);
    }
}
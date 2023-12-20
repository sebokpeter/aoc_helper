#![allow(dead_code)]
use std::ops::{Add, Mul, Sub, Div};
use num_traits::{Num, Pow, FromPrimitive};

/// Represents a point in two dimensional space. The type of T indicates the type of the x and y coordinates. If all the points are located at integer coordinates, T will most likely be an integer type (e.g. [`i32`], [`isize`], [`u32`] or [`usize`]).
/// 
/// # Example:
///  
/// ```
/// use aoc_helper::geometry::Point2D;
/// 
/// // Points with signed integer coorinates
/// let p1 = Point2D { x: 5, y: 6 };
///  
/// // Points with floating point coordinates
/// let p3 = Point2D {x: 5.0, y: 6.0};
/// 
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Point2D<T> 
where
    T: Num,
{
    pub x: T,
    pub y: T
}

impl<T> Point2D<T> 
where
    T: Num + PartialEq + PartialOrd + Ord + FromPrimitive + Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + Copy,
{
    /// Creates a new [`Point2D<T>`].
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    /// Measure the eucladian distance to the other [`Point2D`].
    /// The distance is calulated the following way:
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
    /// assert_eq!(13.0, p1.eucledian_distance_to(&p2));
    /// ```
    pub fn eucledian_distance_to(&self, other: &Point2D<T>) -> f64 
    where
        f64: From::<T>
    {
        let lhs = f64::from(self.x - other.x).pow(2);
        let rhs = f64::from(self.y - other.y).pow(2);
        let res: f64 = lhs + rhs;
        res.sqrt()
    }
}

impl Point2D<isize>
{   
    
    /// Returns the distance to the other [`Point2D`] measured along axes at right angles.
    /// In other words, it returns the sum of distances between the x and y coordinates.
    /// This function requires T to be [`isize`].
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
    /// assert_eq!(6, p1.mannhattan_distance_to(&p2));
    /// ```
    pub fn mannhattan_distance_to(&self, other: &Point2D<isize>) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eucledian_distance_to_works() {
        let p1 = Point2D { x: 5, y: 6 };
        let p2 = Point2D { x: -7, y:11 };

        let expected = 13.0;
        let actual = p1.eucledian_distance_to(&p2);

        assert_eq!(expected, actual);
    } 

    #[test]
    fn mannhattan_distance_to_works() {
        let p1: Point2D<isize> = Point2D {x: 1, y: 4};
        let p2: Point2D<isize> = Point2D {x: 4, y: 1};

        let expected_1 = 6;
        let actual_1 = p1.mannhattan_distance_to(&p2);

        assert_eq!(expected_1, actual_1);

        let p3: Point2D<isize> = Point2D {x: 658, y: 974};
        let p4: Point2D<isize> = Point2D {x: 1001, y: 589};

        let expected_2 = 728;
        let actual_2 = p3.mannhattan_distance_to(&p4);

        assert_eq!(expected_2, actual_2);
    }
}
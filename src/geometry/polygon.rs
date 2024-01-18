#![allow(dead_code)]
use std::cmp::{max, min};

use num_traits::Num;

use crate::{direction::relative_direction::RelativeDirection, iter_ext::IterExt};

use super::point::Point2D;

/// A polygon is a shape defined by three or more vertices (points).
/// The perimeter of the polygon is defined by pairs of vertices. For example, given vertices [a, b, c], the polygon has the following lines:
/// 1. Line from a to b (a->b),
/// 2. Line from b to c (b->c),
/// 3. Line from c to a (c->a)
///
/// Type parameter 'T' is a numeric type, that will be used to represent the coordinates of the vertices in this polygon.
pub struct Polygon<T> {
    vertices: Vec<Point2D<T>>,
}

impl<T> Polygon<T>
where
    T: Num + Ord,
    T: Clone + Copy,
    f64: From<T>,
{
    /// Creates a new [`Polygon<T>`], with no vertices.
    pub fn new() -> Polygon<T> {
        Polygon {
            vertices: Vec::new(),
        }
    }

    /// Creates a new [`Polygon<T>`], using [vertices].
    /// It is assumed that the [vertices] vector is sorted, so that the points are in either clockwise or counter-clockwise order.
    ///
    /// # Arguments
    ///
    /// * `vertices` - A [`Vec<Point<T>>`] that holds the initial vertices for this polygon.
    pub fn new_with_vertices(vertices: Vec<Point2D<T>>) -> Polygon<T> {
        Polygon { vertices }
    }

    /// Returns the number vertices that make up this [`Polygon<T>`].
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Add a vertex to the end of [vertices](Polygon::vertices).
    /// Given a polygon with three vertices [a, b, c], adding a new vertex, 'd', to the end of [vertices](Polygon::vertices) will result in [a, b, c, d].
    ///
    /// # Arguments
    ///
    /// * `vertex` - A [`Point2D<T>`] that will be added to [vertices](Polygon::vertices).
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_helper::geometry::polygon::Polygon;
    /// use aoc_helper::geometry::point::Point2D;
    ///
    /// let mut polygon = Polygon::new();
    ///
    /// polygon.add_vertex_end(Point2D {x: 0, y: 0});
    /// polygon.add_vertex_end(Point2D {x: 0, y: 2});
    /// polygon.add_vertex_end(Point2D {x: 1, y: 1});
    /// ```
    pub fn add_vertex_end(&mut self, vertex: Point2D<T>) {
        self.vertices.push(vertex);
    }

    /// Add a vertex to the start of [vertices](Polygon::vertices).
    /// Given a polygon with three vertices [a, b, c], adding a new vertex, 'd', to the start of [vertices](Polygon::vertices) will result in [d, a, b, c].
    ///
    /// # Arguments
    ///
    /// * `vertex` - A [`Point2D<T>`] that will be added to [vertices](Polygon::vertices).
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_helper::geometry::polygon::Polygon;
    /// use aoc_helper::geometry::point::Point2D;
    ///
    /// let mut polygon = Polygon::new();
    ///
    /// polygon.add_vertex_front(Point2D {x: 0, y: 0});
    /// polygon.add_vertex_front(Point2D {x: 0, y: 2});
    /// polygon.add_vertex_front(Point2D {x: 1, y: 1});
    /// ```    
    pub fn add_vertex_front(&mut self, vertex: Point2D<T>) {
        self.vertices.insert(0, vertex);
    }

    /// Returns the perimeter of this [`Polygon<T>`].
    /// The perimeter is simply the sum of the lengths of all the lines that make up this polygon.
    /// For example, given a polygon with three vertices [a, b, c], we can calculate the perimeter such as this:
    ///
    /// perimeter = line_len(a, b) + line_len(b, c) + line_len(c, a);    
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_helper::geometry::point::Point2D;
    /// use aoc_helper::geometry::polygon::Polygon;
    ///
    /// let vertices = vec![Point2D::new(0, 0), Point2D::new(1, 1), Point2D::new(0, 2)];
    /// let polygon = Polygon::new_with_vertices(vertices);
    ///
    /// let perimeter = polygon.perimeter(); // perimeter == 4.828...
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if this polygon does not have at least three vertices.
    pub fn perimeter(&self) -> f64 {
        if self.vertices.len() < 3 {
            panic!("Must have at least three vertices in a polygon.");
        }

        let mut perimeter = self
            .vertices
            .iter()
            .zip(self.vertices.iter().skip(1))
            .fold(0_f64, |acc, (a, b)| acc + a.euclidean_distance_to(b));

        // Distance between the first and last vertices
        perimeter += self.vertices[0].euclidean_distance_to(self.vertices.last().unwrap());

        perimeter
    }

    /// Returns the area of this [`Polygon<T>`], calculated using the Shoelace formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_helper::geometry::point::Point2D;
    /// use aoc_helper::geometry::polygon::Polygon;
    ///
    /// let vertices = vec![Point2D::new(0, 0), Point2D::new(1, 1), Point2D::new(0, 2)];
    /// let polygon = Polygon::new_with_vertices(vertices);
    ///
    /// let area = polygon.area();
    ///
    /// assert_eq!(1_f64, area);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if this polygon does not have at least three vertices.
    pub fn area(&self) -> f64 {
        let len = self.vertices.len();

        if len < 3 {
            panic!("Must have at least three vertices in a polygon.");
        }

        let mut area = 0_f64;

        for i in 0..len {
            let next_i = (i + 1) % len;
            area += f64::from(
                self.vertices[i].x * self.vertices[next_i].y
                    - self.vertices[next_i].x * self.vertices[i].y,
            );
        }

        0.5 * area.abs()
    }

    /// Check if the given [point] is inside this [`Polygon<T>`].
    ///
    /// # Arguments
    ///
    /// * `point` - The point that is being tested
    ///
    /// # Examples
    /// 
    /// ```
    /// use aoc_helper::geometry::point::Point2D;
    /// use aoc_helper::geometry::polygon::Polygon;
    /// 
    /// let vertices = vec![Point2D::new(0, 0), Point2D::new(3, 0), Point2D::new(0, 4)];
    /// let polygon = Polygon::new_with_vertices(vertices);
    ///
    /// let contains = polygon.contains_point(Point2D::new(2, 2));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if this polygon does not have at least three vertices.
    pub fn contains_point(&self, point: Point2D<T>) -> bool {
        let len = self.vertices.len();

        if len < 3 {
            panic!("Must have at least three vertices in a polygon.");
        }

        // Check if the point is inside the bounding box that surrounds this polygon
        let mut min_x = self.vertices[0].x;
        let mut min_y = self.vertices[0].y;
        let mut max_x = self.vertices[0].x;
        let mut max_y = self.vertices[0].y;

        for vertex in self.vertices.iter().skip(1) {
            min_x = min(min_x, vertex.x);
            min_y = min(min_y, vertex.y);
            max_x = max(max_x, vertex.x);
            max_y = max(max_y, vertex.y);
        }

        if point.x > max_x || point.y > max_y || point.x < min_x || point.y < min_y {
            return false;
        }

        // Starting from the point, shoot a horizontal ray, and count how many times it crosses the boundary of the polygon.
        // If the number of crosses is even, the point is outside of the polygon, otherwise it is inside.
        // From: https://web.archive.org/web/20161108113341/https://www.ecse.rpi.edu/Homepages/wrf/Research/Short_Notes/pnpoly.html

        let mut result = false;
        let mut i = 0;
        let mut j = 1;

        while i < len {
            let point_i = self.vertices[i];
            let point_j = self.vertices[j];

            if (point_i.y > point.y) != (point_j.y > point.y)
                && (point.x
                    < (point_j.x - point_i.x) * (point.y - point_i.y) / (point_j.y - point_i.y)
                        + point_i.x)
            {
                result = !result;
            }

            i += 1;
            j = (i + 1) % len;
        }

        result
    }
}

impl<T> Default for Polygon<T>
where
    T: Num + Ord,
    T: Clone + Copy,
    f64: From<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn can_create_polygon_with_no_vertices() {
        let mut polygon = Polygon::new();

        polygon.add_vertex_end(Point2D::new(0, 0));
        polygon.add_vertex_end(Point2D::new(0, 2));
        polygon.add_vertex_end(Point2D::new(1, 1));

        assert_eq!(3, polygon.vertices.len());
        assert_eq!(Point2D::new(0, 0), polygon.vertices[0]);
        assert_eq!(Point2D::new(1, 1), polygon.vertices[2]);
    }

    #[test]
    fn can_create_polygon_with_vertices() {
        let vertices = vec![Point2D::new(0, 0), Point2D::new(1, 1), Point2D::new(0, 2)];

        let polygon = Polygon::new_with_vertices(vertices);

        assert_eq!(3, polygon.vertices.len());
        assert_eq!(Point2D::new(0, 0), polygon.vertices[0]);
        assert_eq!(Point2D::new(0, 2), polygon.vertices[2]);
    }

    #[test]
    fn can_calculate_perimeter() {
        let vertices = vec![Point2D::new(0, 0), Point2D::new(1, 1), Point2D::new(0, 2)];
        let polygon = Polygon::new_with_vertices(vertices);

        let perimeter = polygon.perimeter();
        // Round to 3 decimal places
        let perimeter_rounded = round(perimeter, 3);

        assert_eq!(4.828, perimeter_rounded);

        // It would be nice to have parameterized tests

        let vertices = vec![
            Point2D::new(0, 0),
            Point2D::new(10, 30),
            Point2D::new(150, 120),
            Point2D::new(48, 5),
            Point2D::new(36, 84),
            Point2D::new(84, 99),
        ];

        let polygon = Polygon::new_with_vertices(vertices);

        let perimeter = polygon.perimeter();
        let perimeter_rounded = round(perimeter, 3);

        assert_eq!(611.803, perimeter_rounded);
    }

    #[test]
    fn can_calculate_area() {
        let vertices = vec![Point2D::new(0, 0), Point2D::new(1, 1), Point2D::new(0, 2)];
        let polygon = Polygon::new_with_vertices(vertices);

        let area = polygon.area();
        // Round to 3 decimal places
        let area_rounded = round(area, 3);

        assert_eq!(1_f64, area_rounded);

        let vertices = vec![
            Point2D::new(0, 0),
            Point2D::new(10, 30),
            Point2D::new(150, 120),
            Point2D::new(48, 5),
            Point2D::new(36, 84),
            Point2D::new(84, 99),
        ];
        let polygon = Polygon::new_with_vertices(vertices);

        let area = polygon.area();
        // Round to 3 decimal places
        let area_rounded = round(area, 3);

        assert_eq!(3975_f64, area_rounded);
    }

    #[test]
    fn point_in_polygon_works() {
        let vertices = vec![Point2D::new(0, 0), Point2D::new(3, 0), Point2D::new(0, 4)];
        let polygon = Polygon::new_with_vertices(vertices);
    
        assert!(!polygon.contains_point(Point2D::new(-1, 0)));
        assert!(!polygon.contains_point(Point2D::new(2, 2)));
        assert!(!polygon.contains_point(Point2D::new(4, 0)));
        assert!(!polygon.contains_point(Point2D::new(3, 0)));


        assert!(polygon.contains_point(Point2D::new(1, 1)));
        assert!(polygon.contains_point(Point2D::new(1, 2)));
        assert!(polygon.contains_point(Point2D::new(2, 1)));
    }

    #[test]
    fn can_calculate_day_18() {
        // Example from AoC 2023 Day 18, Part 1
        let data = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let digs = data.lines().map(Dig::new).collect_vec();

        let vertices = get_vertices(digs);
        let polygon = Polygon::new_with_vertices(vertices);

        let inner_area = polygon.area();
        let trench_area = polygon.perimeter();

        // Divide trench_area by 2, because the inner_area already includes half of the trench
        // Add plus 1, because the polygon is closed, so the perimeter does a 360 degree turn while it goes around
        let total = inner_area + (trench_area / 2_f64) + 1_f64;

        assert_eq!(62_f64, total);
    }

    fn get_vertices(digs: Vec<Dig>) -> Vec<Point2D<i32>> {
        let mut vertices = Vec::new();
        let mut current = Point2D::new(0, 0);

        vertices.push(current);

        for dig in digs {
            current = match dig.direction {
                RelativeDirection::Up => Point2D::new(current.x + dig.amount, current.y),
                RelativeDirection::Right => Point2D::new(current.x, current.y + dig.amount),
                RelativeDirection::Down => Point2D::new(current.x - dig.amount, current.y),
                RelativeDirection::Left => Point2D::new(current.x, current.y - dig.amount),
            };

            vertices.push(current);
        }

        vertices
    }

    /// Round a floating point number to a given amount of decimal places
    fn round(number: f64, decimals: u32) -> f64 {
        let n = 10_u32.pow(decimals);

        (number * (n as f64)).round() / (n as f64)
    }
}

struct Dig {
    amount: i32,
    direction: RelativeDirection,
}

impl Dig {
    fn new(line: &str) -> Dig {
        let split = line.split(' ').collect_vec();
        let direction = match split[0] {
            "U" => RelativeDirection::Up,
            "D" => RelativeDirection::Down,
            "R" => RelativeDirection::Right,
            "L" => RelativeDirection::Left,
            _ => panic!(),
        };

        let amount = split[1].parse::<i32>().unwrap();

        Dig { amount, direction }
    }
}

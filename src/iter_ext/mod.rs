pub trait IterExt : Iterator {
    /// Return the count of elements in self that satisfy the predicate.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use aoc_helper::iter_ext::IterExt;
    /// 
    /// let count = [1, 2, 3, 2, 1].into_iter().count_where(|n| *n < 2 );
    /// assert_eq!(2, count);
    /// 
    /// let a_count = [",", "A", "b", "c", "a", "A", "AA"].into_iter().count_where(|s| *s == "A");
    /// assert_eq!(2, a_count);
    /// ```
    ///
    /// Note, that if we iterate over references, the predicate will receive a double reference:
    /// ```
    /// use aoc_helper::iter_ext::IterExt;
    /// 
    /// let hello = ["Hello", "From", "Count"].iter().count_where(|s| **s == "Hello"); // Needs two *s, beacuse the type of s is &&&s!
    /// assert_eq!(1, hello);
    /// ```
    fn count_where<P>(self, predicate: P) -> usize 
    where
        Self: Sized,
        P: Fn(&Self::Item) -> bool
    {
        self.filter(|s| predicate(s)).count()
    }


    /// Convinience function to collect self into a Vec.
    /// 
    /// # Example:
    /// 
    /// ```
    /// use aoc_helper::iter_ext::IterExt;
    /// 
    /// let vec = [1, 2, 3, 4].into_iter().collect_vec();
    /// assert_eq!(vec![1,2,3,4], vec);
    fn collect_vec(self) -> Vec<Self::Item>
    where
        Self: Sized
    {
        self.collect() 
    }


    /// Convenience function to map and collect an iteratior.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use aoc_helper::iter_ext::IterExt;
    /// 
    /// let double = [1, 2, 3, 4].iter().map_collect_vec(|&v| v * 2);
    /// assert_eq!(double, vec![2,4,6,8]);
    ///
    /// let hello = ["hello", "from", "map", "collect"].iter().map_collect_vec(|s| s.to_uppercase());
    /// assert_eq!(hello, vec!["HELLO", "FROM", "MAP", "COLLECT"]);
    /// ```
    fn map_collect_vec<T>(self, map_fn: impl Fn(Self::Item) -> T) -> Vec<T> 
    where
        Self: Sized,
        T: Sized
    {
        self.map(|e| map_fn(e)).collect_vec()
    }
}

impl<I: Iterator> IterExt for I {}

#[cfg(test)]
mod test_iter_ext {
    use super::IterExt;

    #[test]
    fn count_where_works_int() {
        let one = [1, 2, 3, 2, 1].into_iter().count_where(|n| *n < 2 );
        assert_eq!(2, one);

        let ten = [1, 2, 3, 4, 5, 10, 11, 23, 12, 2, 10].into_iter().count_where(|n| *n == 10);
        assert_eq!(2, ten);

        let string = [",", "A", "b", "c", "a", "A", "AA"].into_iter().count_where(|s| *s == "A");
        assert_eq!(2, string);

        let hello = ["Hello", "From", "Count"].iter().count_where(|s| **s == "Hello"); // Needs to *s, beacuse the type of s is &&&s!
        assert_eq!(1, hello);
    }

    #[test]
    fn collect_vec_works() {
        assert_eq!([1, 2, 3, 4].into_iter().collect_vec(), vec![1, 2, 3, 4]);
        assert_eq!(["A", "B", "C", "D", "E"].into_iter().collect_vec(), vec!["A", "B", "C", "D", "E"]);

        let vec = [1, 2, 3, 4].into_iter().collect_vec();
        assert_eq!(vec![1,2,3,4], vec);
    }

    #[test]
    fn map_collect_vec_works() {
        let double = [1, 2, 3, 4].iter().map_collect_vec(|&v| v * 2);
        assert_eq!(double, vec![2,4,6,8]);

        let hello = ["hello", "from", "map", "collect"].iter().map_collect_vec(|s| s.to_uppercase());
        assert_eq!(hello, vec!["HELLO", "FROM", "MAP", "COLLECT"]);
    }
}
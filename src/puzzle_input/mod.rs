#![allow(dead_code)]
use std::{error::Error, fs, str::FromStr, fmt::Display};

/// Trait that indicates that the type implementing it is the puzzle input.
/// Types implementing [`PuzzleInput`] must also implement [`FromStr`].
pub trait PuzzleInput : FromStr {}


/// Helper function that uses [`FromStr`] to construct an instance of [`PuzzleInput`].
///
/// # Arguments
/// 
/// * `from_str` - A string slice that specifies the input file to be read.
/// 
/// # Errors
///
/// This function will return an error if the file_path argument does not point to a valid file, or if the parsing fails.
///
/// # Examples
///
/// ```
/// use aoc_helper::puzzle_input::{read_puzzle_input, PuzzleInput, PuzzleParseError};
/// use core::str::FromStr;
/// 
/// struct Puzzle {
///     value: usize
/// }
/// 
/// impl FromStr for Puzzle {
///     type Err = PuzzleParseError;
/// 
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         let val = s.parse::<usize>().unwrap();
///         Ok(Puzzle {value: val})
///     }
/// }
/// 
/// impl PuzzleInput for Puzzle {}
/// 
/// // This will read the file 'input.txt', and attempt to parse its content using Puzzle::from_str
/// // let puzzle = read_puzzle_input::<Puzzle>("input.txt").unwrap();
/// 
/// ```
pub fn read_puzzle_input<T>(file_path: &str) -> Result<T, PuzzleParseError>
where
    T: PuzzleInput
{
    let input = fs::read_to_string(file_path).map_err(|err| PuzzleParseError {msg: err.to_string()})?;

    T::from_str(&input).map_err(|_| PuzzleParseError {msg: String::from("Could not parse str!")})
}


/// Convenience function that reads in a [`PuzzleInput`] from "example.txt".
///
/// # Errors
///
/// This function will return an error if there is no "example.txt", or if the parsing fails.
pub fn read_example<T>() -> Result<T, PuzzleParseError>
where
    T: PuzzleInput
{
    read_puzzle_input("example.txt")
}

/// Convenience function that reads in a [`PuzzleInput`] from "input.txt".
///
/// # Errors
///
/// This function will return an error if there is no "input.txt", or if the parsing fails.
pub fn read_input<T>() -> Result<T, PuzzleParseError>
where
    T: PuzzleInput
{
    read_puzzle_input("input.txt")
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuzzleParseError {
    msg: String
}

impl Display for PuzzleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for PuzzleParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

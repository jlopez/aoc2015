use std::cmp::{max, min};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// The regex used to parse the input.
    ///
    /// The regex has 5 capture groups:
    /// 1. The operation to perform.
    /// 2. The x-coordinate of the first light to update.
    /// 3. The y-coordinate of the first light to update.
    /// 4. The x-coordinate of the last light to update (inclusive).
    /// 5. The y-coordinate of the last light to update (inclusive).
    ///
    /// The end coordinates are converted into exclusive coordinates by the
    /// iterator parsing the input.
    static ref EX06_REGEX: Regex = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
}

/// An iterator over the lines of the input.
struct ParserIterator<'a> {
    /// The lines iterator of the input.
    lines: std::str::Lines<'a>,
}

impl <'a> ParserIterator<'a> {
    /// Creates a new [`ParserIterator`] over the provided input.
    ///
    /// # Arguments
    /// * `input` - The input to parse.
    ///
    /// # Returns
    /// A new [`ParserIterator`] over the provided input.
    ///
    /// # Examples
    /// ```
    /// use aoc2015::ex06::ParserIterator;
    /// let input = "turn on 0,0 through 1,1";
    /// let mut parser_iterator = ParserIterator::new(input);
    /// assert_eq!(parser_iterator.next(), Some(("turn on", 0, 0, 2, 2)));
    /// assert_eq!(parser_iterator.next(), None);
    /// ```
    fn new(input: &'a str) -> Self {
        ParserIterator { lines: input.lines() }
    }
}

/// An iterator over the parsed lines of the input.
///
/// Each item is a tuple of the operation and the coordinates.
/// The coordinates are 0-indexed, and the second pair is exclusive.
/// For example, the line `turn on 0,0 through 1,1` will be parsed as
/// `("turn on", 0, 0, 2, 2)`.
///
/// # Panics
/// Panics if the input is malformed.
impl <'a> Iterator for ParserIterator<'a> {
    type Item = (&'a str, usize, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let parts: Vec<_> = EX06_REGEX.captures(line).unwrap().iter().skip(1).map(|p| p.unwrap().as_str()).collect::<_>();
        Some((parts[0],
              parts[1].parse().unwrap(),
              parts[2].parse().unwrap(),
              parts[3].parse::<usize>().unwrap() + 1,
              parts[4].parse::<usize>().unwrap() + 1))
    }
}

/// A grid of lights.
///
/// The grid is stored as a flat vector of bitfields, where each bit represents a light.
/// The bitfields are stored in row-major order, with each bitfield representing a row.
/// The bitfields are stored in little-endian order, so the least significant bit represents
/// the leftmost light.
/// For example, the grid
/// ```text
/// 0 1 0 1
/// 1 0 0 1
/// 0 0 1 0
/// 1 1 1 1
/// ```
/// would be stored as
/// ```text
/// [0b1010, 0b1001, 0b0010, 0b1111]
/// ```
struct Grid {
    /// The bitfields of this grid as a flat vector.
    grid: Vec<u128>,
    /// The height of this grid.
    height: usize,
    /// The width of this grid in 128-bit words.
    width: usize,
}

/// An operation to perform on a [`Grid`].
#[derive(Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

/// A grid of lights.
impl Grid {
    /// Creates a new [`Grid`] of the provided size.
    ///
    /// # Arguments
    /// * `width` - The width of the grid.
    /// * `height` - The height of the grid.
    ///
    /// # Returns
    /// A new [`Grid`] of the provided size.
    ///
    /// # Examples
    /// ```
    /// use aoc2015::ex06::Grid;
    /// let grid = Grid::new(1000, 1000);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if either `width` or `height` is 0.
    fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "Invalid dimensions");
        let width = (width - 1) / 128 + 1;
        Grid { grid: vec![0; width * height], width, height }
    }

    /// Updates this grid according to the provided [`Op`] and coordinates.
    ///
    /// # Arguments
    /// * `op` - The [`Op`] to perform.
    /// * `x1` - The x-coordinate of the first light to update.
    /// * `y1` - The y-coordinate of the first light to update.
    /// * `x2` - The x-coordinate of the last light to update (exclusive).
    /// * `y2` - The y-coordinate of the last light to update (exclusive).
    ///
    /// # Examples
    /// ```
    /// use aoc2015::ex06::{Grid, Op};
    /// let mut grid = Grid::new(1000, 1000);
    /// grid.update(Op::On, 0, 0, 2, 2);
    /// ```
    fn update(&mut self, op: Op, x1: usize, y1: usize, x2: usize, y2: usize) {
        if x1 >= x2 || y1 >= y2 { return; }
        let start_y_index = max(0, y1) * self.width;
        let end_y_index = min(self.height, y2) * self.width;
        let start_x_index = max(0, x1 / 128);
        let end_x_index = min(self.width, x2 / 128);
        for x_index in start_x_index..=end_x_index {
            let start_bit = if x1 < x_index * 128 { 0 } else { x1 - x_index * 128 };
            let end_bit = x2 - x_index * 128;
            let start_bit = 1u128 << start_bit;
            let end_bit = if end_bit < 128 { 1u128 << end_bit } else { 0 };
            let mask = u128::wrapping_sub(end_bit, start_bit);
            for y_index in (start_y_index..end_y_index).step_by(self.width) {
                let bitfield = &mut self.grid[y_index + x_index];
                match op {
                    Op::On => *bitfield |= mask,
                    Op::Off => *bitfield &= !mask,
                    Op::Toggle => *bitfield ^= mask,
                }
            }
        }
    }

    /// Returns the number of lights turned on in this [`Grid`].
    ///
    /// # Examples
    /// ```
    /// use aoc2015::ex06::{Grid, Op};
    /// let mut grid = Grid::new(1000, 1000);
    /// grid.update(Op::On, 0, 0, 2, 2);
    /// assert_eq!(grid.count(), 4);
    /// ```
    fn count(&self) -> u32 {
        self.grid.iter().map(|bitfield| bitfield.count_ones()).sum()
    }

}

impl core::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.chunks(self.width) {
            for bitfield in row {
                let bitfield = format!("{:032X}", bitfield.reverse_bits()).replace("0", " ");
                write!(f, "{bitfield}|")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// 6th day of Advent of Code 2015
//
// https://adventofcode.com/2015/day/6
//
// This is a solution to the first part of the puzzle.
// The solution is found by parsing the input into a grid of lights,
// then updating the grid according to the instructions.
pub fn a(input: &str) -> u32 {
    let mut grid = Grid::new(1000, 1000);
    for line in ParserIterator::new(input) {
        match line {
            ("turn on", x1, y1, x2, y2) => grid.update(Op::On, x1, y1, x2, y2),
            ("turn off", x1, y1, x2, y2) => grid.update(Op::Off, x1, y1, x2, y2),
            ("toggle", x1, y1, x2, y2) => grid.update(Op::Toggle, x1, y1, x2, y2),
            _ => unreachable!(),
        }
    }
    grid.count()
}

pub fn b(_input: &str) -> u32 {
    0
}

/// A somewhat unified interface for the Advent of Code problems.
/// The lifetime `'a` is used to make sure that the input can be borrowed from.
/// This allows you, e.g., to make `Self::Part1Input` a `&'a str`, to borrow from the input.
pub trait AdventOfCodeDay<'a> {
    /// The result of parsing your input, can be whatever you want to store the parsed input as.
    /// If you need to parse the input differently for part 1 and part 2, you can use a tuple here.
    type ParsedInput;

    /// The type of the output for part 1, usually a number.
    /// Sadly AoC solutions are not always numbers. We will use [`u64`] as the default for numbers, and [`String`] for text answers.
    type Part1Output;
    /// The type of the output for part 2, usually a number.
    /// Sadly AoC solutions are not always numbers. We will use [`u64`] as the default for numbers, and [`String`] for text answers.
    type Part2Output;

    /// Solve part 1 of the problem.
    /// You will get the input as a `&str`.
    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output;
    /// Solve part 2 of the problem.
    /// You will get the input as a `&str`.
    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output;
    /// Parse the input into a format that can be used by the solver.
    /// If you make `Self::ParsedInput` a type that has a lifetime of `'a`, then you cam borrow from the input.
    fn parse_input(input: &'a str) -> Self::ParsedInput;
}

/// A somewhat unified interface for the Advent of Code problems.
/// The lifetime `'a` is used to make sure that the input can be borrowed from.
/// This allows you, e.g., to make `Self::Part1Input` a `&'a str`, to borrow from the input.
pub trait AdventOfCodeDay<'a> {
    /// The type of the input for part 1, can be whatever you want to store the parsed input as.
    /// Needs to implement [`Clone`], in case you want to reuse the input for part 2.
    type Part1Input: Clone;

    /// The type of the input for part 1, can be whatever you want to store the parsed input as.
    /// If you want to reuse the input for phase1 for phase2, this needs to be the same as [`Self::Part1Input`].
    type Part2Input;

    /// The type of the output for part 1, usually a number.
    /// Sadly AoC solutions are not always numbers. We will use [`u64`] as the default for numbers, and [`String`] for text answers.
    type Part1Output;
    /// The type of the output for part 2, usually a number.
    /// Sadly AoC solutions are not always numbers. We will use [`u64`] as the default for numbers, and [`String`] for text answers.
    type Part2Output;

    /// Solve part 1 of the problem.
    /// You will get the input as a `&str`.
    fn solve_part1(input: Self::Part1Input) -> Self::Part1Output;
    /// Solve part 2 of the problem.
    /// You will get the input as a `&str`.
    fn solve_part2(input: Self::Part2Input) -> Self::Part2Output;
    /// Parse the input into a format that can be used by the solver.
    /// This can be used to pre-process the input and save some information in `self`, which can later be used by the solvers.
    fn parse_input(input: &'a str) -> Self::Part1Input;
    /// If you have a different parsed format for part 2, you can use this to parse it.
    /// If not, then just return `None`.
    #[allow(unused_variables)]
    fn parse_input_part2(input: &'a str) -> Option<Self::Part2Input> {
        None
    }
}

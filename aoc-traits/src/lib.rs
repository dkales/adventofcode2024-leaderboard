use std::fmt::Display;

/// A somewhat unified interface for the Advent of Code problems.
/// The lifetime `'a` is used to make sure that the input can be borrowed from.
/// This allows you, e.g., to make `Self::Part1Input` a `&'a str`, to borrow from the input.
pub trait AdventOfCodeDay<'a> {
    /// The result of parsing your input, can be whatever you want to store the parsed input as.
    /// If you need to parse the input differently for part 1 and part 2, you can use a tuple here.
    type ParsedInput;

    /// The type of the output for part 1, usually a number.
    /// Sadly AoC solutions are not always numbers. Usually use [`u64`] as the default for numbers, and [`String`] for text answers.
    type Part1Output: Display;
    /// The type of the output for part 2, usually a number.
    /// Sadly AoC solutions are not always numbers. Usually use [`u64`] as the default for numbers, and [`String`] for text answers.
    type Part2Output: Display;

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

// a default impl that panics on all methods
impl AdventOfCodeDay<'_> for () {
    type ParsedInput = ();

    type Part1Output = &'static str;

    type Part2Output = &'static str;

    fn solve_part1(_input: &Self::ParsedInput) -> Self::Part1Output {
        unimplemented!()
    }

    fn solve_part2(_input: &Self::ParsedInput) -> Self::Part2Output {
        unimplemented!()
    }

    fn parse_input(_input: &'_ str) -> Self::ParsedInput {
        unimplemented!()
    }
}

pub fn run_day<'a, Day: AdventOfCodeDay<'a>>(input: &'a str) {
    let input = input.trim();
    let parsed_input = Day::parse_input(input);
    let stage1_solution = Day::solve_part1(&parsed_input);
    println!("Stage 1: {}", stage1_solution.to_string());
    let stage2_solution = Day::solve_part2(&parsed_input);
    println!("Stage 2: {}", stage2_solution.to_string());
}

pub trait AdventOfCodeSolutions<'a> {
    type Day01: AdventOfCodeDay<'a>;
    type Day02: AdventOfCodeDay<'a>;
    type Day03: AdventOfCodeDay<'a>;
    type Day04: AdventOfCodeDay<'a>;
    type Day05: AdventOfCodeDay<'a>;
    type Day06: AdventOfCodeDay<'a>;
    type Day07: AdventOfCodeDay<'a>;
    type Day08: AdventOfCodeDay<'a>;
    type Day09: AdventOfCodeDay<'a>;
    type Day10: AdventOfCodeDay<'a>;
    type Day11: AdventOfCodeDay<'a>;
    type Day12: AdventOfCodeDay<'a>;
    type Day13: AdventOfCodeDay<'a>;
    type Day14: AdventOfCodeDay<'a>;
    type Day15: AdventOfCodeDay<'a>;
    type Day16: AdventOfCodeDay<'a>;
    type Day17: AdventOfCodeDay<'a>;
    type Day18: AdventOfCodeDay<'a>;
    type Day19: AdventOfCodeDay<'a>;
    type Day20: AdventOfCodeDay<'a>;
    type Day21: AdventOfCodeDay<'a>;
    type Day22: AdventOfCodeDay<'a>;
    type Day23: AdventOfCodeDay<'a>;
    type Day24: AdventOfCodeDay<'a>;
    type Day25: AdventOfCodeDay<'a>;

    fn solve_day(day: usize, input: &'a str) -> Result<(), String> {
        let input = input.trim();
        match day {
            1 => run_day::<Self::Day01>(input),
            2 => run_day::<Self::Day02>(input),
            3 => run_day::<Self::Day03>(input),
            4 => run_day::<Self::Day04>(input),
            5 => run_day::<Self::Day05>(input),
            6 => run_day::<Self::Day06>(input),
            7 => run_day::<Self::Day07>(input),
            8 => run_day::<Self::Day08>(input),
            9 => run_day::<Self::Day09>(input),
            10 => run_day::<Self::Day10>(input),
            11 => run_day::<Self::Day11>(input),
            12 => run_day::<Self::Day12>(input),
            13 => run_day::<Self::Day13>(input),
            14 => run_day::<Self::Day14>(input),
            15 => run_day::<Self::Day15>(input),
            16 => run_day::<Self::Day16>(input),
            17 => run_day::<Self::Day17>(input),
            18 => run_day::<Self::Day18>(input),
            19 => run_day::<Self::Day19>(input),
            20 => run_day::<Self::Day20>(input),
            21 => run_day::<Self::Day21>(input),
            22 => run_day::<Self::Day22>(input),
            23 => run_day::<Self::Day23>(input),
            24 => run_day::<Self::Day24>(input),
            25 => run_day::<Self::Day25>(input),
            _ => return Err(format!("Day {} not part of AoC", day)),
        }
        Ok(())
    }
}

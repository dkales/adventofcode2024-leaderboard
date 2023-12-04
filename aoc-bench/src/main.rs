use aoc_traits::AdventOfCodeSolutions;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_aoc<S: for<'a> AdventOfCodeSolutions<'a>>() {}

fn main() {
    bench_aoc::<dkales_aoc::AoC2023>();
}

use std::{
    panic::{self},
    sync::mpsc,
    thread,
    time::Duration,
};

use aoc_traits::{AdventOfCodeDay, AdventOfCodeSolutions};
use criterion::{black_box, Criterion};

fn bench_aoc_day<S: AdventOfCodeDay<'static>>(
    c: &mut Criterion,
    username: &str,
    day: u8,
    input: &'static str,
    expected_stage1: &'static str,
    expected_stage2: &'static str,
) -> std::thread::Result<()> {
    println!("Benchmarking user {}, day{:02}", username, day);
    // give the user's code 60 seconds to run
    let (sender, receiver) = mpsc::channel();
    let t = thread::spawn(move || {
        let res = panic::catch_unwind(|| {
            let input = input.trim();
            let parsed_input = S::parse_input(black_box(input));
            let stage1_solution = S::solve_part1(&parsed_input);
            assert_eq!(stage1_solution.to_string(), expected_stage1);
            let stage2_solution = S::solve_part2(&parsed_input);
            assert_eq!(stage2_solution.to_string(), expected_stage2);
        });
        sender.send(res).unwrap();
    });
    let _ = match receiver.recv_timeout(Duration::from_secs(60)) {
        Ok(x) => x?,
        Err(_) => return Err(Box::new("timeout")),
    };
    let _ = t.join();

    let input = input.trim();
    let parsed_input = S::parse_input(black_box(input));
    c.bench_function(&format!("{username}-day{day:02}-parse"), |b| {
        b.iter(|| {
            black_box(S::parse_input(black_box(input)));
        })
    });
    c.bench_function(&format!("{username}-day{day:02}-part1"), |b| {
        b.iter(|| {
            black_box(S::solve_part1(black_box(&parsed_input)));
        })
    });
    c.bench_function(&format!("{username}-day{day:02}-part2"), |b| {
        b.iter(|| {
            black_box(S::solve_part2(black_box(&parsed_input)));
        })
    });
    Ok(())
}

fn bench_aoc<S: AdventOfCodeSolutions>(c: &mut Criterion, username: &str) {
    for (day, input, out1, out2) in INPUTS_OUTPUTS {
        let result = match day {
            1 => bench_aoc_day::<S::Day01>(c, username, day, input, out1, out2),
            2 => bench_aoc_day::<S::Day02>(c, username, day, input, out1, out2),
            3 => bench_aoc_day::<S::Day03>(c, username, day, input, out1, out2),
            4 => bench_aoc_day::<S::Day04>(c, username, day, input, out1, out2),
            5 => bench_aoc_day::<S::Day05>(c, username, day, input, out1, out2),
            6 => bench_aoc_day::<S::Day06>(c, username, day, input, out1, out2),
            7 => bench_aoc_day::<S::Day07>(c, username, day, input, out1, out2),
            8 => bench_aoc_day::<S::Day08>(c, username, day, input, out1, out2),
            9 => bench_aoc_day::<S::Day09>(c, username, day, input, out1, out2),
            10 => bench_aoc_day::<S::Day10>(c, username, day, input, out1, out2),
            11 => bench_aoc_day::<S::Day11>(c, username, day, input, out1, out2),
            12 => bench_aoc_day::<S::Day12>(c, username, day, input, out1, out2),
            13 => bench_aoc_day::<S::Day13>(c, username, day, input, out1, out2),
            14 => bench_aoc_day::<S::Day14>(c, username, day, input, out1, out2),
            15 => bench_aoc_day::<S::Day15>(c, username, day, input, out1, out2),
            16 => bench_aoc_day::<S::Day16>(c, username, day, input, out1, out2),
            17 => bench_aoc_day::<S::Day17>(c, username, day, input, out1, out2),
            18 => bench_aoc_day::<S::Day18>(c, username, day, input, out1, out2),
            19 => bench_aoc_day::<S::Day19>(c, username, day, input, out1, out2),
            20 => bench_aoc_day::<S::Day20>(c, username, day, input, out1, out2),
            21 => bench_aoc_day::<S::Day21>(c, username, day, input, out1, out2),
            22 => bench_aoc_day::<S::Day22>(c, username, day, input, out1, out2),
            23 => bench_aoc_day::<S::Day23>(c, username, day, input, out1, out2),
            24 => bench_aoc_day::<S::Day24>(c, username, day, input, out1, out2),
            25 => bench_aoc_day::<S::Day25>(c, username, day, input, out1, out2),
            _ => unreachable!(),
        };
        if let Err(e) = result {
            if let Some(error_msg) = e.downcast_ref::<&str>() {
                println!(
                    "Error running benchmark for user {}, day {:02}: {}",
                    username, day, error_msg,
                );
            } else if let Ok(error_msg) = e.downcast::<Box<dyn std::error::Error>>() {
                println!(
                    "Error running benchmark for user {}, day {:02}: {}",
                    username, day, error_msg,
                );
            } else {
                println!(
                    "Error running benchmark for user {}, day {:02}: {}",
                    username, day, "unkown error",
                );
            }
        }
    }
}

fn benches() {
    let mut c = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1))
        .without_plots();
    bench_aoc::<dkales_aoc::AoC2023>(&mut c, "dkales");
    bench_aoc::<fnieddu_aoc::AoC2023>(&mut c, "fnieddu");
    bench_aoc::<fabian_aoc::AoC2023>(&mut c, "fabian1409");
    bench_aoc::<simon_aoc::AoC2023>(&mut c, "devise");
}

fn main() {
    benches();
    Criterion::default().final_summary();
}

const INPUTS_OUTPUTS: [(u8, &'static str, &'static str, &'static str); 25] = [
    (
        1,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01.txt")),
        "56397",
        "55701",
    ),
    (
        2,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02.txt")),
        "2913",
        "55593",
    ),
    (
        3,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03.txt")),
        "554003",
        "87263515",
    ),
    (
        4,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day04.txt")),
        "21485",
        "11024379",
    ),
    (
        5,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day05.txt")),
        "174137457",
        "1493866",
    ),
    (
        6,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day06.txt")),
        "",
        "",
    ),
    (
        7,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day07.txt")),
        "",
        "",
    ),
    (
        8,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08.txt")),
        "",
        "",
    ),
    (
        9,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day09.txt")),
        "",
        "",
    ),
    (
        10,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day10.txt")),
        "",
        "",
    ),
    (
        11,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day11.txt")),
        "",
        "",
    ),
    (
        12,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day12.txt")),
        "",
        "",
    ),
    (
        13,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day13.txt")),
        "",
        "",
    ),
    (
        14,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day14.txt")),
        "",
        "",
    ),
    (
        15,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day15.txt")),
        "",
        "",
    ),
    (
        16,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day16.txt")),
        "",
        "",
    ),
    (
        17,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day17.txt")),
        "",
        "",
    ),
    (
        18,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day18.txt")),
        "",
        "",
    ),
    (
        19,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day19.txt")),
        "",
        "",
    ),
    (
        20,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day20.txt")),
        "",
        "",
    ),
    (
        21,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day21.txt")),
        "",
        "",
    ),
    (
        22,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day22.txt")),
        "",
        "",
    ),
    (
        23,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day23.txt")),
        "",
        "",
    ),
    (
        24,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day24.txt")),
        "",
        "",
    ),
    (
        25,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day25.txt")),
        "",
        "",
    ),
];

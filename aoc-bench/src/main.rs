use std::{
    panic::{self},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use aoc_traits::{AdventOfCodeDay, AdventOfCodeSolutions};
use criterion::{black_box, BatchSize, Criterion};

enum ExecutionError {
    Timeout,
    WrongAnswer,
    NotImplemented,
    Panic,
}

fn bench_aoc_day<S: AdventOfCodeDay + 'static>(
    username: &str,
    day: u8,
    input: &'static [u8],
    expected_stage1: &'static str,
    expected_stage2: &'static str,
) -> (
    Result<(), ExecutionError>,
    Result<(), ExecutionError>,
    Result<(), ExecutionError>,
) {
    let key = std::env::var("AGE_PASSPHRASE")
        .expect("need AGE_PASSPHRASE to be set")
        .into();
    let identity = age::scrypt::Identity::new(key);
    let input_dec = String::from_utf8(age::decrypt(&identity, &input).expect("can decrypt input"))
        .expect("input is utf8");
    println!("Benchmarking user {}, day{:02}", username, day);
    if core::any::TypeId::of::<S>() == core::any::TypeId::of::<()>() {
        return (
            Err(ExecutionError::NotImplemented),
            Err(ExecutionError::NotImplemented),
            Err(ExecutionError::NotImplemented),
        );
    }
    // give the user's code 60 seconds to run

    // check if the parser is implemented and takes less than 1 second
    let (sender, receiver) = mpsc::channel();
    let input = input_dec.clone();
    let t = thread::spawn(move || {
        let res = panic::catch_unwind(move || {
            let input = input.trim();
            let _parsed_input = S::parse_input(input);
            ()
        });
        let _ = sender.send(res);
    });
    let parse_result = match receiver.recv_timeout(Duration::from_secs(1)) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(_)) => Err(ExecutionError::Panic),
        Err(_) => Err(ExecutionError::Timeout),
    };
    // if the parser timed out, we can't run the other benchmarks
    if let Err(ExecutionError::Timeout) = parse_result {
        return (
            Err(ExecutionError::Timeout),
            Err(ExecutionError::Timeout),
            Err(ExecutionError::Timeout),
        );
    } else if let Err(ExecutionError::Panic) = parse_result {
        // if the parser panicked, we can't run the other benchmarks
        return (
            Err(ExecutionError::Panic),
            Err(ExecutionError::Panic),
            Err(ExecutionError::Panic),
        );
    }
    let _ = t.join();

    let mut c = Criterion::default()
        .sample_size(100)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1))
        .without_plots();
    let input = input_dec.clone();
    c.bench_function(&format!("{username}-day{day:02}-parse"), |b| {
        let trimmed_input = input.trim();
        b.iter(move || {
            black_box(S::parse_input(black_box(trimmed_input)));
        })
    });

    let start = Instant::now();
    // check if part1 is implemented and takes less than 10 second
    let (sender, receiver) = mpsc::channel();
    let input = input_dec.clone();
    let t = thread::spawn(move || {
        let res = panic::catch_unwind(|| {
            let input = input.trim();
            let parsed_input = S::parse_input(input);
            let stage1 = S::solve_part1(black_box(&parsed_input));
            if stage1.to_string() != expected_stage1 {
                return Err(ExecutionError::WrongAnswer);
            }
            Ok(())
        });
        let _ = sender.send(res);
    });
    let part1_result = match receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(Ok(x)) => x,
        Ok(Err(e)) => {
            if let Some(msg) = e.downcast_ref::<&str>() {
                if msg.contains("not yet implemented") {
                    Err(ExecutionError::NotImplemented)
                } else {
                    Err(ExecutionError::Panic)
                }
            } else {
                Err(ExecutionError::Panic)
            }
        }
        Err(_) => Err(ExecutionError::Timeout),
    };
    let dur_part1 = start.elapsed();
    if matches!(part1_result, Ok(())) {
        let _ = t.join();

        let mut c = Criterion::default()
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(1))
            .without_plots();
        c = if dur_part1 > Duration::from_millis(100) && dur_part1 < Duration::from_secs(1) {
            c.sample_size(50)
        } else if dur_part1 > Duration::from_secs(1) {
            c.sample_size(10)
        } else {
            c.sample_size(100)
        };

        let input = input_dec.clone();
        c.bench_function(&format!("{username}-day{day:02}-part1"), |b| {
            let trimmed_input = input.trim();
            b.iter_batched_ref(
                || {
                    let parsed_input = S::parse_input(black_box(trimmed_input));
                    parsed_input
                },
                |parsed_input| {
                    black_box(S::solve_part1(parsed_input));
                },
                BatchSize::LargeInput,
            )
        });
    }
    let start = Instant::now();
    // check if part2 is implemented and takes less than 30 second
    let (sender, receiver) = mpsc::channel();
    let input = input_dec.clone();
    let t = thread::spawn(move || {
        let res = panic::catch_unwind(|| {
            let input = input.trim();
            let parsed_input = S::parse_input(input);
            // also re-do part1, since it might change the input
            let _stage1 = S::solve_part1(black_box(&parsed_input));
            let stage2 = S::solve_part2(black_box(&parsed_input));
            if stage2.to_string() != expected_stage2 {
                return Err(ExecutionError::WrongAnswer);
            }
            Ok(())
        });
        let _ = sender.send(res);
    });
    let part2_result = match receiver.recv_timeout(Duration::from_secs(30)) {
        Ok(Ok(x)) => x,
        Ok(Err(e)) => {
            if let Some(msg) = e.downcast_ref::<&str>() {
                println!("msg: {}", msg);
                if msg.contains("not yet implemented") {
                    Err(ExecutionError::NotImplemented)
                } else {
                    Err(ExecutionError::Panic)
                }
            } else {
                Err(ExecutionError::Panic)
            }
        }
        Err(_) => Err(ExecutionError::Timeout),
    };
    let dur_part2 = start.elapsed();
    if matches!(part2_result, Ok(())) {
        let _ = t.join();
        let mut c = Criterion::default()
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(1))
            .without_plots();
        c = if dur_part2 > Duration::from_millis(100) && dur_part2 < Duration::from_secs(1) {
            c.sample_size(50)
        } else if dur_part2 > Duration::from_secs(1) {
            c.sample_size(10)
        } else {
            c.sample_size(100)
        };
        let input = input_dec.clone();
        c.bench_function(&format!("{username}-day{day:02}-part2"), |b| {
            let trimmed_input = input.trim();
            b.iter_batched_ref(
                || {
                    let parsed_input = S::parse_input(black_box(trimmed_input));
                    let _stage1 = S::solve_part1(&parsed_input);
                    parsed_input
                },
                |parsed_input| {
                    black_box(S::solve_part2(parsed_input));
                },
                criterion::BatchSize::LargeInput,
            )
        });
        c.bench_function(&format!("{username}-day{day:02}-Total"), |b| {
            let trimmed_input = input.trim();
            b.iter(|| {
                let parsed_input = S::parse_input(trimmed_input);
                (S::solve_part1(&parsed_input), S::solve_part2(&parsed_input));
            })
        });
    }
    (parse_result, part1_result, part2_result)
}

fn bench_aoc<S: AdventOfCodeSolutions + 'static>(username: &str) {
    for (day, input, out1, out2) in INPUTS_OUTPUTS {
        let result = match day {
            1 => bench_aoc_day::<S::Day01>(username, day, input, out1, out2),
            2 => bench_aoc_day::<S::Day02>(username, day, input, out1, out2),
            3 => bench_aoc_day::<S::Day03>(username, day, input, out1, out2),
            4 => bench_aoc_day::<S::Day04>(username, day, input, out1, out2),
            5 => bench_aoc_day::<S::Day05>(username, day, input, out1, out2),
            6 => bench_aoc_day::<S::Day06>(username, day, input, out1, out2),
            7 => bench_aoc_day::<S::Day07>(username, day, input, out1, out2),
            8 => bench_aoc_day::<S::Day08>(username, day, input, out1, out2),
            9 => bench_aoc_day::<S::Day09>(username, day, input, out1, out2),
            10 => bench_aoc_day::<S::Day10>(username, day, input, out1, out2),
            11 => bench_aoc_day::<S::Day11>(username, day, input, out1, out2),
            12 => bench_aoc_day::<S::Day12>(username, day, input, out1, out2),
            13 => bench_aoc_day::<S::Day13>(username, day, input, out1, out2),
            14 => bench_aoc_day::<S::Day14>(username, day, input, out1, out2),
            15 => bench_aoc_day::<S::Day15>(username, day, input, out1, out2),
            16 => bench_aoc_day::<S::Day16>(username, day, input, out1, out2),
            17 => bench_aoc_day::<S::Day17>(username, day, input, out1, out2),
            18 => bench_aoc_day::<S::Day18>(username, day, input, out1, out2),
            19 => bench_aoc_day::<S::Day19>(username, day, input, out1, out2),
            20 => bench_aoc_day::<S::Day20>(username, day, input, out1, out2),
            21 => bench_aoc_day::<S::Day21>(username, day, input, out1, out2),
            22 => bench_aoc_day::<S::Day22>(username, day, input, out1, out2),
            23 => bench_aoc_day::<S::Day23>(username, day, input, out1, out2),
            24 => bench_aoc_day::<S::Day24>(username, day, input, out1, out2),
            25 => bench_aoc_day::<S::Day25>(username, day, input, out1, out2),
            _ => unreachable!(),
        };
        if let Err(e) = &result.0 {
            print!("{username}-day{day:02}-parse: ");

            match e {
                ExecutionError::Timeout => println!("timeout"),
                ExecutionError::WrongAnswer => println!("wrong answer"),
                ExecutionError::NotImplemented => println!("not implemented"),
                ExecutionError::Panic => println!("panicked"),
            }
        }
        if let Err(e) = &result.1 {
            print!("{username}-day{day:02}-part1: ");

            match e {
                ExecutionError::Timeout => println!("timeout"),
                ExecutionError::WrongAnswer => println!("wrong answer"),
                ExecutionError::NotImplemented => println!("not implemented"),
                ExecutionError::Panic => println!("panicked"),
            }
        }
        if let Err(e) = &result.2 {
            print!("{username}-day{day:02}-part2: ");

            match e {
                ExecutionError::Timeout => println!("timeout"),
                ExecutionError::WrongAnswer => println!("wrong answer"),
                ExecutionError::NotImplemented => println!("not implemented"),
                ExecutionError::Panic => println!("panicked"),
            }
        }
        match result {
            (Ok(()), Ok(()), Ok(())) => {}
            (
                Err(ExecutionError::NotImplemented),
                Err(ExecutionError::NotImplemented),
                Err(ExecutionError::NotImplemented),
            ) => {
                println!("{username}-day{day:02}-Total: not implemented");
            }
            _ => {
                println!("{username}-day{day:02}-Total: error");
            }
        }
    }
}

fn benches() {
    bench_aoc::<dkales_aoc::AoC2024>("dkales");
    bench_aoc::<franco_aoc::AoC2024>("franco");
    bench_aoc::<fabian_aoc::AoC2024>("fabian1409");
    bench_aoc::<simon_aoc::AoC2024>("devise");
}

fn main() {
    benches();
    Criterion::default().final_summary();
}

const INPUTS_OUTPUTS: [(u8, &'static [u8], &'static str, &'static str); 6] = [
    (
        1,
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01.txt.age")),
        "1319616",
        "27267728",
    ),
    (
        2,
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02.txt.age")),
        "680",
        "710",
    ),
    (
        3,
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03.txt.age")),
        "184511516",
        "90044227",
    ),
    (
        4,
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day04.txt.age")),
        "2427",
        "1900",
    ),
    (
        5,
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day05.txt.age")),
        "4957",
        "6938",
    ),
    (
        6,
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day06.txt.age")),
        "4964",
        "1740",
    ),
    // (
    //     7,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day07.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     8,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     9,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day09.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     10,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day10.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     11,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day11.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     12,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day12.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     13,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day13.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     14,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day14.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     15,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day15.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     16,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day16.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     17,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day17.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     18,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day18.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     19,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day19.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     20,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day20.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     21,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day21.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     22,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day22.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     23,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day23.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     24,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day24.txt.age")),
    //     "",
    //     "",
    // ),
    // (
    //     25,
    //     include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day25.txt.age")),
    //     "",
    //     "",
    // ),
];

use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use clap::Parser;
use color_eyre::eyre::{Context, Result};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

#[derive(Parser)]
struct Args {
    /// the directory containing the criterion output
    #[clap(short, long)]
    criterion_dir: PathBuf,
    /// the log file from the benchmark run
    #[clap(short, long)]
    logfile: PathBuf,
}

#[derive(Default)]
struct AoCBenchmarks {
    days: BTreeMap<u8, AoCBenchmarkDay>,
}

#[derive(Default)]
struct AoCBenchmarkDay {
    phases: BTreeMap<String, AoCBenchmarkPhase>,
}

#[derive(Default)]
struct AoCBenchmarkPhase {
    median_for_user: BTreeMap<String, f64>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if !args.criterion_dir.is_dir() {
        return Err(color_eyre::eyre::eyre!("criterion_dir is not a directory"));
    }
    let log = std::fs::read_to_string(&args.logfile)?;

    let mut benchmarks = AoCBenchmarks::default();
    let mut users: BTreeSet<String> = BTreeSet::new();

    for entry in std::fs::read_dir(args.criterion_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let entry_file_name = entry.file_name();
        let testcase_dir = entry_file_name.to_string_lossy();
        if testcase_dir == "report" {
            continue;
        }

        // our dirs are of the form "username-dayXX-{parse,part1,part2}"
        if testcase_dir.split("-").count() != 3 {
            continue;
        }

        let [username, day, phase] = testcase_dir
            .split("-")
            .collect::<Vec<_>>()
            .try_into()
            .expect("we check its len 3");

        // println!("username: {}, day: {}, phase: {}", username, day, phase);
        let day = day
            .strip_prefix("day")
            .ok_or_else(|| color_eyre::eyre::eyre!("day doesn't start with day"))?
            .parse::<u8>()?;

        let path = entry.path().join("new/estimates.json");
        let estimates: serde_json::Value = serde_json::from_reader(
            std::fs::File::open(&path)
                .with_context(|| format!("trying to open {}", path.display()))?,
        )?;
        let median = estimates["median"]["point_estimate"]
            .as_f64()
            .ok_or_else(|| {
                color_eyre::eyre::eyre!("no median.point_estimate in {}", path.display())
            })?;
        users.insert(username.to_string());

        benchmarks
            .days
            .entry(day)
            .or_default()
            .phases
            .entry(phase.to_string())
            .or_default()
            .median_for_user
            .insert(username.to_string(), median);
    }
    let users: Vec<String> = users.into_iter().collect();
    // for each day, add a total phase
    for day_benchmarks in benchmarks.days.values_mut() {
        let mut total_phase = AoCBenchmarkPhase::default();
        for user in &users {
            total_phase.median_for_user.insert(user.to_owned(), 0.0);
        }
        for phase_benchmarks in day_benchmarks.phases.values() {
            for user in &users {
                if !phase_benchmarks.median_for_user.contains_key(user) {
                    total_phase.median_for_user.remove(user);
                } else if let Some(m) = total_phase.median_for_user.get_mut(user) {
                    *m += phase_benchmarks.median_for_user[user];
                }
            }
        }
        day_benchmarks
            .phases
            .insert("Total".to_string(), total_phase);
    }

    let mut table_builder = Builder::default();
    // header
    table_builder.set_header(
        [
            vec!["Day", "Phase"],
            users.iter().map(|s| s.as_str()).collect(),
        ]
        .concat(),
    );

    for (day, day_benchmarks) in benchmarks.days {
        for (phase, phase_benchmarks) in &day_benchmarks.phases {
            let mut row = vec![day.to_string(), phase.to_owned()];
            let min_median = phase_benchmarks
                .median_for_user
                .values()
                .copied()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            for user in &users {
                let median = phase_benchmarks.median_for_user.get(user).copied();
                if let Some(median) = median {
                    let maybe_bold = if median < min_median * 1.05 { "**" } else { "" };
                    let (median, unit) = helper::scale_nanoseconds_value(median);
                    row.push(format!("{}{:.3}{}{}", maybe_bold, median, unit, maybe_bold));
                } else {
                    // check what happened here
                    if log.contains(&format!("{user}-day{day:02}-{phase}: not implemented")) {
                        row.push("-".to_string());
                    } else if log.contains(&format!("{user}-day{day:02}-{phase}: error")) {
                        row.push("😔".to_string());
                    } else if log.contains(&format!("{user}-day{day:02}-{phase}: timeout")) {
                        row.push("🐌".to_string());
                    } else if log.contains(&format!("{user}-day{day:02}-{phase}: panicked")) {
                        row.push("💥".to_string());
                    } else if log.contains(&format!("{user}-day{day:02}-{phase}: wrong result")) {
                        row.push("❌".to_string());
                    } else {
                        row.push("⁉️".to_string());
                    }
                }
            }
            table_builder.push_record(row);
        }
    }
    println!("# AoC2023 Benchmark Results");
    println!("");
    println!(
        "{}",
        table_builder
            .build()
            .with(Style::markdown())
            .with(Modify::new(Rows::new(1..)).with(Alignment::right()))
            .to_string(),
    );
    println!();
    println!("🐌 - Program timeout (parse: 1sec, part1: 10sec, part2: 30sec)");
    println!("💥 - Program panicked");
    println!("❌ - Program produced invalid result");
    println!("- - Not implemented");
    println!("⁉️ - Unknown error occured");

    Ok(())
}

mod helper {
    // made similar to DurationFormatter from criterion
    pub fn scale_nanoseconds_value(ns: f64) -> (f64, &'static str) {
        let (factor, unit) = if ns < 10f64.powi(0) {
            (10f64.powi(3), "ps")
        } else if ns < 10f64.powi(3) {
            (10f64.powi(0), "ns")
        } else if ns < 10f64.powi(6) {
            (10f64.powi(-3), "µs")
        } else if ns < 10f64.powi(9) {
            (10f64.powi(-6), "ms")
        } else {
            (10f64.powi(-9), "s")
        };

        (ns * factor, unit)
    }
}

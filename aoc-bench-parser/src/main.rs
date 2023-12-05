use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use clap::Parser;
use color_eyre::eyre::{Context, Result};
use prettytable::{Cell, Row};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    criterion_dir: PathBuf,
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
    // Day, Phase, User, User, User, ...
    let users: Vec<String> = users.into_iter().collect();

    let mut table = prettytable::Table::new();
    // header
    table.set_titles(Row::new(
        [
            vec![Cell::new("Day"), Cell::new("Phase")],
            users.iter().map(|s| Cell::new(s)).collect(),
        ]
        .concat(),
    ));

    for (day, day_benchmarks) in benchmarks.days {
        for (phase, phase_benchmarks) in &day_benchmarks.phases {
            let mut row = vec![Cell::new(&day.to_string()), Cell::new(&phase)];
            for user in &users {
                let median = phase_benchmarks.median_for_user.get(user).copied();
                if let Some(median) = median {
                    let (median, unit) = helper::scale_nanoseconds_value(median);
                    row.push(Cell::new(&format!("{:.3}{}", median, unit)));
                } else {
                    row.push(Cell::new("-"));
                }
            }
            table.add_row(Row::new(row));
        }
        // add the total for the day
        let mut row = vec![Cell::new(&day.to_string()), Cell::new("Total")];
        for user in &users {
            let median = day_benchmarks
                .phases
                .values()
                .map(|phase| phase.median_for_user.get(user).copied())
                .sum();
            if let Some(median) = median {
                let (median, unit) = helper::scale_nanoseconds_value(median);
                row.push(Cell::new(&format!("{:.3}{}", median, unit)));
            } else {
                row.push(Cell::new("-"));
            }
        }
        table.add_row(Row::new(row));
    }
    table.printstd();

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

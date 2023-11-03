use std::{env, fmt::Display, str::FromStr};

use anyhow::{Context, Result};
use tracing::{error, info, instrument, span, Level};
use tracing_subscriber::fmt::format::FmtSpan;
use yansi::Paint;

use crate::{load, Solution};

pub trait BaseName {
    fn base_name(&self) -> Self;
}

impl BaseName for &str {
    fn base_name(&self) -> Self {
        self.rfind('.').map_or(self, |n| &self[..n])
    }
}

pub fn log_init() {
    // install global collector configured based on RUST_LOG env var.
    let level =
        env::var("RUST_LOG").map_or(Level::INFO, |v| Level::from_str(&v).unwrap_or(Level::INFO));
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ACTIVE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(level)
        .init();
}

#[instrument]
pub fn run<S, R>(samples: &[&str], full: &[&str]) -> Result<()>
where
    S: Solution
        + TryFrom<std::io::BufReader<std::fs::File>, Error = std::io::Error>
        + std::fmt::Debug,
    S::Result: Context<R, anyhow::Error>,
    R: Display,
{
    let basename = std::env::current_exe()
        .ok()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .map(|s| s.base_name())
        .unwrap()
        .to_owned();

    span!(Level::INFO, "samples").in_scope(|| {
        for suffix in samples.iter() {
            let filename = format!("input/{basename}.{suffix}");
            if let Err(e) = run_solution_file::<S, R>(&filename, false) {
                error!(
                    "{}Failed running against '{}': {:?}",
                    Paint::mask("🎄 "),
                    filename,
                    e
                );
            }
        }
    });
    span!(Level::INFO, "full").in_scope(|| {
        for suffix in full.iter() {
            let filename = format!("input/{basename}.{suffix}");
            if let Err(e) = run_solution_file::<S, R>(&filename, true) {
                error!(
                    "{}Failed running against '{}': {:?}",
                    Paint::mask("🎅 "),
                    filename,
                    e
                );
            }
        }
    });
    Ok(())
}

fn run_solution_file<S, R>(filename: &str, is_full: bool) -> Result<()>
where
    S: Solution
        + TryFrom<std::io::BufReader<std::fs::File>, Error = std::io::Error>
        + std::fmt::Debug,
    S::Result: Context<R, anyhow::Error>,
    R: Display,
{
    let mut solution = load::<S>(filename)?;
    info!(
        "{}{} {}: {:?}",
        Paint::mask("🎄 "),
        Paint::bold(&Paint::green(filename)),
        Paint::bold(&Paint::yellow("solution")),
        solution
    );
    solution.analyse(is_full);
    info!(
        "{}part1 answer is {}",
        Paint::mask("🎅 "),
        Paint::bold(&Paint::red(
            &solution.answer_part1(is_full).context("part1 failed")?
        ))
    );
    info!(
        "{}part2 answer is {}",
        Paint::mask("🎅 "),
        Paint::bold(&Paint::red(
            &solution.answer_part2(is_full).context("part2 failed")?
        ))
    );

    Ok(())
}

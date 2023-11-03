use anyhow::Result;
use day1::{ResultType, Solution};

fn main() -> Result<()> {
    utils::log_init();

    utils::run::<Solution, ResultType>(&["sample"], &["full"])
}

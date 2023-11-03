use anyhow::Result;
use day2::{ResultType, Solution};

fn main() -> Result<()> {
    utils::log_init();

    utils::run::<Solution, ResultType>(&["sample"], &["full"])
}

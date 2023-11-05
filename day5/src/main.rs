use anyhow::Result;
use day5::{ResultType, Solution};

fn main() -> Result<()> {
    utils::log_init();

    utils::run::<Solution, ResultType>(&["sample"], &["full"])
}

use anyhow::{Context, Result as AnyResult};
use std::{env, path::PathBuf};

fn main() -> AnyResult<()> {
    eprintln!("=-=-=-=-= No documentation as-yet for 'setup'");
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("setup") => setup(),
        _ => xtaskops::tasks::main(),
    }
}

fn setup() -> AnyResult<()> {
    let entry = env::args().nth(2).context("require new workspace entry")?;

    // Copy template to new directory
    println!("* copy 'template' to '{entry}'");
    xtaskops::ops::copy_contents("template", &entry, false)?;
    // Replace 'template' with entry name in Cargo.toml, src/main.rs
    for filename in ["Cargo.toml", "src/main.rs"]
        .iter()
        .map(|name| PathBuf::from(&entry).join(name))
    {
        println!("* setup {filename:?}");
        let contents = std::fs::read_to_string(&filename)?;
        let contents = contents.replace("template", &entry);
        std::fs::write(filename, contents)?;
    }
    // Add entry to workspace
    println!("* setup Cargo.toml");

    let workspace_toml_contents = std::fs::read_to_string("Cargo.toml")?;
    let mut toml = workspace_toml_contents.parse::<toml_edit::Document>()?;
    toml["workspace"]["members"]
        .as_array_mut()
        .context("read workspace members")?
        .push(entry);
    std::fs::write("Cargo.toml", toml.to_string())?;
    Ok(())
}

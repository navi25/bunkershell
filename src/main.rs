// Declare modules
mod ai;
mod core;

// External crates
use anyhow::Result;

// Internal imports
use crate::core::bunkershell::BunkerShell;

fn main() -> Result<()> {
    let bunker_shell = BunkerShell::new();
    bunker_shell.run()
}

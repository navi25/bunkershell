// Declare modules
mod ai;
mod base;

// External crates
use anyhow::Result;

// Internal imports
use crate::base::bunkershell::BunkerShell;

fn main() -> Result<()> {
    let bunker_shell = BunkerShell::new();
    bunker_shell.run()
}

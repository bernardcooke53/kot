mod cli;
mod magik;

use anyhow::Result;

fn main() -> Result<()> {
    cli::main::main()
}

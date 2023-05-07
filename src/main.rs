use pastry as lib;

use anyhow::Result;

fn main() -> Result<()> {
    print!("{}", lib::pastry()?);
    Ok(())
}

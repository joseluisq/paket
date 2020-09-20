extern crate paket;

use paket::{Paket, Result};

fn main() -> Result {
    Paket::new()?.run()?;

    Ok(())
}

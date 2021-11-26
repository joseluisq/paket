#![deny(warnings)]
#![deny(rust_2018_idioms)]

use paket::{Paket, Result};

fn main() -> Result {
    Paket::new()?.run()?;

    Ok(())
}

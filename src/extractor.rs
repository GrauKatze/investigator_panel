//! module for extractor work

use crate::Manual;

struct Extractor;

impl Manual for Extractor {
    fn run(_opt: Option<Vec<String>>) -> Result<(), String> {
        println!("EXTRACTOR");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}

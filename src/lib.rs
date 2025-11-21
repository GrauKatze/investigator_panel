#![warn(missing_docs)]

//! # Panel

mod analyse;
mod extractor;
mod inspector;
mod register;

///Setup launch program
pub mod configuration;

mod manual {
    pub fn run(_opt: Option<Vec<String>>) -> Result<(), String> {
        println!("MANUAL");
        Ok(())
    }
}

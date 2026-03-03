//! # Panel

mod analyse;
mod extractor;
mod inspector;
mod register;

///Setup launch program
pub mod configuration;

trait Manual {
    fn run(_opt: Option<Vec<String>>)->Result<(),String>;
    fn configure()->Result<(),String>;
}
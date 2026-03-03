//! module for analyst work

use log::debug;
use crate::Manual;

pub struct Analyser;

impl Manual for Analyser {
    fn run(opt: Option<Vec<String>>) -> Result<(), String> {
        println!("ANALYSE");
        debug!("{:?}", opt);
        match match analyst::configuration::init(opt) {
            Ok(conf) => analyst::configuration::run(conf),
            Err(msg) => Err(msg),
        } {
            Ok(()) => Ok(()),
            Err(msg) => Err(msg),
        }
    }
}

#[cfg(test)]
mod tests {

}

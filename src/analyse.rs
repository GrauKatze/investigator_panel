//! module for analyst work

use log::debug;

pub fn run(opt: Vec<String>) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_with_empty_opt() {
        let src = Vec::new();
        assert_eq!(Err("need more argument".to_string()), run(src));
    }

    #[test]
    fn run_test_with_self() {
        let src = vec![String::from("src/analyse.rs")];
        assert_eq!(Ok(()), run(src));
    }
}

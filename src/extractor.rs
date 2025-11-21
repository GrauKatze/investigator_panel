//! module for extractor work

use std::path::Path;

pub fn _file_extract(src: String, dst: String) -> Result<(), String> {
    if Path::exists(Path::new(&src)) && Path::exists(Path::new(&dst)) {
        Ok(())
    } else {
        Err("err".to_string())
    }
}
pub fn _extract_from_file(src: String, dst: String) -> Result<(), String> {
    match extractor::check_file(&src) {
        Ok(()) => {
            if Path::exists(Path::new(&src)) && Path::exists(Path::new(&dst)) {
                Ok(())
            } else {
                Err("err".to_string())
            }
        }
        Err(msg) => Err(msg),
    }
}

pub fn run(_src: Option<String>, _dst: Option<String>) -> Result<(), String> {
    println!("EXTRACTOR");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extractor::run;
    #[test]
    fn run_test_with_empty_src() {
        let src = Some(String::new());
        let dst = Some(String::new());

        assert_eq!(Ok(()), run(src, dst));
    }

    #[test]
    fn run_test_with_self() {
        let src = Some(String::from("src/extractor.rs"));
        let dst = Some(String::new());

        assert_eq!(Ok(()), run(src, dst));
    }
}

//! module for extractor work

pub fn _file_extract(src: String, dst: String) -> Result<(), String> {
    if _is_path_valid(&src) && _is_path_valid(&dst) {
        Ok(())
    } else {
        Err("err".to_string())
    }
}
pub fn _extract_from_file(src: String, dst: String) -> Result<(), String> {
    match extractor::check_file(&src) {
        Ok(()) => {
            if _is_path_valid(&src) && _is_path_valid(&dst) {
                Ok(())
            } else {
                Err("err".to_string())
            }
        }
        Err(msg) => Err(msg),
    }
}
fn _is_path_valid(_path: &String) -> bool {
    true
}

pub fn run(src: Option<String>, dst: Option<String>) -> Result<(), String> {
    println!("EXTRATOR");
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

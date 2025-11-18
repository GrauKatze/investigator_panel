//! module for register work
fn _reg_file(file_path: String) {
    register::take_file(file_path).unwrap()
}

fn _take_file(file_path: String) {
    register::give_file(file_path).unwrap()
}

pub fn run() -> Result<(), String> {
    println!("REGISTR");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::analyse::run;
    #[test]
    fn run_test_with_empty_src() {
        let src = Some(String::new());
        assert_eq!(Ok(()), run(src));
    }

    #[test]
    fn run_test_with_self() {
        let src = Some(String::from("src/registr.rs"));
        assert_eq!(Ok(()), run(src));
    }
}

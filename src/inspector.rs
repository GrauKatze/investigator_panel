pub fn run(src: Option<String>) -> Result<(), String> {
    println!("INSPECTOR");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_with_empty_src() {
        let src = Some(String::new());
        assert_eq!(Ok(()), run(src));
    }

    #[test]
    fn run_test_with_self() {
        let src = Some(String::from("src/inspector.rs"));
        assert_eq!(Ok(()), run(src));
    }
}

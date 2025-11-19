//! module for analyst work

fn analyse(src: String) -> Result<(), String> {
    let an = analyst::file_identification::_file_ident(src);
    match an {
        Ok(mt) => {
            println!("analyze: {:#?}", mt);
            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

pub fn run(src: Option<String>) -> Result<(), String> {
    println!("ANALYSE");
    match src {
        Some(path) => match analyse(path) {
            Ok(()) => Ok(()),
            Err(msg) => Err(msg),
        },

        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_with_empty_src() {
        let src = Some(String::new());
        assert_eq!(Err(String::from("path is not valid, path: ''")), run(src));
    }

    #[test]
    fn run_test_with_self() {
        let src = Some(String::from("src/analyse.rs"));
        assert_eq!(Ok(()), run(src));
    }
}

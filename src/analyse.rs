//! module for analyst work

fn analyse(src: String) {
    println!(
        "analyze: {:#?}",
        analyst::file_identification::_file_ident(src)
    );
}

pub fn run(src: Option<String>) -> Result<(), String> {
    println!("ANALYSE");
    match src {
        Some(path) => {
            analyse(path);
            Ok(())
        }

        None => Ok(()),
    }
}

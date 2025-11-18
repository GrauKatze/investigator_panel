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

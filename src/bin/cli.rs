use std::env::args;

use investigator::Panel;

fn main() {
    println!(env!("CARGO_PKG_NAME"));
    println!("===START===");

    let start_option: Vec<String> = args().skip(1).collect();

    match match Panel::init(Some(start_option)) {
        Ok(panel) => panel.run(),
        Err(msg) => Err(msg),
    } {
        Ok(()) => (),
        Err(msg) => println!("{msg}"),
    }

    println!("===END===");
}

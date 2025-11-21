fn main() {
    println!(env!("CARGO_PKG_NAME"));
    println!("===START===");
    match match investigator::configuration::init(std::env::args().skip(1).collect()) {
        Ok(cnf) => investigator::configuration::run(cnf),
        Err(msg) => Err(msg),
    } {
        Ok(()) => (),
        Err(msg) => println!("{}", msg),
    }

    println!("===END===");
}

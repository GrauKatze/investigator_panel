fn main() {
    println!(env!("CARGO_PKG_NAME"));
    println!("===START===");
    investigator::configuration::init(std::env::args().skip(1).collect());
    println!("===END===");
}

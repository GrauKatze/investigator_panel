fn main() {
    println!(env!("CARGO_PKG_NAME"));
    println!("===START===");
    let conf = investigator::configuration::init(std::env::args().skip(1).collect());
    match conf {
        Ok(cnf) => investigator::configuration::run(cnf),
        Err(msg) => println!("{msg}\nwrite 'help'"),
    };
    println!("===END===");
}

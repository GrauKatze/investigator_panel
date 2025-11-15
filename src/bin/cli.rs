fn main() {
    configuration::init(std::env::args().skip(1).collect());
}

///Setup launch program
mod configuration {
    pub enum ConfigType {
        /// Вывод справки
        Help,
        /// Вывод версии приложения
        Version,
    }

    fn write_help() {
        println!(env!("CARGO_PKG_NAME"));
        println!("version: {}", env!("CARGO_PKG_VERSION"));
        println!(env!("CARGO_PKG_DESCRIPTION"));
        println!("\nUsage: {} [key]\n", env!("CARGO_PKG_NAME"));
        println!("KEYS:");
        println!("{:25} {}", "-h | --help", "this text");
        println!("{:25} {}", "-v | --version", "version");
    }
    pub fn args_pars(args: Vec<String>) -> Result<ConfigType, String> {
        if args.len() > 0 {
            match args[0].as_str() {
                "-h" | "--help" => Ok(ConfigType::Help),
                "-v" | "--version" => Ok(ConfigType::Version),
                _ => Err(format!("not found this argument\n{:?}", args)),
            }
        } else {
            Err("need more argument\n".to_string())
        }
    }
    pub fn init(args: Vec<String>) {
        match args_pars(args) {
            Ok(config_type) => match config_type {
                ConfigType::Help => {
                    write_help();
                }
                ConfigType::Version => {
                    println!("version: {}", env!("CARGO_PKG_VERSION"));
                }
            },
            Err(msg) => {
                println!("{}", msg);
                std::process::exit(1);
            }
        }
    }
}

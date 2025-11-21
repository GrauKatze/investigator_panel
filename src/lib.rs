#![warn(missing_docs)]

//! # Panel

mod analyse;
mod extractor;
mod inspector;
mod register;

///Setup launch program
pub mod configuration {
    use crate::analyse;
    use crate::extractor;
    use crate::inspector;
    use crate::manual;
    use crate::register;
    use colored::Colorize;

    /// Setup configuration
    #[derive(Debug)]
    pub enum ConfigType {
        /// Вывод справки
        Help,
        /// Вывод версии приложения
        Version,
        /// Analyse
        Analyse(Option<Vec<String>>),
        /// Extract Files
        Extract(Option<String>, Option<String>),
        /// Inpector
        Inpector(Option<String>),
        /// Registr
        Registr(Option<String>),
        /// Manual
        Manual(Option<Vec<String>>),
    }

    fn write_help() {
        println!(env!("CARGO_PKG_NAME"));
        println!("version: {}", env!("CARGO_PKG_VERSION"));
        println!(env!("CARGO_PKG_DESCRIPTION"));
        println!("\nUsage: {} [module] <key>\n", env!("CARGO_PKG_NAME"));
        println!("KEYS:");
        println!("{:3} | {:25} {}", "-h".bold(), "--help", "this text");
        println!("{:3} | {:25} {}", "-v".bold(), "--version", "version");
        println!(
            "{:3} | {:25} {}",
            "-a".bold(),
            "--analyse <OPTION>",
            "analyse"
        );
        println!(
            "{:3} | {:25} {}",
            "-e".bold(),
            "--extractor <SRC> <DST>",
            "extractor"
        );
        println!(
            "{:3} | {:25} {}",
            "-i".bold(),
            "--inspector <SRC>",
            "inspector"
        );
        println!("{:3} | {:25} {}", "-r".bold(), "--registr", "registr");
        println!("{:3} | {:25} {}", "-m".bold(), "--manual <OPT>", "manual");
    }

    fn args_pars(args: Vec<String>) -> Result<ConfigType, String> {
        match args.get(0) {
            Some(p) => {
                match p.as_str() {
                    "-h" | "--help" | "h" | "help" => Ok(ConfigType::Help),
                    "-v" | "--version" | "v" | "version" => Ok(ConfigType::Version),
                    "-a" | "--analyse" | "a" | "analyze" => {
                        let src: Option<Vec<String>> = match args.get(1..) {
                            Some(opt) => Some(opt.to_vec()),
                            None => None,
                        };
                        Ok(ConfigType::Analyse(src))
                    }

                    "-e" | "--extractor" | "e" | "extractor" => {
                        let src: Option<&String> = args.get(1);
                        let dst: Option<&String> = args.get(2);
                        Ok(ConfigType::Extract(src.cloned(), dst.cloned()))
                    }
                    "-i" | "--inspector" | "i" | "inspector" => {
                        Ok(ConfigType::Inpector(args.get(1).cloned()))
                    }
                    "-r" | "--registr" | "r" | "registr" => {
                        Ok(ConfigType::Registr(args.get(1).cloned()))
                    }
                    //FIXME: maybe i can use only vec<string>?
                    "-m" | "--manual" | "m" | "manual" => match args.get(1..) {
                        Some(conf) => Ok(ConfigType::Manual(Some(conf.to_vec()))),
                        None => Ok(ConfigType::Manual(None)),
                    },

                    _ => Err(format!("not found this arguments\n{{ {} }}", args.concat())),
                }
            }
            None => Err("need more argument".to_string()),
        }
    }

    /// Initial program configuration
    pub fn init(args: Vec<String>) -> Result<ConfigType, String> {
        // dbg!(&args);
        match args_pars(args) {
            Ok(config_type) => Ok(config_type),
            Err(msg) => Err(msg),
        }
    }
    //TODO: add doc
    #[allow(missing_docs)]
    pub fn run(config_type: ConfigType) -> Result<(),String>{
        match config_type {
            ConfigType::Help => {
                write_help();
                Ok(())
            }
            ConfigType::Version => {
                println!("version: {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            //TODO: Error work
            ConfigType::Analyse(src) => analyse::run(src.expect("msg")),
            ConfigType::Extract(src, dst) => extractor::run(src, dst),
            ConfigType::Inpector(src) => inspector::run(src),
            ConfigType::Manual(opt) => manual::run(opt),
            ConfigType::Registr(src) => register::run(src),
        }
    }
}

mod manual {
    pub fn run(_opt: Option<Vec<String>>) -> Result<(), String> {
        println!("MANUAL");
        Ok(())
    }
}

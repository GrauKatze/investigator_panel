use crate::Manual;
use crate::analyse;
use crate::extractor;
use crate::inspector;
use crate::register;
use colored::Colorize;
use log::debug;

/// Setup configuration
pub enum ConfigType {
    /// Вывод справки
    Help,
    /// Вывод версии приложения
    Version,
    /// Analyse
    Analyse(Option<Vec<String>>),
    /// Extract Files
    Extract(Option<Vec<String>>),
    /// Inpector
    Inpector(Option<Vec<String>>),
    /// Registr
    Registr(Option<Vec<String>>),
}

// TODO:Обновить справку
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
}

fn args_pars(args: Vec<String>) -> Result<ConfigType, String> {
    match args.get(0) {
        Some(p) => match p.as_str() {
            "-h" | "--help" | "h" | "help" => Ok(ConfigType::Help),
            "-v" | "--version" | "v" | "version" => Ok(ConfigType::Version),
            "-a" | "--analyse" | "a" | "analyze" => match args.get(1..) {
                Some(opt) => Ok(ConfigType::Analyse(Some(opt.to_vec()))),
                None => Ok(ConfigType::Analyse(None)),
            },

            "-e" | "--extractor" | "e" | "extractor" => match args.get(1..) {
                Some(conf) => Ok(ConfigType::Extract(Some(conf.to_vec()))),
                None => Ok(ConfigType::Extract(None)),
            },
            "-i" | "--inspector" | "i" | "inspector" => match args.get(1..) {
                Some(conf) => Ok(ConfigType::Inpector(Some(conf.to_vec()))),
                None => Ok(ConfigType::Inpector(None)),
            },
            "-r" | "--registr" | "r" | "registr" => match args.get(1..) {
                Some(conf) => Ok(ConfigType::Registr(Some(conf.to_vec()))),
                None => Ok(ConfigType::Registr(None)),
            },

            _ => Err(format!("not found this arguments\n{{ {} }}", args.concat())),
        },
        None => Err("need more argument".to_string()),
    }
}

/// Initial program configuration
pub fn init(args: Vec<String>) -> Result<ConfigType, String> {
    debug!("{:?}", args);
    match args_pars(args) {
        Ok(config_type) => Ok(config_type),
        Err(msg) => Err(msg),
    }
}

//TODO: add doc
#[allow(missing_docs)]
pub fn run(config_type: ConfigType) -> Result<(), String> {
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
        ConfigType::Analyse(opt) => analyse::Analyser::run(opt),
        ConfigType::Extract(opt) => extractor::Extractor::run(opt),
        ConfigType::Inpector(src) => inspector::Inspector::run(src),
        ConfigType::Registr(src) => register::run(src),
    }
}

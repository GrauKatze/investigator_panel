//! # Panel

use crate::{
    mod_analyse::IAnalyser, mod_extractor::IExtractor, mod_inspector::IInspector,
    mod_register::IRegister,
};

pub mod mod_analyse;
pub mod mod_extractor;
pub mod mod_inspector;
pub mod mod_register;

enum Config {
    Help,
    Version,
    RunComponent(Component),
}

enum Component {
    Anayser(ComponentConfig),
    Extractor(ComponentConfig),
    Inspector(ComponentConfig),
    Register(ComponentConfig),
}

pub struct Panel {
    configuration: Config,
}

impl Panel {
    fn args_pars(init_options: Option<Vec<String>>) -> Result<Config, String> {
        match init_options {
            None => Err("need mode argemunts".to_string()),
            Some(options) => match options.get(0) {
                Some(p) => match p.as_str() {
                    "-h" | "--help" | "h" | "help" => Ok(Config::Help),
                    "-v" | "--version" | "v" | "version" => Ok(Config::Version),
                    "-a" | "--analyser" | "a" | "analyser" => Ok(Config::RunComponent(
                        Component::Anayser(ComponentConfig::build(options)),
                    )),
                    "-e" | "--extractor" | "e" | "extractor" => Ok(Config::RunComponent(
                        Component::Extractor(ComponentConfig::build(options)),
                    )),
                    "-i" | "--inspector" | "i" | "inspector" => Ok(Config::RunComponent(
                        Component::Inspector(ComponentConfig::build(options)),
                    )),
                    "-r" | "--register" | "r" | "register" => Ok(Config::RunComponent(
                        Component::Register(ComponentConfig::build(options)),
                    )),
                    _ => Err(format!(
                        "not found this arguments\n{{ {:?} }}",
                        options.iter().collect::<Vec<&String>>()
                    )),
                },
                None => Err("need mode argemunts".to_string()),
            },
        }
    }

    pub fn init(init_options: Option<Vec<String>>) -> Result<Panel, String> {
        match Self::args_pars(init_options) {
            Ok(config) => Ok(Panel {
                configuration: config,
            }),
            Err(msg) => Err(msg),
        }
    }
    pub fn run(&self) -> Result<(), String> {
        match &self.configuration {
            Config::RunComponent(modul) => match modul {
                Component::Extractor(comp_conf) => IExtractor::build(comp_conf).run(),
                Component::Anayser(comp_conf) => IAnalyser::build(comp_conf).run(),
                Component::Register(comp_conf) => IRegister::build(comp_conf).run(),
                Component::Inspector(comp_conf) => IInspector::build(comp_conf).run(),
            },
            Config::Help => {
                Self::write_help_msg();
                Ok(())
            }
            Config::Version => {
                Self::write_version_msg();
                Ok(())
            }
        }
    }
    fn write_help_msg() {
        println!(env!("CARGO_PKG_NAME"));
        println!(env!("CARGO_PKG_VERSION"));
    }
    fn write_version_msg() {
        println!(env!("CARGO_PKG_VERSION"));
    }
}

#[derive(Clone)]
enum ComponentConfig {
    Help,
    Version,
    Command(Option<Vec<String>>),
}

impl ComponentConfig {
    fn build(options: Vec<String>) -> ComponentConfig {
        match options.get(1) {
            Some(option) => match option.as_str() {
                "-h" | "--help" | "h" | "help" => ComponentConfig::Help,
                "-v" | "--version" | "v" | "version" => ComponentConfig::Version,
                _ => {
                    let command_collect: Vec<String> = options[1..].to_vec();
                    ComponentConfig::Command(Some(command_collect))
                }
            },
            None => ComponentConfig::Command(None),
        }
    }
}

trait ComponentWorker {
    //TODO: add threading
    fn run(&self) -> Result<(), String>;
}

trait ComponentBuilder {
    fn build(comp_conf: &ComponentConfig) -> Self;
}

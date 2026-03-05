//! module for analyst work
use crate::{ComponentBuilder, ComponentConfig, ComponentWorker};
use register::Register;

pub struct IRegister {
    configure: ComponentConfig,
    // pub component: Analyser,
}

impl ComponentWorker for IRegister {
    fn run(&self) -> Result<(), String> {
        match &self.configure {
            ComponentConfig::Command(_opt) => self::Register::build(_opt).run(),
            ComponentConfig::Help => self::Register::write_help_msg(),
            ComponentConfig::Version => Err("not write".to_string()),
        }
    }
}

impl ComponentBuilder for IRegister {
    fn build(comp_conf: &ComponentConfig) -> Self {
        IRegister {
            configure: comp_conf.clone(),
        }
    }
}

#[cfg(test)]
mod tests {}

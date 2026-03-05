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
            ComponentConfig::Command(opt) => match self::Register::build(opt) {
                Ok(register_component) => register_component.run(),
                Err(msg) => Err(format!("Register ERROR:\n{}", msg)),
            },
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

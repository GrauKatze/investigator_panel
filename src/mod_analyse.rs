//! module for analyst work
use std::path::Component;

use crate::{ComponentBuilder, ComponentConfig, ComponentWorker};
use analyst::Analyser;

pub struct IAnalyser {
    configure: ComponentConfig,
    // pub component: Analyser,
}

impl ComponentWorker for IAnalyser {
    fn run(&self) -> Result<(), String> {
        match &self.configure {
            ComponentConfig::Command(opt) => match self::Analyser::build(opt) {
                Ok(analyser_component) => analyser_component.run(),
                Err(msg) => Err(format!("Analyser ERROR:\n{}", msg)),
            },
            ComponentConfig::Help => self::Analyser::write_help_msg(),
            ComponentConfig::Version => self::Analyser::write_version_msg(),
        }
    }
}

impl ComponentBuilder for IAnalyser {
    fn build(comp_conf: &ComponentConfig) -> Self {
        IAnalyser {
            configure: comp_conf.clone(),
        }
    }
}

#[cfg(test)]
mod tests {}

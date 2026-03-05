//! module for extractor work

use extractor::Extractor;

use crate::{ComponentBuilder, ComponentConfig, ComponentWorker};
pub struct IExtractor {
    configure: ComponentConfig,
}

impl ComponentWorker for IExtractor {
    fn run(&self) -> Result<(), String> {
        match &self.configure {
            ComponentConfig::Command(opt) => match self::Extractor::build(opt) {
                Ok(extractor_component) => extractor_component.run(),
                Err(msg) => Err(format!("Extractor ERROR:\n{}", msg)),
            },
            ComponentConfig::Help => self::Extractor::write_help_msg(),
            ComponentConfig::Version => self::Extractor::write_version_msg(),
        }
    }
}

impl ComponentBuilder for IExtractor {
    fn build(comp_conf: &ComponentConfig) -> Self {
        IExtractor {
            configure: comp_conf.clone(),
        }
    }
}

#[cfg(test)]
mod tests {}

//! module for analyst work
use crate::{ComponentBuilder, ComponentConfig, ComponentWorker};
use inspector::Inspector;

pub struct IInspector {
    configure: ComponentConfig,
    // pub component: Analyser,
}

impl ComponentWorker for IInspector {
    fn run(&self) -> Result<(), String> {
        match &self.configure {
            ComponentConfig::Command(opt) => match self::Inspector::build(opt) {
                Ok(inspector_component) => inspector_component.run(),
                Err(msg) => Err(format!("Inspector ERROR:\n{}", msg)),
            },
            ComponentConfig::Help => self::Inspector::write_help_msg(),
            ComponentConfig::Version => Err("not write".to_string()),
        }
    }
}

impl ComponentBuilder for IInspector {
    fn build(comp_conf: &ComponentConfig) -> Self {
        IInspector {
            configure: comp_conf.clone(),
        }
    }
}

#[cfg(test)]
mod tests {}

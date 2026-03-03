use crate::Manual;

pub struct Inspector;

impl Manual for Inspector {
    fn run(_src: Option<Vec<String>>) -> Result<(), String> {
        println!("INSPECTOR");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}

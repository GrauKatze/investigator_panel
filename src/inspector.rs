use crate::Manual;

struct Inspector;

impl Manual for Inspector {
    fn run(_opt: Option<Vec<String>>)->Result<(),String> {
        Ok(())
    }
}



pub fn run(_src: Option<Vec<String>>) -> Result<(), String> {
    println!("INSPECTOR");
    Ok(())
}

#[cfg(test)]
mod tests {

}

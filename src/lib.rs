#![warn(missing_docs)]

//! # Panel

///module for analyst work
mod analyst_panel {
    fn _file_ident(file_path: String) {
        analyst::file_identification::_file_ident(file_path).unwrap();
    }
}

///module for extractor work
mod extractor_panel {
    pub fn _file_extract(src: String, dst: String) -> Result<(), String> {
        if _is_path_valid(&src) && _is_path_valid(&dst) {
            Ok(())
        } else {
            Err("err".to_string())
        }
    }
    pub fn _extract_from_file(src: String, dst: String) -> Result<(), String> {
        match extractor::check_file(&src) {
            Ok(()) => {
                if _is_path_valid(&src) && _is_path_valid(&dst) {
                    Ok(())
                } else {
                    Err("err".to_string())
                }
            }
            Err(msg) => Err(msg),
        }
    }
    fn _is_path_valid(_path: &String) -> bool {
        true
    }
}

///module for register work
mod register_panel {
    fn _reg_file(file_path: String){
        register::take_file(file_path).unwrap()
    }

    fn _take_file(file_path: String){
        register::give_file(file_path).unwrap()
    }
}

use std::{env,path::PathBuf};

pub fn current() -> Option<PathBuf> {
    match env::current_exe() {
        Ok(mut ex_dir) => {
            ex_dir.pop();
            Some(ex_dir)
        },
        Err(_) => None
    }
}
pub fn current_as_string() -> Option<String> {
    match env::current_exe() {
        Ok(mut ex_dir) => {
            ex_dir.pop();
            match ex_dir.to_str() {
                Some(stg) => Some(String::from(stg)),
                None => None
            } },
        Err(_) => None
    }
}
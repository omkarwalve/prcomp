#[cfg(test)]
mod tests {
    use std::{env,io,path::PathBuf};
    use crate::config::{Tokenizer,ParseError};
    #[test]
    fn program_read() -> Result<(),ParseError> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        // println!("{}",manifest_dir);
        let mut program = Tokenizer::init(PathBuf::from(format!("{}/wsch/{}",manifest_dir,"sample.wsch")))?; 
        while !program.is_lexed() {
            program.eat()?;
        }
        Ok(())
    }
}
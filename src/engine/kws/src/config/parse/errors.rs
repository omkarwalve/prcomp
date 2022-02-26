
use std::fmt::{Debug,Formatter,self};
pub const INVALID_SYNTAX: &'static str = "INVALID SYNTAX";

pub enum ParseError {
    InvalidSyntax(String),
    IndentationError(String),
    IOError(String),
}
impl Debug for ParseError {
    fn fmt(&self,f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg)    =>  write!(f,"{}::{}",INVALID_SYNTAX,msg),
            ParseError::IndentationError(msg) =>  write!(f,"{}",msg),
            ParseError::IOError(msg)          =>  write!(f,"{}",msg),
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IOError(format!("{}",err))
    }
}
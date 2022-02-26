
// #### Backus-Naur Form
// ***
// Program
// : Statements
// ;
//
// Statement
// : Flags | Process
// ;
// 
// Flags
// : #[ EqualsMonad, EqualsMonad,... ];
// ;
// 
// Process
// : Singleton | Batch
// ;
// 
// Singleton
// : Identifier ProcessOperator Monad
// ;
// 
// Batch
// : Identifier ProcessOperator [ Monad , Monad , Monad => [ Monad,..., Monad => [Monad...]  ],...];
// ;
// 
// Monad
// : NodalMonad | EqualsMonad
// ;
// 
// EqualsMonad
// : Variable = Value
// ;
// 
// NodalMonad
// : Variable = (Value,Value,...) | Variable = (Value,Value,Value,....) ProcessChainOperator Identifier

use crate::{Tree,types::Node,config::{parse::INVALID_SYNTAX,ParseError}};
use std::{
    io::{self,BufRead,BufReader,Read},
    fmt::{self,Display},
    fs::File,path::PathBuf,str,
};

/// Levels Enum
pub enum ProcessKind<T> {
    SINGELTON(Monad<T>),
    BATCH(Tree<Monad<T>>),
}

/// ## StatementKind
/// Can be an `Process` or a `Flag`.
pub enum StatementKind<T: Copy + Clone> {
    /// ## Process `:: tree`
    /// A tree type assignment structure holding nodes of data to be accessed in a top -> bottom fashion.
    /// Can Hold `Singleton`,`Parent with Children` and `Direct Children` type of data.
    /// ***
    /// ### Examples
    /// - A `SINGELTON` process type:
    /// ```
    /// Foo :: bar = (baz,qux);
    /// ```
    /// - A `BATCH` process type:
    /// ```
    /// P1 :: [ base = (Class,prod-container) 
    ///         => [ name = (Attr,data-field,pname)
    ///            , ... 
    ///            , uri = (Name,a) => [ warranty = (Class,p_warnt)
    ///                                , returns = (Class,p_retrn)
    ///                                , reviews = (Class,cust_rvw)
    ///                                ],
    ///            ]
    ///          , flat_uri = "https://www.amazon.in/?q="
    ///         ];
    /// ```
    PROCESS(ProcessKind<T>),

    /// ## Flags `:: preprocessor`
    /// A preprocessor-like assignment structure holding flags related to Statement rather than being read into variables.
    /// ***
    /// ### Examples
    /// Name of a webschema(.wsch) is defined using the `name` flag :
    /// ```
    /// #[ name = "Amazon" ]
    /// ```
    FLAGS(Vec<Monad<T>>),
}

/// ## Monad
/// *****
/// The last unit statement that exists within a `Statement`.
pub enum Monad<T> {
    /// ## Nodal
    /// A Monad type that holds `node` assignments.
    /// ***
    /// ### Examples
    /// A simple node definition:
    /// ```
    /// foo = (bar,baz,qux)
    /// ```
    /// Node with pointers:
    /// ```
    /// foo = (bar,baz,qux) :: P2
    /// ```
    Nodal { name: T, value: Vec<Vec<T>>, pointer: Option<T> },

    /// ## Equal
    /// A simple assignment type Found in a `Statement::FLAG`.
    Equal{ lhs: T, rhs: T },
}

pub enum TokenKind {
    IDENTIFIER,
    OPERATOR,
}

impl TokenKind {
    fn from(vec: &[u8]) -> Result<Self,ParseError> {
        let [mut is_alnum,mut is_symbol] = [false; 2];
        vec.iter().enumerate().for_each(|(idx,byte)| {
            match byte {
                c if u8::is_ascii_alphanumeric(c) => is_alnum = true,
                c if u8::is_ascii_punctuation(c) => is_symbol = true,
                _ => (),
            }
        });

        match (is_alnum,is_symbol) {
            (true,true) | (false,false) => Err(ParseError::InvalidSyntax(String::from(""))),
                           (true,false) => Ok(Self::IDENTIFIER),
                           (false,true) => Ok(Self::OPERATOR),
        }
    }
}
impl Display for TokenKind {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IDENTIFIER  => write!(f,"IDENTIFIER"),
            Self::OPERATOR    => write!(f,"OPERATOR"),
        }
    }
}

pub struct Token<T> {
    kind: TokenKind,
    value: T,
    at: (u8,u8),
}
impl Token<String> {
    pub fn from(vec: &[u8],pos: (u8,u8)) -> Result<Self,ParseError> {
        let vec_as_string = String::from_utf8_lossy(&vec).to_string();
        match TokenKind::from(vec) {
            Ok(kind) => {
                Ok(Self {
                    kind,
                    value: vec_as_string,
                    at: pos,
                })
            },
            Err(_) => Err(ParseError::InvalidSyntax(format!("({},{}):- `{}`",pos.0,pos.1,vec_as_string))),
        }
    }
}
impl<D: Display> Display for Token<D> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Token {{\n\tkind: {}\n\tvalue: {}\n\tat: ({},{})\n}}",
            self.kind,
            self.value,
            self.at.0,
            self.at.1
        )
    }
}

pub enum Operator {
    Arrow,
    EqualTo,
    LRound,
    RRound,
    LSquare,
    RSquare,
    Hashtag,
    ChainOperator,
    ProcessOperator,
    Semicolon,
}

pub struct Tokenizer {
    /// (`row`, `column`)
    reader_cursor: (u8,u8),
    reader: BufReader<File>,
    buffer: Vec<u8>,
    body: Tree<Node<Token<String>>>,
    /// - `0` : **Not Started**
    /// - `1` : **Lexing Complete**
    status: usize, 
}

impl Tokenizer {
    pub fn init(file: PathBuf) -> io::Result<Self> {
        let fh = File::open(file)?;
        Ok(Self {
            reader_cursor: (0,0),
            buffer: vec![],
            reader: BufReader::new(fh),
            body: Tree::new(),
            status: 0,
        })
    }
    pub fn eat(&mut self) -> Result<(),ParseError> {
        if self.status == 0 {
            let mut to_buffer = true;
            let mut local_buffer: [u8; 1] = [0; 1];
            let mut read_char = || {
                match self.reader.read_exact(&mut local_buffer) {
                    Ok(_) => self.reader_cursor.1 += 1,
                    Err(_) => { println!("Done"); self.status = 1 },
                };
            };
            read_char();
            match local_buffer[0] {
                c if c == '(' as u8 => {
                    // println!("Found token '('");
                    let token = Token::from(&[c],self.reader_cursor)?;
                    println!("{}",token);
                    to_buffer = false;
                },
                c if u8::is_ascii_whitespace(&c) || c == ',' as u8 || c == ')' as u8 => {
                    to_buffer = false;
                    // Increment cursor if newline.
                    if self.buffer.len() != 0 {
                        // Cursor correction
                        // println!("!!:-({},{}) BufLen:- {} | data: \"{}\"",self.reader_cursor.0,self.reader_cursor.1,self.buffer.len(), String::from_utf8_lossy(&self.buffer));
                        let token_cursor = (self.reader_cursor.0, self.reader_cursor.1 - self.buffer.len() as u8);
                        // `self.buffer` contents needs to be fed into Token Parser
                        // FIXME: Bug on passing a Monad::Nodal::NodalValue since Multitoken input
                        let token = Token::from(&self.buffer,token_cursor)?;
                        println!("{}",token);
                        // println!("Buffer Contents({},{}):- {}",token_cursor.0,token_cursor.1, String::from_utf8_lossy(&self.buffer));
                        self.clear_buffer();
                    }
                    if c == '\n' as u8 { self.reader_cursor.0 += 1; self.reader_cursor.1 = 0; }
                    if c == ')' as u8 { println!("{}", Token::from(&[c],self.reader_cursor)?); }
                },
                _ => (),
            }
            // if u8::is_ascii_whitespace(&local_buffer[0]) { 
            //     // Increment cursor if newline.
            //     if local_buffer[0] == '\n' as u8 { self.reader_cursor.0 += 1; self.reader_cursor.1 = 0; }
            //     if self.buffer.len() != 0 {
            //         // Cursor correction
            //         let token_cursor = (self.reader_cursor.0, self.reader_cursor.1 - self.buffer.len() as u8);
            //         // `self.buffer` contents needs to be fed into Token Parser
            //         // FIXME: Bug on passing a Monad::Nodal::NodalValue since Multitoken input
            //         let token = Token::from(&self.buffer,token_cursor)?;
            //         println!("Buffer Contents({},{}):- {}",token_cursor.0,token_cursor.1, String::from_utf8_lossy(&self.buffer));
            //         self.clear_buffer();
            //     }
            // }
            if to_buffer { self.buffer.push(local_buffer[0]); }
        }
        Ok(())
    }
    pub fn is_lexed(&self) -> bool {
        if self.status > 0 { true } else { false }
    }
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}
//! This is the abstract syntax tree for the language
//! All usable constructs and syntax sugar should be encoded here
//! 
use crate::ident_env::*;

#[derive(Debug)]
pub enum Program {
    // if its just a list of definitions its the main module
    Definitions(Vec<Binding>),
    /*Module {
        // None indicates there was no export list (it could be empty)
        exports: Option<Vec<Varid>>,
        //types: Vec<()>,
        definitions: Vec<Binding>,
    }*/
}


#[derive(Debug)]
pub struct Binding {
    ident: Id,
    value: Box<Expr>
}

impl Binding {
    pub fn new(ident: Id, value: Expr) -> Self {
        Self {
            ident,
            value: Box::new(value)
        }
    }
}


#[derive(Debug)]
pub enum Expr {
    Tuple(Vec<Expr>),
    Ident(Id),
    Literal(Literal),
    Lambda {
        args: Vec<Id>,
        body: Box<Expr>
    },
    Application {
        func: Box<Expr>,
        args: Vec<Expr>
    },
    Operator(Box<Expr>, Id, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<Binding>, Box<Expr>),
    //LetRec(Vec<Binding>, Box<Expr>),
}


#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    Char(char),
    Byte(u8),
    Float(f64)
    // should add floats
}
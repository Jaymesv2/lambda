//! This is the abstract syntax tree for the language
//! All usable constructs and syntax sugar should be encoded here
//! 


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


// variable id

#[derive(Debug)]
pub struct Varid {
    name: String
}

impl Varid {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
    pub fn from_str(name: &str) -> Self {
        Self::new(name.to_string())
    }
}

#[derive(Debug)]
pub struct Application {
    func: Box<Expr>,
    args: Vec<Expr>
}

impl Application {
    pub fn new(func: Expr, args: Vec<Expr>) -> Self {
        Self {
            func: Box::new(func),
            args,
        }
    }
}


#[derive(Debug)]
pub struct Tuple {
    pub comps: Vec<Expr>,
}

impl Tuple {
    pub fn new(vals: Vec<Expr>) -> Self {
        Self {
            comps: vals
        }
    }
}

#[derive(Debug)]
pub struct Binding {
    ident: Varid,
    value: Box<Expr>
}

impl Binding {
    pub fn new(ident: Varid, value: Expr) -> Self {
        Self {
            ident,
            value: Box::new(value)
        }
    }
}

#[derive(Debug)]
pub struct Lambda {
    arg: Vec<Varid>,
    body: Box<Expr>
}

impl Lambda {
    pub fn new(arg: Vec<Varid>, body: Expr) -> Self {
        Self {
            arg,
            body: Box::new(body)
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Tuple(Tuple),
    Ident(Varid),
    Literal(Literal),
    Lambda(Lambda),
    Application (Application),
    Operator(Box<Expr>, Varid, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<Binding>, Box<Expr>),
    //LetRec(Vec<Binding>, Box<Expr>),
}


/*#[derive(Debug)]
pub struct Operator {

}*/


#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    Char(char),
    Byte(u8),
    Float(f64)
    // should add floats
}
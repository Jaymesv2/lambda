#[derive(Debug)]
pub enum Ast {
    Definitions(Vec<Binding>)
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
    // cant be a literal
    func: Box<Expr>,
    arg: Box<Expr>
}

impl Application {
    pub fn new(func: Expr, arg: Expr) -> Self {
        Self {
            func: Box::new(func),
            arg: Box::new(arg),
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
    arg: Varid,
    body: Box<Expr>
}

impl Lambda {
    pub fn new(arg: Varid, body: Expr) -> Self {
        Self {
            arg,
            body: Box::new(body)
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Ident(Varid),
    Literal(Literal),
    Lambda(Lambda),
    Application (Application)
}




#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    // should add floats
}
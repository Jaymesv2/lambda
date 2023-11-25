//! This is the abstract syntax tree for the language
//! All usable constructs and syntax sugar should be encoded here
//! 


use crate::ident_env::*;


#[derive(Debug, Clone)]
pub enum Program<Id> {
    // if its just a list of definitions its the main module
    Definitions(Vec<TopLevel<Id>>),
    /*Module {
        // None indicates there was no export list (it could be empty)
        exports: Option<Vec<Varid>>,
        //types: Vec<()>,
        definitions: Vec<Binding>,
    }*/
}

#[derive(Debug, Clone)]
pub enum TopLevel<Id> {
    TypeHint(TypeHint<Id>),
    Binding(Binding<Id>),
    TypeDef(TypeDef<Id>)
}

#[derive(Debug, Clone)]
pub struct TypeHint<Id> {
    pub ident: Id,
    pub ty: Type<Id>
}

#[derive(Debug, Clone)]
pub enum Type<Id> {
    Ident(Id),
    Generic(Id),
    Application(Box<Type<Id>>, Vec<Type<Id>>),
    Function(Box<Type<Id>>, Box<Type<Id>>),
    Tuple(Vec<Type<Id>>)
}

/*
Patterns:
    Patterns match when a 
    patterns can contain wildcards `_`

    Kinds of patterns
        Atomic patterns:
            variable:
                x
            wildcard:
                _
            Literal:
                char | int | string
            Tuple patterns:.
                (<Pattern*>)
            X - List destruction pattern
                (<Pattern>:xs) 

        Data pattern:
            AtomicPattern
            Pattern:
                <Data Constructor name> <patterns*>
        Pattern:

    Alternatives:
    
*/

#[derive(Debug, Clone)]
pub enum Pattern<Id> {
    // atomics
    Literal(Literal),
    Variable(Id),
    Wildcard,
    Tuple(Vec<Pattern<Id>>),
    
    // ConPattern
    ConPattern(Id, Vec<Pattern<Id>>)
}
#[derive(Debug, Clone)]
// can have guards
pub struct Alternative<Id> {
    pub pattern: Pattern<Id>,

    pub expr: Box<ParseExpr<Id>>
}







// Vector String
// Vector<String>

// Monad m => m a

#[derive(Debug, Clone)]
pub struct Binding<Id> {
    pub ident: Id,
    pub value: Box<ParseExpr<Id>>
}

#[derive(Debug, Clone)]
pub struct TypeDef<Id> {
    pub ident: Id,
    pub args: Vec<Id>,   
    pub constructors: Vec<DataConstructor<Id>>
}

#[derive(Debug, Clone)]
pub struct DataConstructor<Id> {
    pub ident: Id,
    pub args: Vec<Type<Id>>,
}



#[derive(Debug, Clone)]
pub enum ParseExpr<Id> {
    // atomic
    Tuple(Vec<ParseExpr<Id>>),
    Ident(Id),
    DataConIdent(Id),
    Literal(Literal),
    // infix
    Lambda {
        args: Vec<Id>,
        body: Box<ParseExpr<Id>>
    },
    Operator {
        lhs: Box<ParseExpr<Id>>, 
        op: Id, 
        rhs: Box<ParseExpr<Id>>
    },
    // application
    Application {
        func: Box<ParseExpr<Id>>,
        args: Vec<ParseExpr<Id>>
    },
    // expr
    If {
        cond: Box<ParseExpr<Id>>, 
        then: Box<ParseExpr<Id>>, 
        // else is a keyword :(
        r#else: Box<ParseExpr<Id>>
    },
    Let(Vec<Binding<Id>>, Box<ParseExpr<Id>>),
    Case {
        scrutinee: Box<ParseExpr<Id>>,
        alternatives: Vec<Alternative<Id>>,
    }
    //LetRec(Vec<Binding>, Box<Expr>),
}


#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    Char(char),
    Byte(u8),
    Float(f64)
    // should add floats
}
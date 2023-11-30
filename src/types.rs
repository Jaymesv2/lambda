#[derive(Debug, Clone)]
pub enum Type<Id> {
    Ident(Id),
    Generic(Id),
    Application(Box<Type<Id>>, Vec<Type<Id>>),
    Function(Box<Type<Id>>, Box<Type<Id>>),
    Tuple(Vec<Type<Id>>),
    Hole
}

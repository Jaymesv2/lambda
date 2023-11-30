use crate::{parser::parse_tree::*, scoped_map::*, types::*};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
};
/*
    Top level declaration must be unique in the current scope.

    Variables need not be unique while Top level declaration like
        type constructors,
        data constructors, functions, etc.. must be unique


for each block of bindings (top level or )
go over each of them

Step:
    pass over everything rename all of top level names along with bringing them into scope.
    recursivly enter each expression and rename bindings

If ident is used it must either be defined in the file (previously or in scope)



exprs in bindings depend on data constructors and other bindings

typdefs depend on other types so 

*/

type Result<T> = std::result::Result<T, ()>;


pub fn rename(x: Program<String>, env: &mut Env) -> Result<Program<NameId>> {
    let Program::Definitions(s) = x else {
        unimplemented!("modules are unsupported");
    };

    // split these up
    let mut bindings = vec![];
    let mut type_defs = vec![];
    let mut type_hints = vec![];
    for x in s {
        match x {
            TopLevel::Binding(b) => bindings.push(b),
            TopLevel::TypeDef(t) => type_defs.push(t),
            // ignore type hints for now
            TopLevel::TypeHint(t) => type_hints.push(t),
        }
    }

    // 1. import definitions 
    //      for builtints
    //      the prelude
    //      from imports

    // 2. bring all definitions into scope
    //      Type constructors
    //      Data constructors
    //      bindings

    // 3. rename
    //      1. Type constructors
    //      2. Data constructors
    //      3. bindings
    //      5. binding bodies
    //      4. type hints
    

    let type_defs = type_defs
        .into_iter()
        .map(|def| env.define_type(&def).map(|s| (s, def)))
        .collect::<Result<Vec<_>>>()?;

    let type_defs = type_defs
        .into_iter()
        .map(|(name, def)| {
            Ok(TypeDef {
                ident: name,
                args: if def.args.len() == 0 {
                        vec![]
                    } else {
                        //return Err(());
                        unimplemented!()
                    },
                constructors: def.constructors.into_iter().map(|con| rename_data_constructor(con, env)).collect::<Result<Vec<_>>>()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    //let type_defs = type_defs.into_iter().map();

    let bindings = rename_bindings(bindings, env)?;
    
    //let type_hints = type_hints.into_iter()

    Ok(Program::Definitions( 
        type_defs.into_iter().map(TopLevel::TypeDef)
            .chain(
                bindings.into_iter().map(TopLevel::Binding)
            )
            /*/.chain(
                type_hints.into_iter().map(TopLevel::TypeHint)
            )*/
        .collect()
    ))
}

fn rename_bindings(
    bindings: Vec<Binding<String>>,
    env: &mut Env,
) -> Result<Vec<Binding<NameId>>> {
    let bindings = bindings
        .into_iter()
        .map(|bind| env.define_var(bind.ident.clone()).map(|s| (s, bind)))
        .collect::<Result<Vec<_>>>()?;

    bindings
        .into_iter()
        .map(|(ident, id)| {
            Ok(Binding {
                ident,
                value: {
                    env.enter_scope();
                    let s = rename_expr(*id.value, env);
                    env.exit_scope();
                    Box::new(s?)
                },
            })
        })
        .collect()
}


fn rename_expr(expr: ParseExpr<String>, env: &mut Env) -> Result<ParseExpr<NameId>> {
    match expr {
        ParseExpr::Tuple(idents) => idents
                .into_iter()
                .map(|i| rename_expr(i, env))
                .collect::<Result<Vec<_>>>().map(ParseExpr::Tuple),
        ParseExpr::Ident(id) => env.reference_var(&id).ok_or(()).map(ParseExpr::Ident),
        ParseExpr::Application { func, args } => 
            Ok(ParseExpr::Application { func: Box::new(rename_expr(*func, env)?), args: args.into_iter().map(|x|rename_expr(x, env)).collect::<Result<_>>()? })
        ,
        ParseExpr::Literal(l) => Ok(ParseExpr::Literal(l)),
        ParseExpr::Case {
            scrutinee,
            alternatives,
        } => unimplemented!(),
        ParseExpr::DataConIdent(id) => unimplemented!(),
        ParseExpr::If { cond, then, r#else } => Ok(ParseExpr::If {cond : Box::new(rename_expr(*cond, env)?), then: Box::new(rename_expr(*then, env)?), r#else: Box::new(rename_expr(*r#else, env)?)}),
        ParseExpr::Let(bindings, expr) => rename_bindings(bindings, env)
            .and_then(|binds| rename_expr(*expr, env).map(|x| ParseExpr::Let(binds, Box::new(x)))),
        ParseExpr::Lambda { args, body } => {
            env.enter_scope();
            let s = ParseExpr::Lambda { args: args.into_iter().map(|x| env.define_var(x)).collect::<Result<_>>()?, body: Box::new(rename_expr(*body, env)?) };
            env.exit_scope();
            Ok(s)
        },
        ParseExpr::Operator { lhs, op, rhs } => {
            Ok(ParseExpr::Operator { lhs: Box::new(rename_expr(*lhs, env)?), op: env.reference_var(&op).ok_or(())?, rhs: Box::new(rename_expr(*rhs,env)?) })
        },
    }
}

fn rename_typedefs(defs: Vec<TypeDef<String>>, env: &mut Env) -> Result<Vec<TypeDef<NameId>>> {
    unimplemented!()
}

fn rename_data_constructor(
    con: DataConstructor<String>,
    env: &mut Env,
) -> Result<DataConstructor<NameId>> {
    unimplemented!()
}

fn rename_typehints(
    hints: Vec<TypeHint<String>>,
    env: &mut Env,
) -> Result<Vec<TypeHint<NameId>>> {
    hints
        .into_iter()
        .map(|hint| {
            Ok(TypeHint {
                ident: {
                    let Some(x) = env.reference_var(&hint.ident) else {
                        return Err(());
                    };
                    x
                },
                ty: rename_type(hint.ty, env)?,
            })
        })
        .collect()
}

fn rename_type(ty: Type<String>, env: &mut Env) -> Result<Type<NameId>> {
    Ok(match ty {
        Type::Generic(_) => unimplemented!(),
        Type::Application(a, b) => Type::Application(Box::new(rename_type(*a, env)?), b.into_iter().map(|x| rename_type(x, env)).collect::<Result<_>>()?),
        Type::Function(a, b) => Type::Function(Box::new(rename_type(*a, env)?), Box::new(rename_type(*b, env)?)),
        Type::Tuple(xs) => Type::Tuple(xs.into_iter().map(|x| rename_type(x, env)).collect::<Result<_>>()?),
        Type::Ident(ident) => Type::Ident(env.reference_type(ident).ok_or(())?),
        Type::Hole => Type::Hole,
    })
}

#[derive(Debug, Clone)]
pub enum NameId {
    TypeId(usize),
    VarId(usize)
}

#[derive(Debug, Clone)]
pub struct Name {}

#[derive(Debug, Clone)]
pub struct Env {
    /*/// maps the name of the data constructor to its id
    pub data_constructor_map: HashMap<String, DataConId>,

    pub data_constructors: Vec<DataConData>,*/
    /// maps the name of the type to its id
    type_map: HashMap<String, NameId>,
    /// Contains the types.
    types: Vec<TypeData>,

    //in_scope_generics: 

    binding_map: ScopedMap<String, NameId>,
    bindings: Vec<NameData>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            type_map: HashMap::new(),
            types: Vec::new(),
            binding_map: ScopedMap::new(),
            bindings: Vec::new(),
        }
    }

    pub fn define_type(&mut self, ident: &TypeDef<String>) -> Result<NameId> {
        let mut e = self.type_map.entry(ident.ident.clone());

        let Entry::Vacant(entry) = e else {
            return Err(());
        };
        /*
        for con in &ident.constructors {
            if self.data_constructor_map.contains_key(&con.ident) {
                return Err(());
            }
        }
         */

        let t = TypeData {
            name: ident.ident.clone(),
            constructors: Vec::with_capacity(ident.constructors.len()),
        };
        let tid = NameId::TypeId(self.types.len());
        self.types.push(t);
        entry.insert(tid);

        /*for con in &ident.constructors {
            let Entry::Vacant(entry) = self.data_constructor_map.entry(con.ident.clone()) else {
                return Err(());
            };


            //con.args

        }*/

        //let entry.insert(t)

        Err(())
    }

    pub fn reference_type(&mut self, ident: String) -> Option<NameId> {
        unimplemented!()
    }

    pub fn define_var(&mut self, ident: String) -> Result<NameId> {
        let idx = self.bindings.len();
        self.bindings.push(NameData {
            ident: ident.clone()
        });
        self.binding_map.insert(ident, NameId::VarId(idx));
        return Ok(NameId::VarId(idx));
        //self.bindings.push()
    }

    pub fn reference_var(&mut self, ident: &String) -> Option<NameId> {
        self.binding_map.get(ident).cloned()
    }

    pub fn enter_scope(&mut self) {
        self.binding_map.enter_scope();
    }

    pub fn exit_scope(&mut self) {
        self.binding_map.exit_scope();
    }

}

// the index of the type
#[derive(Debug, Clone)]
pub struct TypeId(usize);

#[derive(Debug, Clone)]
struct TypeData {
    pub name: String,
    // how to do arguments

    //args: V
    pub constructors: Vec<Id>,
}

#[derive(Debug, Clone)]
struct DataConData {
    pub name: String,
    pub type_id: TypeId,
    // how to handle the
    //pub args: Vec<>
}

#[derive(Debug, Clone)]
struct NameData {
    ident: String,
}

#[derive(Clone)]
pub struct Id {
    name: String,
    id: usize,
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.name, self.id)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
struct IdentData {
    id: usize,
    display_name: String,
    bound: bool,
}

/*
#[derive(Debug)]
pub struct IdentEnv {
    // maps strings to variable ids.
    map: ScopedMap<String, usize>,
    vars: Vec<IdentData>,
    //current_unused_id: usize
    //var_map: HashMap<String, usize>,
    //scopes: Vec<Vec<(usize, Option<usize>)>>,
    //vars: Vec<IdentData>,
}

impl IdentEnv {
    pub fn new() -> Self {
        Self {
            // maps a name to a varid
            map: ScopedMap::new(),
            vars: Vec::new(),
            //current_unused_id: 0
            //var_map: HashMap::new(),
            //scopes: Vec
        }
    }

    /// If the variable does not exist
    ///
    ///
    pub fn reference(&mut self, name: &str) -> Id {
        let s = name.to_string();
        if let Some(id) = self.map.get(&s) {
            Id {
                id: *id,
                name: s
            }
        } else {
            let id = self.vars.len();
            let s = IdentData {
                id,
                display_name: name.to_string(),
                bound: false
            };
            self.vars.push(s);
            Id {
                id,
                name: name.to_string(),
            }
        }

    }

    /// binds a variable
    ///
    /// If a variable is bound and it already exists then
    pub fn bind(&mut self, name: &str) -> Id {
        let name = name.to_string();
        if let Some(s) = self.map.get(&name) {
            self.vars[*s].bound = true;
            return Id {
                name,
                id: *s
            }
        }

        let id = self.vars.len();
        let s = IdentData {
            id,
            display_name: name.to_string(),
            bound: true
        };
        self.vars.push(s);
        self.map.insert(name.to_string(), id);
        Id {
            id,
            name: name.to_string(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.map.enter_scope();
    }

    pub fn leave_scope(&mut self) {
        self.map.leave_scope();
    }

}
 */

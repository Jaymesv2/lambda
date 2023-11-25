use crate::scoped_map::*;
use std::fmt::Debug;



fn rename() {
    
}



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

/*
#[derive(Debug)]
enum VarData {
    Known {

    },
    Unknown {
    }
}
 */

#[derive(Debug)]
struct IdentData {
    id: usize,
    display_name: String,
    bound: bool
}





/// Variable renaming
/// 
/// Each variable is given a unique id
/// 
/// Variables can only be introduced in:
/// - let
/// - lambda

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
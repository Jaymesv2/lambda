use std::collections::HashMap;
use crate::scoped_map::*;
/// Variables are marked as defined if they exist
/// 
/// 




#[derive(Debug)]
pub struct Id {
    name: String,
    uid: usize,
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
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
    current_unused_id: usize


    //var_map: HashMap<String, usize>,
    //scopes: Vec<Vec<(usize, Option<usize>)>>,
    //vars: Vec<IdentData>,


}



impl IdentEnv {
    pub fn new() -> Self {
        Self {
            map: ScopedMap::new(),
            vars: Vec::new(),
            current_unused_id: 0
            //var_map: HashMap::new(),
            //scopes: Vec
        }
    }

    /// for referencing a variable
    /// 
    pub fn reference(&self, name: &str) -> Id {
        unimplemented!()
    }

    /// binds a variable
    /// 
    /// If a variable is bound and it already exists then 
    pub fn bind(&mut self, name: &str) -> Id {
        /*if let Some(uid) = self.var_map.get(name) {
            return Id {
                name: name.to_string(),
                uid: *uid
            };
        };*/
        

        //self.map.


        unimplemented!()
    }

    pub fn enter_scope(&mut self) {
        self.map.enter_scope();
        //self.scopes.push(vec![]);
    }

    pub fn leave_scope(&mut self) {
        self.map.leave_scope();
    }

}


use std::collections::HashMap;
use either::*;
use std::hash::Hash;
/// Ideas for scoped map:
/// 
/// 1. use a persistent HAMT for 



#[derive(Debug)]
pub struct ScopedMap<K,V> {
    map: HashMap<K,V>,
    // None acts as a marker 
    scopes: Vec<Either<(K, V), ()>>,
    //scope_id: usize,
}

impl<K: Eq + Hash + Clone, V> ScopedMap<K,V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            scopes: vec![],
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        let x = self.map.insert(key.clone(),value);
        if let Some(s) = x {
            self.scopes.push(Left((key, s)));
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /*pub fn remove(&mut self, key: &K) -> &V {
        unimplemented!()
    }*/

    pub fn enter_scope(&mut self) {
        self.scopes.push(Right(()));
        //self.scopes.push(None);
    }

    pub fn leave_scope(&mut self) {
        while let Some(Left((k,v))) = self.scopes.pop() {
            let _ = self.map.insert(k,v);
        }
    }
}


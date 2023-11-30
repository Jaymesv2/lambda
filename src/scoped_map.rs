use either::*;
use std::collections::HashMap;
use std::hash::Hash;
/// Ideas for scoped map:
///
/// 1. use a persistent HAMT for

#[derive(Debug, Clone)]
pub struct ScopedMap<K, V> {
    map: HashMap<K, V>,
    /// Right marks the beginning of a scope.
    ///
    /// Left contains a key and an optional value.
    /// If the Option<V> contains true the name was shadowed
    scopes: Vec<Either<(K, Option<V>), ()>>,
    //scope_id: usize,
}

impl<K: Eq + Hash + Clone, V> ScopedMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            scopes: vec![],
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        let x = self.map.insert(key.clone(), value);
        self.scopes.push(Left((key, x)));
    }

    /*pub fn insert_global(&mut self, key: K, value: V) {
        let x = self.map.insert(key.clone(),value);
    }*/

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /*pub fn remove(&mut self, key: &K) -> &V {
        unimplemented!()
    }*/

    pub fn scope_contains(&self, key: &K) -> bool {
        self.scopes.iter().rev().map_while(|x| if let Either::Left(s) = x {
            Some(&s.0)
        } else {
            None
        }).any(|x| x == key)
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Right(()));
        //self.scopes.push(None);
    }

    pub fn exit_scope(&mut self) {
        while let Some(Left((k, v))) = self.scopes.pop() {
            if let Some(v) = v {
                let _ = self.map.insert(k, v);
            }
        }
    }
}

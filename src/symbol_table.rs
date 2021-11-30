use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
struct SymbolTable<K, V> {
    env: HashMap<K, V>,
    // outer: Option<SymbolTable<K, V>>
}

impl<K: Hash + Eq, V> SymbolTable<K, V> {
    // fn new(outer: SymbolTable<K, V>) -> Self {
    //     SymbolTable { env: Default::default(), outer:Some(outer) }
    // }

    fn insert(& mut self, k: K, v: V) -> Option<V> {
        self.env.insert(k, v)
    }

    fn lookup(&self, k: &K) -> Option<&V> {
        self.env.get(k)
    }

    // fn enter(self) -> Self {
    //     SymbolTable::new(self)
    // }
    //
    // fn exit(self) -> Option<Self> {
    //     self.outer
    // }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::SymbolTable;

    #[test]
    fn test_add_symbol() {
        let mut env: SymbolTable<String, i32> = SymbolTable::default();
        env.insert("a".to_string(), 5);
        match env.lookup(&"a".to_string()) {
            Some(&v) => assert_eq!(v, 5),
            None => assert!(false, "{}", format!("Error env is: {:#?}", env))
        }
    }
}
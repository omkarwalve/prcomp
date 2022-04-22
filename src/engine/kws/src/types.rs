use std::{
    fmt::{self,Display,Debug},
};

//  ┌┬┐┌─┐┌─┐
//  │││├─┤├─┘
//  ┴ ┴┴ ┴┴  
#[derive(Clone)]
pub struct Map<K,V>
where K: Eq,
      V: Clone {
    list : Vec<(K,V)>,
}
impl<K: Eq,V: Clone> Map<K,V> {
    pub fn new() -> Self {
        Self {
            list: vec![]
        }
    }
    /// ### get
    /// Retrieve a value from the `Map` using a `key`. If `key` doesn't exist in the `Map` then `None` is returned.
    pub fn get(&self,key: K) -> Option<V> {
        match self.list.iter().find(|tuple| tuple.0 == key) {
            Some(tuple) => Some(tuple.1.clone()),
            None => None
        }
    }
    /// ### push
    /// Push a `key` `value` pair into the `Map`.
    pub fn push(&mut self,key: K,val: V) {
        self.list.push((key,val));
    }
}

impl<V: Clone> From<Vec<V>> for Map<usize,V> {
    fn from(vec: Vec<V>) -> Self {
        Map { list: vec.into_iter()
                       .enumerate()
                       .collect() }
    }
}
impl<K: Eq,V: Clone> From<Vec<(K,V)>> for Map<K,V> {
    fn from(vec: Vec<(K,V)>) -> Self {
        Map { list: vec }
    }
}

impl<K: Eq + Debug,V: Clone + Debug> Debug for Map<K,V> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maps = self.list.iter().map(|(k,v)| format!("{:?} => {:?}",k,v)).collect::<Vec<String>>();
        write!(f,"{{\n\t{}\n\t}}",maps.join("\n\t"))
    }

}
impl<K: Eq + Display,V: Clone + Display> Display for Map<K,V> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maps = self.list.iter().map(|(k,v)| format!("{} => {}",k,v)).collect::<Vec<String>>();
        write!(f,"{{\n\t{}\n}}",maps.join("\n"))
    }

}
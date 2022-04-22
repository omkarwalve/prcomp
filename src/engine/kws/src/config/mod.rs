use crate::Map;

#[derive(Clone)]
enum ConfigValue {
    Array(Vec<String>),
    Number(usize),
    String(String),
}
pub struct Configuration<S: Eq> {
    map: Map<S,ConfigValue>,
}

pub trait Configurable {
}
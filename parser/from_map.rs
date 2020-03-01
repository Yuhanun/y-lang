pub use std::collections::HashMap;

pub trait FromMap<T> {
    fn from(map: &HashMap<&'static str, T>, data: String) -> Self;
    fn skip(&self) -> bool;
}

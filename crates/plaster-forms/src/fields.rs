pub mod big_checkbox;
pub mod checkbox;
pub mod file;
pub mod key_value;
pub mod select;
pub mod text;

use std::sync::Arc;

#[derive(Clone)]
pub struct ValidationFn<V> {
    func: Arc<dyn Fn(V) -> Option<String>>,
}

impl<V> ValidationFn<V> {
    pub fn validate(&self, i: V) -> Option<String> {
        (self.func)(i)
    }
}

impl<V> Default for ValidationFn<V> {
    fn default() -> ValidationFn<V> {
        ValidationFn {
            func: Arc::new(|_: V| None),
        }
    }
}

impl<V> PartialEq for ValidationFn<V> {
    fn eq(&self, _other: &ValidationFn<V>) -> bool {
        true
    }
}

impl<V, FN> From<FN> for ValidationFn<V>
where
    FN: Fn(V) -> Option<String> + 'static,
{
    fn from(f: FN) -> ValidationFn<V> {
        ValidationFn { func: Arc::new(f) }
    }
}

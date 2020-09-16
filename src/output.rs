use crate::schema::Type;
use std::any::Any;
use std::collections::HashMap;

pub trait Output: Sized + 'static {
    fn to_any_map(self) -> HashMap<String, Box<dyn Any>>;
    fn schema() -> HashMap<String, Type>;
}

pub struct OutputSingular<T: 'static> {
    pub val: T,
}

impl<T: 'static> Output for OutputSingular<T> {
    fn to_any_map(self) -> HashMap<String, Box<dyn Any>> {
        let mut map = HashMap::new();
        map.insert(
            "val".to_owned(),
            ::std::boxed::Box::new(self.val) as ::std::boxed::Box<dyn ::std::any::Any>,
        );
        map
    }
    fn schema() -> HashMap<String, Type> {
        let mut map = HashMap::new();
        map.insert(
            "val".to_owned(),
            Type {
                name: ::std::any::type_name::<T>().to_owned(),
            },
        );
        map
    }
}

impl<T> From<T> for OutputSingular<T> {
    fn from(val: T) -> Self {
        Self { val }
    }
}

impl Output for () {
    fn to_any_map(self) -> HashMap<String, Box<dyn Any>> {
        HashMap::new()
    }
    fn schema() -> HashMap<String, Type> {
        HashMap::new()
    }
}

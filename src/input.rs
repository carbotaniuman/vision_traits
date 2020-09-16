use crate::DeserializationError;
use crate::schema::Type;
use std::any::Any;
use std::collections::HashMap;

pub trait Input<'a>: Sized {
    fn from_any_map(any_map: &'a HashMap<String, &dyn Any>) -> Result<Self, DeserializationError>;
    fn schema() -> HashMap<String, Type>;
}

pub struct InputSingular<'a, T: 'static> {
    pub val: &'a T,
}

impl<'a, T: 'static> Input<'a> for InputSingular<'a, T> {
    fn from_any_map(map: &'a HashMap<String, &dyn Any>) -> Result<Self, DeserializationError> {
        Ok(Self {
            val: map.get("val").ok_or_else(|| DeserializationError::MissingField("val".to_owned()))?.downcast_ref::<T>().ok_or_else(|| DeserializationError::TypeError("val".to_owned()))?,
        })
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

impl Input<'_> for () {
    fn from_any_map(_: &HashMap<String, &dyn Any>) -> Result<Self, DeserializationError> {
        Ok(())
    }
    fn schema() -> HashMap<String, Type> {
        HashMap::new()
    }
}

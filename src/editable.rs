use crate::DynErrResult;
use crate::schema::SettingType;
use json::JsonValue;
use std::collections::HashMap;
use std::error::Error;

pub trait Editable: Sized + 'static {
    fn schema() -> SettingType;
    fn deserialize(input: &JsonValue) -> DynErrResult<Self>;
}

macro_rules! editable_integral {
    ($ty:ident => $method:ident) => {
        impl Editable for $ty {
            fn schema() -> SettingType {
                let mut map = HashMap::new();
                map.insert("min".to_owned(), ($ty::MIN as f64).into());
                map.insert("max".to_owned(), ($ty::MAX as f64).into());

                SettingType {
                    name: stringify!($ty).to_owned(),
                    params: map,
                }
            }
            fn deserialize(input: &JsonValue) -> DynErrResult<Self> {
                Ok(input.$method().ok_or(concat!("input could not be deserialized into ", stringify!($ty)))?)
            }
        }
    };
}

macro_rules! editable {
    ($ty:ident => $method:ident) => {
        impl Editable for $ty {
            fn schema() -> SettingType {
                SettingType {
                    name: stringify!($ty).to_owned(),
                    params: HashMap::new(),
                }
            }
            fn deserialize(input: &JsonValue) -> Result<Self, Box<dyn Error>> {
                Ok(input.$method().ok_or(concat!("input could not be deserialized into", stringify!($ty)))?)
            }
        }
    };
}

editable_integral!(u8 => as_u8);
editable_integral!(u16 => as_u16);
editable_integral!(u32 => as_u32);
editable_integral!(i8 => as_i8);
editable_integral!(i16 => as_i16);
editable_integral!(i32 => as_i32);

editable!(f64 => as_f64);
editable!(bool => as_bool);

impl Editable for String {
    fn schema() -> SettingType {
        SettingType {
            name: "string".to_owned(),
            params: HashMap::new(),
        }
    }
    fn deserialize(input: &JsonValue) -> Result<Self, Box<dyn Error>> {
        Ok(input.as_str().map(|e| e.to_owned()).unwrap())
    }
}

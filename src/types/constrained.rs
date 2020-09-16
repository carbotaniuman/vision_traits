use crate::editable::Editable;
use crate::schema::SettingType;
use json::JsonValue;
use std::collections::HashMap;
use std::error::Error;

macro_rules! constrained {
    ($ty:ident => $method:ident) => {
        paste::item! {
            #[repr(transparent)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct [<Constrained $ty:camel>]<const MIN: $ty, const MAX: $ty, const BOUNDS_INCLUSIVE: bool>($ty);
            impl<const MIN: $ty, const MAX: $ty, const BOUNDS_INCLUSIVE: bool> Editable for [<Constrained $ty:camel>]<MIN, MAX, BOUNDS_INCLUSIVE> {
                fn schema() -> SettingType {
                    let mut map = HashMap::new();
                    map.insert("min".to_owned(), $ty::MIN.into());
                    map.insert("max".to_owned(), $ty::MAX.into());

                    SettingType {
                        name: stringify!([<Constrained $ty:camel>]).to_owned(),
                        params: map,
                    }
                }
                fn deserialize(input: &JsonValue) -> Result<Self, Box<dyn Error>> {
                    Ok(input.$method().filter(|e| &MIN <= e && e <= &MAX).map(|e| [<Constrained $ty:camel>](e)).unwrap())
                }
            }
        }
    };
}

constrained!(u8 => as_u8);
constrained!(u16 => as_u16);
constrained!(u32 => as_u32);
constrained!(i8 => as_i8);
constrained!(i16 => as_i16);
constrained!(i32 => as_i32);

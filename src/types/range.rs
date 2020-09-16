use crate::editable::Editable;
use crate::schema::SettingType;
use json::JsonValue;
use std::collections::HashMap;
use std::error::Error;

macro_rules! range {
    ($ty:ident => $method:ident) => {
        paste::item! {
            #[repr(C)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct [<Range $ty:camel>]<const MIN: $ty, const MAX: $ty> {
                pub min: $ty,
                pub max: $ty,
            }
            impl<const MIN: $ty, const MAX: $ty> Editable for [<Range $ty:camel>]<MIN, MAX> {
                fn schema() -> SettingType {
                    // Suboptimal, I know
                    if(MIN > MAX) {
                        panic!("MIN must be less than MAX");
                    }
                    let mut map = HashMap::new();
                    map.insert("min".to_owned(), MIN.into());
                    map.insert("max".to_owned(), MAX.into());

                    SettingType {
                        name: stringify!([<Range $ty:camel>]).to_owned(),
                        params: map,
                    }
                }
                fn deserialize(input: &JsonValue) -> Result<Self, Box<dyn Error>> {
                    // Suboptimal, I know
                    if(MIN > MAX) {
                        panic!("MIN must be less than MAX");
                    }
                    let min = input["min"].$method().filter(|e| &MIN <= e && e <= &MAX).unwrap();
                    let max = input["max"].$method().filter(|e| &MIN <= e && e <= &MAX).unwrap();
                    Ok([<Range $ty:camel>]{ min, max })
                }
            }
        }
    };
}

range!(u8 => as_u8);
range!(u16 => as_u16);
range!(u32 => as_u32);
range!(i8 => as_i8);
range!(i16 => as_i16);
range!(i32 => as_i32);

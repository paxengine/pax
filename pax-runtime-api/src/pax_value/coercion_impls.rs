// ------------------------------- Coersion rules ----------------------------------
// default coercion only allows a single type: the type expected
// custom coercion rules can be implemented by a type

use crate::{
    impl_default_coercion_rule, Color, ColorChannel, Fill, Numeric, PaxValue,
    Percent, Property, Rotation, Size, Stroke, Transform2D,
};

// Default coersion rules:
// call Into::<first param>::into() on contents of second enum variant
impl_default_coercion_rule!(bool, PaxValue::Bool);

impl_default_coercion_rule!(u8, PaxValue::Numeric);
impl_default_coercion_rule!(u16, PaxValue::Numeric);
impl_default_coercion_rule!(u32, PaxValue::Numeric);
impl_default_coercion_rule!(u64, PaxValue::Numeric);

impl_default_coercion_rule!(i8, PaxValue::Numeric);
impl_default_coercion_rule!(i16, PaxValue::Numeric);
impl_default_coercion_rule!(i32, PaxValue::Numeric);
impl_default_coercion_rule!(i64, PaxValue::Numeric);

impl_default_coercion_rule!(f32, PaxValue::Numeric);
impl_default_coercion_rule!(f64, PaxValue::Numeric);

impl_default_coercion_rule!(isize, PaxValue::Numeric);
impl_default_coercion_rule!(usize, PaxValue::Numeric);

// Pax internal types
impl_default_coercion_rule!(Color, PaxValue::Color);
impl_default_coercion_rule!(Transform2D, PaxValue::Transform2D);

pub trait CoercionRules
where
    Self: Sized + 'static,
{
    fn try_coerce(value: PaxValue) -> Result<Self, String>;
}

// Fill is a type that other types (Color) can be coerced into, thus the default
// from to pax macro isn't enough (only translates directly back and forth, and returns
// an error if it contains any other type)
impl CoercionRules for Fill {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::Color(color) => Fill::Solid(color),
            PaxValue::Fill(fill) => fill,
            _ => return Err(format!("{:?} can't be coerced into a Fill", pax_value)),
        })
    }
}

impl CoercionRules for Stroke {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::Color(color) => Stroke {
                color: Property::new(color),
                width: Property::new(Size::Pixels(1.into())),
            },
            PaxValue::Stroke(stroke) => stroke,
            _ => return Err(format!("{:?} can't be coerced into a Stroke", pax_value)),
        })
    }
}

impl CoercionRules for ColorChannel {
    fn try_coerce(value: PaxValue) -> Result<Self, String> {
        Ok(match value {
            PaxValue::ColorChannel(color) => color,
            PaxValue::Percent(perc) => ColorChannel::Percent(perc.0),
            PaxValue::Numeric(num) => ColorChannel::Integer(num),
            _ => return Err(format!("{:?} can't be coerced into a ColorChannel", value)),
        })
    }
}

impl CoercionRules for Percent {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::Percent(p) => p,
            PaxValue::Numeric(v) => Percent(v),
            _ => return Err(format!("{:?} can't be coerced into a Percent", pax_value)),
        })
    }
}

impl CoercionRules for Size {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::Size(size) => size,
            PaxValue::Percent(p) => Size::Percent(p.0),
            PaxValue::Numeric(v) => Size::Pixels(v),
            _ => return Err(format!("{:?} can't be coerced into a Size", pax_value)),
        })
    }
}

impl CoercionRules for String {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::String(s) => s,
            PaxValue::Numeric(n) => {
                if n.is_float() {
                    n.to_float().to_string()
                } else {
                    n.to_int().to_string()
                }
            }
            _ => return Err(format!("{:?} can't be coerced into a String", pax_value)),
        })
    }
}

impl CoercionRules for Rotation {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::Rotation(r) => r,
            PaxValue::Numeric(n) => Rotation::Degrees(n),
            PaxValue::Percent(p) => Rotation::Percent(p.0),
            _ => return Err(format!("{:?} can't be coerced into a Rotation", pax_value)),
        })
    }
}

impl CoercionRules for Numeric {
    fn try_coerce(pax_value: PaxValue) -> Result<Self, String> {
        Ok(match pax_value {
            PaxValue::Numeric(n) => n.into(),
            PaxValue::Size(n) => n.into(),
            _ => return Err(format!("{:?} can't be coerced into a Numeric", pax_value)),
        })
    }
}


impl<T: CoercionRules> CoercionRules for Vec<T> {
    fn try_coerce(value: PaxValue) -> Result<Self, String> {
        match value {
            PaxValue::Vec(vec) => {
                let res: Result<Vec<T>, _> = vec.into_iter().map(|v| T::try_coerce(v)).collect();
                res.map_err(|e| format!("couldn't coerce vec, element {:?}", e))
            }
            v => Err(format!(
                "{:?} can't be coerced into {:?}",
                v,
                std::any::type_name::<Vec<T>>(),
            )),
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(crate = "crate::serde")]
// pub enum PaxValue {
//     Bool(bool),
//     Numeric(Numeric),
//     String(String),
//     Transform2D(Transform2D),
//     Size(Size),
//     Percent(Percent), 
//     Color(Color),
//     ColorChannel(ColorChannel),
//     Rotation(Rotation),
//     Fill(Fill),
//     Stroke(Stroke),
//     Option(Box<Option<PaxValue>>),
//     // Ideally this is later changed to Vec<PaxValue>, once structs can be
//     // represented in PaxValue as a map, enabling serialize/deserialization
//     // debug impl, etc.
//     Vec(Vec<PaxValue>),
//     Object(HashMap<String, PaxValue>),
//     Enum(String, Vec<PaxValue>),
// }



impl<T: CoercionRules> CoercionRules for Option<T> {
    fn try_coerce(value: PaxValue) -> Result<Self, String> {
        match value {
            PaxValue::Option(opt) => {
                let res: Result<Option<T>, _> = opt.map(|v| T::try_coerce(v)).transpose();
                res.map_err(|e| format!("couldn't coerce option, element {:?}", e))
            }
            v => Err(format!(
                "{:?} can't be coerced into {:?}",
                v,
                std::any::type_name::<Option<T>>(),
            )),
        }
    }
}

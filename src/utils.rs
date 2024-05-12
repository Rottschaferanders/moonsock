// use serde::{Deserialize, Deserializer};
// use serde::de;


// pub fn deserialize_single_item_array<'de, D, T>(deserializer: D) -> Result<T, D::Error>
// where
//     D: Deserializer<'de>,
//     T: Deserialize<'de>,
// {
//     let v: Vec<T> = Deserialize::deserialize(deserializer)?;
//     if v.len() == 1 {
//         Ok(v.into_iter().next().unwrap())
//     } else {
//         Err(de::Error::custom("Expected an array with a single element"))
//     }
// }


// pub mod serde_button_event {
//     use crate::ButtonEventParam;
//     use serde::{Deserialize, Deserializer, Serialize, Serializer};
//     use std::fmt;

//     pub fn serialize<S>(param: &ButtonEventParam, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.collect_seq(std::iter::once(param))
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<ButtonEventParam, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct ButtonEventVisitor;

//         impl<'de> serde::de::Visitor<'de> for ButtonEventVisitor {
//             type Value = ButtonEventParam;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a single-element sequence")
//             }

//             fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::SeqAccess<'de>,
//             {
//                 let param = seq.next_element()?
//                     .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
//                 Ok(param)
//             }
//         }

//         deserializer.deserialize_seq(ButtonEventVisitor)
//     }
// }

// pub mod button_event_param_serde {
//     use serde::{Serialize, Serializer, Deserialize, Deserializer};
//     use serde::ser::SerializeSeq;
//     use crate::ButtonEventParam;

//     pub fn serialize<S>(param: &ButtonEventParam, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(1))?;
//         seq.serialize_element(param)?;
//         seq.end()
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<ButtonEventParam, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let params = Vec::<ButtonEventParam>::deserialize(deserializer)?;
//         if params.len() != 1 {
//             return Err(serde::de::Error::custom("expected a single-element array"));
//         }
//         Ok(params.into_iter().next().unwrap())
//     }
// }

pub mod single_element_array {
    use serde::{Serialize, Serializer, Deserialize, Deserializer};
    use serde::ser::SerializeSeq;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1))?;
        seq.serialize_element(value)?;
        seq.end()
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let params = Vec::<T>::deserialize(deserializer)?;
        if params.len() != 1 {
            return Err(serde::de::Error::custom("expected a single-element array"));
        }
        Ok(params.into_iter().next().unwrap())
    }
}
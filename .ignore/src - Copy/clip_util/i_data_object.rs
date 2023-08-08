// i_data_object IDataObject
#[allow(dead_code, unused, unused_imports)] // object creation (pre debug)
// !------------------------------------------------------------
// , PartialEq need manual implementation
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IDataObject {
    // &[u8]/Vec<u8>/[u8; N]	postgres::types::Type::BYTEA 
    i_data: Box<Blob>,
    // i_data: Box<dyn Any>,
    // i_data: Arc<Box<dyn Any>>,
    // i_data: Arc<Box<Blob>>,
    // serde uses Value enum: Option(Option<Box<Value>>) & Newtype(Box<Value>),     // ! todo fails with Serde!!!
    // i_data: Arc<Box<String>>, Mockup
    // Eq, PartialEq, ?
}

// impl Clone for IDataObject {
//     fn clone(&self) -> Self {
//         Self { i_data: self.i_data.clone() }
//     }
// }

impl IDataObject {}
impl Default for IDataObject {
    fn default() -> Self {
        IDataObject {
            i_data: Box::new(Blob::new()),
            // i_data: Box::new("".to_string()),
            // i_data: Arc::new(Box::new(Blob::new())),
        } // ! todo finalize Type
    }
}
impl Eq for IDataObject {}
impl PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        match self.i_data {
            // Format Type
            // ! todo Format requires advanced equality handling using Data Type
            _ => true, // ! normally false
        }
    }
}

// END OF CODE

// impl<'de> Deserialize<'de> for IDataObject {
// }

// impl Serialize for IDataObject {
// ! todo Custom Serialzation for Data Object Forat Types
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("IDataObject", 1)?;
//         state.serialize_field("i_data", &self)?;
//         state.end()
//     }
// }

// !------------------------------------------------------------
// Notes: IDataObject
/*
    int year,
    int month,
    int day,
    int hour,
    int minute,
    int second,
    int millisecond,
    Calendar calendar,
    DateTimeKind kind
*/
// !------------------------------------------------------------
    // fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    //     where
    //         D: serde::Deserializer<'de>, {
    // }
    // fn deserialize_i_data_object<V>(self, visitor: V) -> Result<V::Value>
    // where
    //     V: Visitor<'de>,
    // {
    //     serde::Deserializer::deserialize_newtype_struct(self, "IDataObject", visitor)
    //     // visitor.visit_some(deserializer)
    // }
    //
    // fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value>
    // where
    //     V: Visitor<'de>,
    // {
    //     visitor.visit_some(deserializer)
    // }
    //
    // !  serde::de for IDataObject Data Type
    // fn deserialize_i_data_object<D>(deserializer: D) -> Result<Self, D::Error>
    // where
    //     D: serde::Deserializer<'de>,
    //     // V: serde::de::Visitor<'de>,
    // {
    //     // Ok(IDataObject::default()) // Mockup
    //     // Serde's Vistor pattern might be needed here
    //     // deserializer.deserialize_option(IDataObjectVistor)
    //     // const FIELDS: &'static [&'static str] = &["i_data"];
    //     // deserializer.deserialize_struct("IDataObject", FIELDS, deserializer)
    //     serde::Deserializer::deserialize_newtype_struct(self, "IDataObject", visitor)
    //     // deserialize_any::<_>(deserializer, visitor)
    // }
// !------------------------------------------------------------
    // type Error = de::Error;
    // type Error = dyn serde::de::Error;
    // type Error1 = dyn std::error::Error;
// // ------------------
//     // Look at the input data to decide what Serde data model type to
//     // deserialize as. Not all data formats are able to support this operation.
//     // Formats that support `deserialize_any` are known as self-describing.
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         // V: Visitor<'de>,
//         D: serde::Deserializer<'de>,
//     {    
// // ------------------
//         // Field
//         enum Field {            // IDataObject
//             Field0,             //   i_data: Arc::new(Box::new("".to_string()))
//             Ignore,
//         }
// // ------------------

//         struct IDataObjectVistor;
//         impl<'de> Visitor<'de> for IDataObjectVistor {
//             type Value = IDataObject;
//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("Field Identifier")
//             }
//             fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
//             where
//                 D: serde::Deserializer<'de>,
//             {
//                 // self
//                 // let mut i_data_object = self.visit_some(deserializer)?;
//                 let i_data_object: IDataObject = IDataObject::default();
//                 Ok(i_data_object)
//                 // i_data_object
//             }
//             fn visit_newtype_struct<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
//             where
//                 D: serde::Deserializer<'de>,
//             {
//                 // Ok(self)
//                 let i_data_object: IDataObject = IDataObject::default();
//                 // self.visit_newtype_struct(deserializer).unwrap();
//                 Ok(i_data_object)
//                 // i_data_object
//             }
//         }

//         struct FieldVisitor;
//         impl<'de> Visitor<'de> for FieldVisitor {
//             type Value = Field;
//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("Data Object field")
//             }
//             fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//                 where
//                     E: serde::de::Error, {
//                         match value {
//                             0u64 => Ok(Field::Field0),
//                             _ => Ok(Field::Ignore),
//                         }
                
//             }
//             fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//                 where
//                     E: serde::de::Error, {
//                 const FIELDS: &'static [&'static str] = &["i_data"];
//                 match value {
//                     "i_data " => Ok(Field::Field0),
//                         // Ok(deserializer.deserialize_string(visitor)) // ;; .deserialize_struct("DataCreationTime", FIELDS, FieldVisitor))
//                         // Ok(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor))
//                         // Ok(&self.datetimeutc)
//                         // Ok(DateTime::parse_from_str(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor)))
//                         // DateTime::parse_from_str(&self.deserialize_struct("datetimeutc", FIELDS, FieldVisitor).unwrap()).unwrap()
//                         // DateTime::parse_from_str(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor)).unwrap()
//                     "IDataObject" => {
//                         Ok(
//                             Field::Ignore
//                             // deserializer.deserialize_struct("DataCreationTime", FIELDS, FieldVisitor)
//                         )
//                     }, // the whole struct???
//                     _ => Err(de::Error::unknown_field(value, FIELDS)),
//                     // "datetimeutc " => Ok(DataCreationTime::datetimeutc),
//                 }                
//             }
//         }

//         struct __Visitor<'de> {
//             marker: PhantomData<IDataObject>,
//             lifetime: PhantomData<&'de ()>
//         }
//         impl<'de> Visitor<'de> for __Visitor<'de> {
//             type Value = IDataObject;
//
//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("I Data Object")
//             }
//             // no types to process
//         }

//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(
//                 deserializer: D
//             ) -> Result<Self, D::Error>
//             where
//                 D: serde::Deserializer<'de>,
//             {
//                 serde::Deserializer::deserialize_identifier::<_>(deserializer, FieldVisitor)
//             }
//         }

//         deserializer.deserialize_any(IDataObjectVistor {})

//         // match Self.type_id() {
//         //         TypeId::of::<IDataObject> => self.deserialize_newtype_struct(self, "IDataObject", visitor),
//         //         TypeId::of::<SerializeStruct> => self.deserialize_struct(visitor),
//         //         TypeId::of::<String> => self.deserialize_string(visitor),
//         //         TypeId::of::<str> => self.deserialize_str(visitor),
//         //         // TypeId::of::<Unit> => self.deserialize_unit(visitor),
//         //         _ => Err(Error::Syntax),
//         //     }
//     }
// !------------------------------------------------------------

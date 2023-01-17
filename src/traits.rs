use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::{boxed::Box, vec::Vec};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// For more info refer to:
// https://github.com/fragcolor-xyz/shards/blob/devel/include/shards.h

#[cfg(not(feature = "std"))]
type String = Vec<u8>;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum TriState {
    Either,
    True,
    False,
}

/// Enum that represents the Code Type.
///
/// Note: There can only be 2 Code Types:
/// 1. A Shard
/// 2. A Wire
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum CodeType {
    /// A list of shards, to be injected into more complex blocks of code or wires
    Shards,
    /// An actual wire
    Wire { looped: TriState },
}

/// Struct represents the information about a Code (Note: There are only 2 Code Types: A Shard or a Wire. See the enum `CodeType` above to understand more)
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct CodeInfo {
    /// The Type of the Code (i.e the Code Type)
    pub kind: CodeType,
    /// List of variables that must be available to the Code's code context, before the Code even executes. Otherwise, the
    ///
    /// Note: Each variable is represented as a tuple of its name and its type.
    pub requires: Vec<(String, VariableType)>,
    /// List of variables that are in the Code's code context. Each variable is represented as a tuple of its name and its type.
    pub exposes: Vec<(String, VariableType)>,
    /// List of variable types that are inputted into the Code
    pub inputs: Vec<VariableType>,
    /// The variable type of the output of the Code
    pub output: VariableType,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct TableInfo {
    /// Tha name of the keys, an empty key represent any name, allowing multiple instances of the corresponding index type
    pub keys: Vec<String>,
    /// Following keys array (should be same len), the types expected
    pub types: Vec<Vec<VariableType>>,
}

/// Enum represents all the possible types that a variable can be
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum VariableType {
    None,
    Any,
    /// VendorID, TypeID
    Enum {
        vendor_id: u32,
        type_id: u32,
    },
    Bool,
    /// A 64bits int
    Int,
    /// A vector of 2 64bits ints
    Int2,
    /// A vector of 3 32bits ints
    Int3,
    /// A vector of 4 32bits ints
    Int4,
    /// A vector of 8 16bits ints
    Int8,
    /// A vector of 16 8bits ints
    Int16,
    /// A 64bits float
    Float,
    /// A vector of 2 64bits floats
    Float2,
    /// A vector of 3 32bits floats
    Float3,
    /// A vector of 4 32bits floats
    Float4,
    /// A vector of 4 uint8
    Color,
    // Non Blittables
    Bytes,
    String,
    Image,
    Seq(Vec<VariableType>),
    Table(TableInfo),
    /// VendorID, TypeID
    Object {
        vendor_id: u32,
        type_id: u32,
    },
    Audio,
    Code(Box<CodeInfo>),
    Mesh,
    Channel(Box<VariableType>),
}

/// Struct contains information about a variable type
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct VariableTypeInfo {
    /// The variable type
    #[cfg_attr(feature = "std", serde(alias = "type"))]
    pub type_: VariableType,
    /// Raw-bytes representation of the default value of the variable type (optional)
    pub default: Option<Vec<u8>>,
}

/// TODO Review - Definition
/// A Trait Attribute's Type
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum RecordInfo {
    SingleType(VariableTypeInfo),
    MultipleTypes(Vec<VariableTypeInfo>),
}

/// Struct represents a Trait
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct Trait {
    /// Name of the Trait
    pub name: String,
    /// List of attributes of the Trait. An attribute is represented as a **tuple that contains the attribute's name and the attribute's type**.
    pub records: Vec<(String, RecordInfo)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_simple_1() {
        let mut trait1 = vec![(
            "int1".to_string(),
            RecordInfo::SingleType(VariableTypeInfo {
                type_: VariableType::Int,
                default: None,
            }),
        )];

        // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
        trait1 = trait1
            .into_iter()
            .map(|(name, info)| (name.to_lowercase(), info))
            .collect();
        trait1.dedup_by(|a, b| a.0 == b.0);
        // Note: "Strings are ordered lexicographically by their byte values ... This is not necessarily the same as “alphabetical” order, which varies by language and locale". Source: https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str
        trait1.sort_by(|a, b| a.0.cmp(&b.0));

        let trait1 = Trait {
            name: "Trait1".to_string(),
            records: trait1,
        };

        let e_trait1 = trait1.encode();

        let d_trait1 = Trait::decode(&mut e_trait1.as_slice()).unwrap();

        assert!(trait1 == d_trait1);
    }

    #[test]
    fn encode_decode_boxed_1() {
        let mut trait1 = vec![
            (
                "int1".to_string(),
                RecordInfo::SingleType(VariableTypeInfo {
                    type_: VariableType::Int,
                    default: None,
                }),
            ),
            (
                "boxed1".to_string(),
                RecordInfo::SingleType(VariableTypeInfo {
                    type_: VariableType::Code(Box::new(CodeInfo {
                        kind: CodeType::Wire {
                            looped: TriState::Either,
                        },
                        requires: vec![("int1".to_string(), VariableType::Int)],
                        exposes: vec![],
                        inputs: vec![],
                        output: VariableType::None,
                    })),
                    default: None,
                }),
            ),
        ];

        // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
        trait1 = trait1
            .into_iter()
            .map(|(name, info)| (name.to_lowercase(), info))
            .collect();
        trait1.dedup_by(|a, b| a.0 == b.0);
        trait1.sort_by(|a, b| a.0.cmp(&b.0));

        let trait1 = Trait {
            name: "Trait1".to_string(),
            records: trait1,
        };

        let e_trait1 = trait1.encode();

        let d_trait1 = Trait::decode(&mut e_trait1.as_slice()).unwrap();

        assert!(trait1 == d_trait1);
        assert!(d_trait1.records[0].0 == "boxed1".to_string());
        let requires = match d_trait1.records[0].1 {
            RecordInfo::SingleType(VariableTypeInfo {
                type_: VariableType::Code(ref code),
                default: None,
            }) => code.requires.clone(),
            _ => panic!("Expected a code"),
        };
        assert!(requires[0].0 == "int1".to_string());
    }

    #[test]
    fn test_json_simple_1() {
        let mut trait1 = vec![(
            "int1".to_string(),
            RecordInfo::SingleType(VariableTypeInfo {
                type_: VariableType::Int,
                default: None,
            }),
        )];

        // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
        trait1 = trait1
            .into_iter()
            .map(|(name, info)| (name.to_lowercase(), info))
            .collect();
        trait1.dedup_by(|a, b| a.0 == b.0);
        trait1.sort_by(|a, b| a.0.cmp(&b.0));

        let trait1 = Trait {
            name: "Trait1".to_string(),
            records: trait1,
        };

        let json_trait1 = serde_json::to_string(&trait1).unwrap();

        println!("json_trait1: {}", json_trait1);

        let d_trait1 = serde_json::from_str(&json_trait1).unwrap();

        assert!(trait1 == d_trait1);
    }

    #[test]
    fn test_json_boxed_1() {
        let mut trait1 = vec![
            (
                "int1".to_string(),
                RecordInfo::SingleType(VariableTypeInfo {
                    type_: VariableType::Int,
                    default: None,
                }),
            ),
            (
                "boxed1".to_string(),
                RecordInfo::SingleType(VariableTypeInfo {
                    type_: VariableType::Code(Box::new(CodeInfo {
                        kind: CodeType::Wire {
                            looped: TriState::Either,
                        },
                        requires: vec![("int1".to_string(), VariableType::Int)],
                        exposes: vec![],
                        inputs: vec![],
                        output: VariableType::None,
                    })),
                    default: None,
                }),
            ),
        ];

        // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
        trait1 = trait1
            .into_iter()
            .map(|(name, info)| (name.to_lowercase(), info))
            .collect();
        trait1.dedup_by(|a, b| a.0 == b.0);
        trait1.sort_by(|a, b| a.0.cmp(&b.0));

        let trait1 = Trait {
            name: "Trait1".to_string(),
            records: trait1,
        };

        let json_trait1 = serde_json::to_string(&trait1).unwrap();

        let d_trait1 = serde_json::from_str(&json_trait1).unwrap();

        assert!(trait1 == d_trait1);
        assert!(d_trait1.records[0].0 == "boxed1".to_string());
        let requires = match d_trait1.records[0].1 {
            RecordInfo::SingleType(VariableTypeInfo {
                type_: VariableType::Code(ref code),
                default: None,
            }) => code.requires.clone(),
            _ => panic!("Expected a code"),
        };
        assert!(requires[0].0 == "int1".to_string());
    }

    #[test]
    fn test_json_textual_from_str() {
        let trait1 = Trait {
            name: "Trait1".to_string(),
            records: vec![(
                "int1".to_string(),
                RecordInfo::SingleType(VariableTypeInfo {
                    type_: VariableType::Int,
                    default: None,
                }),
            )],
        };

        let json_trait1 = r#"{"name":"Trait1","records":[["int1",{"SingleType":{"type":"Int","default":null}}]]}"#;

        let d_trait1 = serde_json::from_str(&json_trait1).unwrap();

        assert!(trait1 == d_trait1);
    }
}

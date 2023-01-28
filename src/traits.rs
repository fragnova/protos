use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::{boxed::Box, vec::Vec};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// For more info refer to:
// https://github.com/fragcolor-xyz/shards/blob/devel/include/shards.h

#[cfg(not(feature = "std"))]
type String = Vec<u8>;

/// 256 bytes u8-Array
pub type Hash256 = [u8; 32];

/// 128 bytes u8-Array
pub type Hash128 = [u8; 16];

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
    Wire { looped: Option<bool> },
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
    Proto(Hash256),
    Fragment(Hash128, u64, u64),
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

/// Struct represents a Trait
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct Trait {
    /// Name of the Trait
    pub name: String,
    /// List of attributes of the Trait. An attribute is represented as a **tuple that contains the attribute's name and the attribute's type**.
    pub records: Vec<(String, Vec<VariableTypeInfo>)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_simple_1() {
        let mut trait1 = vec![(
            "int1".to_string(),
            vec![VariableTypeInfo {
                type_: VariableType::Int,
                default: None,
            }],
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
                vec![VariableTypeInfo {
                    type_: VariableType::Int,
                    default: None,
                }],
            ),
            (
                "boxed1".to_string(),
                vec![VariableTypeInfo {
                    type_: VariableType::Code(Box::new(CodeInfo {
                        kind: CodeType::Wire {
                            looped: None,
                        },
                        requires: vec![("int1".to_string(), VariableType::Int)],
                        exposes: vec![],
                        inputs: vec![],
                        output: VariableType::None,
                    })),
                    default: None,
                }],
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
    fn test_json_simple_2() {
        let mut trait2 = vec![(
            "string1".to_string(),
            RecordInfo::SingleType(VariableTypeInfo {
                type_: VariableType::String,
                default: None,
            }),
        )];

        trait2 = trait2
            .into_iter()
            .map(|(name, info)| (name.to_lowercase(), info))
            .collect();
        trait2.dedup_by(|a, b| a.0 == b.0);
        trait2.sort_by(|a, b| a.0.cmp(&b.0));

        let trait2 = Trait {
            name: "Trait2".to_string(),
            records: trait2,
        };

        let json_trait2 = serde_json::to_string(&trait2).unwrap();

        println!("json_trait2: {}", json_trait2);

        let d_trait2 = serde_json::from_str(&json_trait2).unwrap();

        assert!(trait2 == d_trait2);
    }

    #[test]
    fn test_json_simple_3() {
        let mut trait3 = vec![(
            "bool1".to_string(),
            RecordInfo::SingleType(VariableTypeInfo {
                type_: VariableType::Bool,
                default: None,
            }),
        )];

        trait3 = trait3
            .into_iter()
            .map(|(name, info)| (name.to_lowercase(), info))
            .collect();
        trait3.dedup_by(|a, b| a.0 == b.0);
        trait3.sort_by(|a, b| a.0.cmp(&b.0));

        let trait3 = Trait {
            name: "Trait3".to_string(),
            records: trait3,
        };

        let json_trait3 = serde_json::to_string(&trait3).unwrap();

        println!("json_trait3: {}", json_trait3);

        let d_trait3 = serde_json::from_str(&json_trait3).unwrap();

        assert!(trait3 == d_trait3);
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
                            looped: None,
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

use crate::categories::{Categories, TextCategories, TextureCategories};
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
  /// List of variables that must be available to the Code's code context, before the Code even executes.
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
    #[codec(compact)]
    vendor_id: u32,
    #[codec(compact)]
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
  BoundedSeq {
    types: Vec<VariableType>,
    #[codec(compact)]
    max_len: u32,
  },
  FixedSeq {
    types: Vec<VariableType>,
    #[codec(compact)]
    fixed_len: u32,
  },
  Table(TableInfo),
  /// VendorID, TypeID
  Object {
    #[codec(compact)]
    vendor_id: u32,
    #[codec(compact)]
    type_id: u32,
  },
  Audio,
  Code(Box<CodeInfo>),
  Mesh,
  Channel(Box<VariableType>),
  Proto(Categories),
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

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct Record {
  pub name: String,
  pub types: Vec<VariableTypeInfo>,
}

impl From<(String, Vec<VariableTypeInfo>)> for Record {
  fn from((name, types): (String, Vec<VariableTypeInfo>)) -> Self {
    Self { name, types }
  }
}

/// Struct represents a Trait
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct Trait {
  /// Name of the Trait
  pub name: String,
  /// Revision of the Trait
  #[codec(compact)]
  pub revision: u32,
  /// List of attributes of the Trait. An attribute is represented as a **tuple that contains the attribute's name and the attribute's type**.
  pub records: Vec<Record>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encode_decode_simple_1() {
    let mut trait1: Vec<Record> = vec![(
      "int1".to_string(),
      vec![VariableTypeInfo {
        type_: VariableType::Int,
        default: None,
      }],
    )
      .into()];

    // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
    trait1 = trait1
      .into_iter()
      .map(|r| (r.name.to_lowercase(), r.types).into())
      .collect();
    trait1.dedup_by(|a, b| a.name == b.name);
    // Note: "Strings are ordered lexicographically by their byte values ... This is not necessarily the same as “alphabetical” order, which varies by language and locale". Source: https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str
    trait1.sort_by(|a, b| a.name.cmp(&b.name));

    let trait1 = Trait {
      name: "Trait1".to_string(),
      revision: 1,
      records: trait1,
    };

    let e_trait1 = trait1.encode();

    let d_trait1 = Trait::decode(&mut e_trait1.as_slice()).unwrap();

    assert!(trait1 == d_trait1);
  }

  #[test]
  fn encode_decode_boxed_1() {
    let mut trait1: Vec<Record> = vec![
      (
        "int1".to_string(),
        vec![VariableTypeInfo {
          type_: VariableType::Int,
          default: None,
        }],
      )
        .into(),
      (
        "boxed1".to_string(),
        vec![VariableTypeInfo {
          type_: VariableType::Code(Box::new(CodeInfo {
            kind: CodeType::Wire { looped: None },
            requires: vec![("int1".to_string(), VariableType::Int)],
            exposes: vec![],
            inputs: vec![],
            output: VariableType::None,
          })),
          default: None,
        }],
      )
        .into(),
    ];

    // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
    trait1 = trait1
      .into_iter()
      .map(|r| (r.name.to_lowercase(), r.types).into())
      .collect();
    trait1.dedup_by(|a, b| a.name == b.name);
    trait1.sort_by(|a, b| a.name.cmp(&b.name));

    let trait1 = Trait {
      name: "Trait1".to_string(),
      revision: 1,
      records: trait1,
    };

    let e_trait1 = trait1.encode();

    let d_trait1 = Trait::decode(&mut e_trait1.as_slice()).unwrap();

    assert!(trait1 == d_trait1);
    assert!(d_trait1.records[0].name == "boxed1".to_string());
    let type_ = &d_trait1.records[0].types[0].type_;
    let requires = match type_ {
      VariableType::Code(code) => &code.requires,
      _ => panic!("Should be a code"),
    };
    assert!(requires[0].0 == "int1".to_string());
  }

  #[test]
  fn test_json_simple_1() {
    let mut trait1: Vec<Record> = vec![(
      "int1".to_string(),
      vec![VariableTypeInfo {
        type_: VariableType::Int,
        default: None,
      }],
    )
      .into()];

    // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
    trait1 = trait1
      .into_iter()
      .map(|r| (r.name.to_lowercase(), r.types).into())
      .collect();
    trait1.dedup_by(|a, b| a.name == b.name);
    trait1.sort_by(|a, b| a.name.cmp(&b.name));

    let trait1 = Trait {
      name: "Trait1".to_string(),
      revision: 1,
      records: trait1,
    };

    let e_trait1 = serde_json::to_string(&trait1).unwrap();

    let d_trait1: Trait = serde_json::from_str(&e_trait1).unwrap();

    assert!(trait1 == d_trait1);
  }

  #[test]
  fn test_json_boxed_1() {
    let mut trait1: Vec<Record> = vec![
      (
        "int1".to_string(),
        vec![VariableTypeInfo {
          type_: VariableType::Int,
          default: None,
        }],
      )
        .into(),
      (
        "boxed1".to_string(),
        vec![VariableTypeInfo {
          type_: VariableType::Code(Box::new(CodeInfo {
            kind: CodeType::Wire { looped: None },
            requires: vec![("int1".to_string(), VariableType::Int)],
            exposes: vec![],
            inputs: vec![],
            output: VariableType::None,
          })),
          default: None,
        }],
      )
        .into(),
    ];

    // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
    trait1 = trait1
      .into_iter()
      .map(|r| (r.name.to_lowercase(), r.types).into())
      .collect();
    trait1.dedup_by(|a, b| a.name == b.name);
    trait1.sort_by(|a, b| a.name.cmp(&b.name));

    let trait1 = Trait {
      name: "Trait1".to_string(),
      revision: 1,
      records: trait1,
    };

    let e_trait1 = serde_json::to_string(&trait1).unwrap();

    let d_trait1: Trait = serde_json::from_str(&e_trait1).unwrap();

    assert!(trait1 == d_trait1);
    assert!(d_trait1.records[0].name == "boxed1".to_string());
    let type_ = &d_trait1.records[0].types[0].type_;
    let requires = match type_ {
      VariableType::Code(code) => &code.requires,
      _ => panic!("Should be a code"),
    };
    assert!(requires[0].0 == "int1".to_string());
  }

  #[test]
  fn test_json_textual_from_str() {
    let trait1 = Trait {
      name: "Trait1".to_string(),
      revision: 1,
      records: vec![(
        "int1".to_string(),
        vec![VariableTypeInfo {
          type_: VariableType::Int,
          default: None,
        }],
      )
        .into()],
    };

    let json_trait1 = r#"{
      "name": "Trait1",
      "revision": 1,
      "records": [
        {
          "name": "int1",
          "types": [
            {
              "type": "Int",
              "default": null
            }
          ]
        }
      ]
    }"#;

    let d_trait1 = serde_json::from_str(&json_trait1).unwrap();

    assert!(trait1 == d_trait1);
  }

  #[test]
  fn test_json_textual_from_str_ambal() {
    let json_trait1 = r#"{
      "name": "AmbalLoreFragment",
      "revision": 1,
      "records": [
        {
          "name": "banner",
          "types": [
            {"type": {"Proto": {"texture": "pngFile"}}},
            {"type": {"Proto": {"texture": "jpgFile"}}},
            {"type": "Image"}
          ]
        },
        {
          "name": "content",
          "types": [
            {"type": {"Proto": {"text": "plain"}}},
            {"type": "String"}
          ]
        }
      ]
    }"#;

    let trait1 = Trait {
      name: "AmbalLoreFragment".to_string(),
      revision: 1,
      records: vec![
        (
          "banner".to_string(),
          vec![
            VariableTypeInfo {
              type_: VariableType::Proto(Categories::Texture(TextureCategories::PngFile)),
              default: None,
            }
            .into(),
            VariableTypeInfo {
              type_: VariableType::Proto(Categories::Texture(TextureCategories::JpgFile)),
              default: None,
            }
            .into(),
            VariableTypeInfo {
              type_: VariableType::Image,
              default: None,
            }
            .into(),
          ],
        )
          .into(),
        (
          "content".to_string(),
          vec![
            VariableTypeInfo {
              type_: VariableType::Proto(Categories::Text(TextCategories::Plain)),
              default: None,
            }
            .into(),
            VariableTypeInfo {
              type_: VariableType::String,
              default: None,
            }
            .into(),
          ],
        )
          .into(),
      ],
    };

    let d_trait1 = serde_json::from_str(&json_trait1).unwrap();

    assert!(trait1 == d_trait1);
  }
}

use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;

// For more info refer to:
// https://github.com/fragcolor-xyz/shards/blob/devel/include/shards.h

#[cfg(not(feature = "std"))]
type String = Vec<u8>;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum TriState {
    Either,
    True,
    False,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum CodeType {
    /// A list of shards, to be injected into more complex blocks of code or wires
    Shards,
    /// An actual wire
    Wire { looped: TriState },
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct CodeInfo {
    pub kind: CodeType,
    pub requires: Vec<(String, VariableType)>,
    pub exposes: Vec<(String, VariableType)>,
    pub inputs: Vec<VariableType>,
    pub output: VariableType,
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub struct TableInfo {
    /// Tha name of the keys, an empty key represent any name, allowing multiple instances of the corresponding index type
    pub keys: Vec<String>,
    /// Following keys array (should be same len), the types expected
    pub types: Vec<Vec<VariableType>>,
}

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
    Int,    // A 64bits int
    Int2,   // A vector of 2 64bits ints
    Int3,   // A vector of 3 32bits ints
    Int4,   // A vector of 4 32bits ints
    Int8,   // A vector of 8 16bits ints
    Int16,  // A vector of 16 8bits ints
    Float,  // A 64bits float
    Float2, // A vector of 2 64bits floats
    Float3, // A vector of 3 32bits floats
    Float4, // A vector of 4 32bits floats
    Color,  // A vector of 4 uint8
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
    Mesh {
        name: String,
    },
    Channel(Box<VariableType>),
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
pub enum TraitInfo {
    SingleType(VariableType),
    MultipleTypes(Vec<VariableType>),
}

pub type Traits = Vec<(String, TraitInfo)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_simple_1() {
        let mut trait1 = vec![("int1".to_string(), TraitInfo::SingleType(VariableType::Int))];

        // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
        trait1.dedup_by(|a, b| a.0.to_lowercase() == b.0.to_lowercase());
        trait1.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        let e_trait1 = trait1.encode();

        let d_trait1 = Traits::decode(&mut e_trait1.as_slice()).unwrap();

        assert!(trait1 == d_trait1);
    }

    #[test]
    fn encode_decode_boxed_1() {
        let mut trait1 = vec![
            ("int1".to_string(), TraitInfo::SingleType(VariableType::Int)),
            (
                "boxed1".to_string(),
                TraitInfo::SingleType(VariableType::Code(Box::new(CodeInfo {
                    kind: CodeType::Wire {
                        looped: TriState::Either,
                    },
                    requires: vec![("int1".to_string(), VariableType::Int)],
                    exposes: vec![],
                    inputs: vec![],
                    output: VariableType::None,
                }))),
            ),
        ];

        // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
        trait1.dedup_by(|a, b| a.0.to_lowercase() == b.0.to_lowercase());
        trait1.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        let e_trait1 = trait1.encode();

        let d_trait1 = Traits::decode(&mut e_trait1.as_slice()).unwrap();

        assert!(trait1 == d_trait1);
        assert!(d_trait1[0].0 == "boxed1".to_string());
        let requires = match d_trait1[0].1 {
            TraitInfo::SingleType(VariableType::Code(ref code)) => code.requires.clone(),
            _ => panic!("Expected a code"),
        };
        assert!(requires[0].0 == "int1".to_string());
    }
}

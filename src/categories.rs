use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;

#[cfg(not(feature = "std"))]
type String = Vec<u8>;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// A XX64 hash of the trait interface.
pub type ShardsTrait = [u8; 8];

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum ShardsFormat {
  /// Canonical textual format interpreted by the Shards runtime
  Edn,
  /// Serialized binary format
  Binary,
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub struct ShardsScriptInfo {
  pub format: ShardsFormat,
  #[codec(compact)]
  pub shards_version: u32,
  pub requiring: Vec<ShardsTrait>,
  pub implementing: Vec<ShardsTrait>,
}

// serde(rename_all = "camelCase") is needed or polkadot.js will not be able to deserialize

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum AudioCategories {
  /// A compressed audio file in the ogg container format
  OggFile,
  /// A compressed audio file in the mp3 format
  Mp3File,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum ModelCategories {
  /// A GLTF binary model
  GltfFile,
  /// 🤫😄
  Sdf,
  /// A physics collision model
  PhysicsCollider,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum TextureCategories {
  PngFile,
  JpgFile,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum VectorCategories {
  /// A Scalable Vector Graphics file
  SvgFile,
  /// A TrueType font file
  TtfFile,
  /// A font file in the OpenType format
  OtfFile,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum VideoCategories {
  /// A compressed video file in the mkv container format
  MkvFile,
  /// A compressed video file in the mp4 container format
  Mp4File,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum TextCategories {
  /// Plain Text
  Plain,
  /// Json String
  Json,
  /// WebGPU shader code
  Wgsl,
  /// A markdown file
  Markdown,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum BinaryCategories {
  /// A generic wasm program, compiled to run on a WASI runtime
  WasmProgram,
  /// A generic wasm reactor, compiled to run on a WASI runtime
  WasmReactor,
  /// A blender file. Royalties distribution of blender files derived protos will always allocate a % to the Blender Foundation
  BlendFile,
  /// An ONNX ML model in its binary format
  OnnxModel,
  /// A safetensors ML model as from https://github.com/huggingface/safetensors
  SafeTensors,
  /// A RareForm Engine Domain
  RareDomain,
}

/// Types of categories that can be attached to a Proto-Fragment to describe it (e.g Code, Audio, Video etc.)
#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, scale_info::TypeInfo)]
#[cfg_attr(
  feature = "std",
  derive(Serialize, Deserialize),
  serde(rename_all = "camelCase")
)]
pub enum Categories {
  /// Text of the supported sub-categories
  Text(TextCategories),
  /// A Scripting Trait declaration, traits are unique, and are used to describe how Shards work (Scripts)
  /// None is meant to be used in Fragnova protos when uploading and RPCs mainly.
  Trait(Option<ShardsTrait>),
  /// Shards scripts of various sub-categories
  /// Shards use interoperability traits to describe how they can be used in other shards
  Shards(ShardsScriptInfo),
  /// Audio files and effects
  Audio(AudioCategories),
  /// Textures of the supported sub-categories
  Texture(TextureCategories),
  /// Vectors of the supported sub-categories (e.g. SVG, Font)
  Vector(VectorCategories),
  /// Video file of the supported formats
  Video(VideoCategories),
  /// 2d/3d models of the supported formats
  Model(ModelCategories),
  /// Binary of the supported sub-categories
  Binary(BinaryCategories),
  /// A bundle of many protos
  Bundle,
}

use bitflags::bitflags;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};

bitflags! {
  /// Permissions for fragments and fragment's bundles.
  #[derive(Encode, Decode, MaxEncodedLen, scale_info::TypeInfo)]
  pub struct FragmentPerms: u32 {
    const NONE = 0;
    const EDIT = 1;
    const COPY = 2;
    const TRANSFER = 4;
    const ALL = Self::EDIT.bits | Self::COPY.bits | Self::TRANSFER.bits;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestStruct {
    pub _name: String,
    pub permissions: FragmentPerms,
  }

  #[test]
  fn t1() {
    let test_struct = TestStruct {
      _name: "test".to_string(),
      permissions: FragmentPerms::NONE,
    };

    assert_eq!(test_struct.permissions.bits, 0);
  }
}

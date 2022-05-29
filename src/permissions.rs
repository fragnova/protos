use bitflags::bitflags;

bitflags! {
  /// Permissions for fragments and fragment's bundles.
  struct FragmentPerms: u32 {
    const NONE = 0;
    const EDIT = 1;
    const COPY = 2;
    const TRANSFER = 4;
    const ALL = Self::EDIT.bits | Self::COPY.bits | Self::TRANSFER.bits;
  }
}

// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ARCHIVE_ENTRY_TYPE: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ARCHIVE_ENTRY_TYPE: i8 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ARCHIVE_ENTRY_TYPE: [ArchiveEntryType; 3] = [
  ArchiveEntryType::File,
  ArchiveEntryType::EmptyFolder,
  ArchiveEntryType::EmptyFile,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ArchiveEntryType(pub i8);
#[allow(non_upper_case_globals)]
impl ArchiveEntryType {
  pub const File: Self = Self(0);
  pub const EmptyFolder: Self = Self(1);
  pub const EmptyFile: Self = Self(2);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::File,
    Self::EmptyFolder,
    Self::EmptyFile,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::File => Some("File"),
      Self::EmptyFolder => Some("EmptyFolder"),
      Self::EmptyFile => Some("EmptyFile"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ArchiveEntryType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ArchiveEntryType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ArchiveEntryType {
    type Output = ArchiveEntryType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ArchiveEntryType {
  type Scalar = i8;
  #[inline]
  fn to_little_endian(self) -> i8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i8) -> Self {
    let b = i8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ArchiveEntryType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ArchiveEntryType {}
pub enum AppendixEntryOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AppendixEntry<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AppendixEntry<'a> {
  type Inner = AppendixEntry<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AppendixEntry<'a> {
  pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
  pub const VT_OFFSET: flatbuffers::VOffsetT = 6;
  pub const VT_SIZE_: flatbuffers::VOffsetT = 8;
  pub const VT_NAME: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AppendixEntry { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args AppendixEntryArgs<'args>
  ) -> flatbuffers::WIPOffset<AppendixEntry<'bldr>> {
    let mut builder = AppendixEntryBuilder::new(_fbb);
    builder.add_size_(args.size_);
    builder.add_offset(args.offset);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_type_(args.type_);
    builder.finish()
  }


  #[inline]
  pub fn type_(&self) -> ArchiveEntryType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ArchiveEntryType>(AppendixEntry::VT_TYPE_, Some(ArchiveEntryType::File)).unwrap()}
  }
  #[inline]
  pub fn offset(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(AppendixEntry::VT_OFFSET, Some(0)).unwrap()}
  }
  #[inline]
  pub fn size_(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(AppendixEntry::VT_SIZE_, Some(0)).unwrap()}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(AppendixEntry::VT_NAME, None)}
  }
}

impl flatbuffers::Verifiable for AppendixEntry<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<ArchiveEntryType>("type_", Self::VT_TYPE_, false)?
     .visit_field::<u64>("offset", Self::VT_OFFSET, false)?
     .visit_field::<u64>("size_", Self::VT_SIZE_, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .finish();
    Ok(())
  }
}
pub struct AppendixEntryArgs<'a> {
    pub type_: ArchiveEntryType,
    pub offset: u64,
    pub size_: u64,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for AppendixEntryArgs<'a> {
  #[inline]
  fn default() -> Self {
    AppendixEntryArgs {
      type_: ArchiveEntryType::File,
      offset: 0,
      size_: 0,
      name: None,
    }
  }
}

pub struct AppendixEntryBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> AppendixEntryBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_type_(&mut self, type_: ArchiveEntryType) {
    self.fbb_.push_slot::<ArchiveEntryType>(AppendixEntry::VT_TYPE_, type_, ArchiveEntryType::File);
  }
  #[inline]
  pub fn add_offset(&mut self, offset: u64) {
    self.fbb_.push_slot::<u64>(AppendixEntry::VT_OFFSET, offset, 0);
  }
  #[inline]
  pub fn add_size_(&mut self, size_: u64) {
    self.fbb_.push_slot::<u64>(AppendixEntry::VT_SIZE_, size_, 0);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AppendixEntry::VT_NAME, name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> AppendixEntryBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    AppendixEntryBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AppendixEntry<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AppendixEntry<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AppendixEntry");
      ds.field("type_", &self.type_());
      ds.field("offset", &self.offset());
      ds.field("size_", &self.size_());
      ds.field("name", &self.name());
      ds.finish()
  }
}
pub enum AppendixOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Appendix<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Appendix<'a> {
  type Inner = Appendix<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Appendix<'a> {
  pub const VT_ENTRIES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Appendix { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args AppendixArgs<'args>
  ) -> flatbuffers::WIPOffset<Appendix<'bldr>> {
    let mut builder = AppendixBuilder::new(_fbb);
    if let Some(x) = args.entries { builder.add_entries(x); }
    builder.finish()
  }


  #[inline]
  pub fn entries(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AppendixEntry<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AppendixEntry>>>>(Appendix::VT_ENTRIES, None)}
  }
}

impl flatbuffers::Verifiable for Appendix<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<AppendixEntry>>>>("entries", Self::VT_ENTRIES, false)?
     .finish();
    Ok(())
  }
}
pub struct AppendixArgs<'a> {
    pub entries: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AppendixEntry<'a>>>>>,
}
impl<'a> Default for AppendixArgs<'a> {
  #[inline]
  fn default() -> Self {
    AppendixArgs {
      entries: None,
    }
  }
}

pub struct AppendixBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> AppendixBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_entries(&mut self, entries: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<AppendixEntry<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Appendix::VT_ENTRIES, entries);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> AppendixBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    AppendixBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Appendix<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Appendix<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Appendix");
      ds.field("entries", &self.entries());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Appendix`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_appendix_unchecked`.
pub fn root_as_appendix(buf: &[u8]) -> Result<Appendix, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Appendix>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Appendix` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_appendix_unchecked`.
pub fn size_prefixed_root_as_appendix(buf: &[u8]) -> Result<Appendix, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Appendix>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Appendix` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_appendix_unchecked`.
pub fn root_as_appendix_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Appendix<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Appendix<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Appendix` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_appendix_unchecked`.
pub fn size_prefixed_root_as_appendix_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Appendix<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Appendix<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Appendix and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Appendix`.
pub unsafe fn root_as_appendix_unchecked(buf: &[u8]) -> Appendix {
  flatbuffers::root_unchecked::<Appendix>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Appendix and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Appendix`.
pub unsafe fn size_prefixed_root_as_appendix_unchecked(buf: &[u8]) -> Appendix {
  flatbuffers::size_prefixed_root_unchecked::<Appendix>(buf)
}
#[inline]
pub fn finish_appendix_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Appendix<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_appendix_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Appendix<'a>>) {
  fbb.finish_size_prefixed(root, None);
}

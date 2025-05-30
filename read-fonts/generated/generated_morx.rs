// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [morx (Extended Glyph Metamorphosis)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html) table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct MorxMarker {
    chains_byte_len: usize,
}

impl MorxMarker {
    pub fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn unused_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn n_chains_byte_range(&self) -> Range<usize> {
        let start = self.unused_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn chains_byte_range(&self) -> Range<usize> {
        let start = self.n_chains_byte_range().end;
        start..start + self.chains_byte_len
    }
}

impl MinByteRange for MorxMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.chains_byte_range().end
    }
}

impl TopLevelTable for Morx<'_> {
    /// `morx`
    const TAG: Tag = Tag::new(b"morx");
}

impl<'a> FontRead<'a> for Morx<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        let n_chains: u32 = cursor.read()?;
        let chains_byte_len = {
            let data = cursor.remaining().ok_or(ReadError::OutOfBounds)?;
            <Chain as VarSize>::total_len_for_count(data, n_chains as usize)?
        };
        cursor.advance_by(chains_byte_len);
        cursor.finish(MorxMarker { chains_byte_len })
    }
}

/// The [morx (Extended Glyph Metamorphosis)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html) table.
pub type Morx<'a> = TableRef<'a, MorxMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Morx<'a> {
    /// Version number of the extended glyph metamorphosis table (either 2 or 3).
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of metamorphosis chains contained in this table.
    pub fn n_chains(&self) -> u32 {
        let range = self.shape.n_chains_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    pub fn chains(&self) -> VarLenArray<'a, Chain<'a>> {
        let range = self.shape.chains_byte_range();
        VarLenArray::read(self.data.split_off(range.start).unwrap()).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Morx<'a> {
    fn type_name(&self) -> &str {
        "Morx"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("n_chains", self.n_chains())),
            2usize => Some(Field::new(
                "chains",
                traversal::FieldType::var_array("Chain", self.chains(), self.offset_data()),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Morx<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// A chain in a `morx` table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct ChainMarker {
    features_byte_len: usize,
    subtables_byte_len: usize,
}

impl ChainMarker {
    pub fn default_flags_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn chain_length_byte_range(&self) -> Range<usize> {
        let start = self.default_flags_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn n_feature_entries_byte_range(&self) -> Range<usize> {
        let start = self.chain_length_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn n_subtables_byte_range(&self) -> Range<usize> {
        let start = self.n_feature_entries_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn features_byte_range(&self) -> Range<usize> {
        let start = self.n_subtables_byte_range().end;
        start..start + self.features_byte_len
    }

    pub fn subtables_byte_range(&self) -> Range<usize> {
        let start = self.features_byte_range().end;
        start..start + self.subtables_byte_len
    }
}

impl MinByteRange for ChainMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.subtables_byte_range().end
    }
}

impl<'a> FontRead<'a> for Chain<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        let n_feature_entries: u32 = cursor.read()?;
        let n_subtables: u32 = cursor.read()?;
        let features_byte_len = (n_feature_entries as usize)
            .checked_mul(Feature::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(features_byte_len);
        let subtables_byte_len = {
            let data = cursor.remaining().ok_or(ReadError::OutOfBounds)?;
            <Subtable as VarSize>::total_len_for_count(data, n_subtables as usize)?
        };
        cursor.advance_by(subtables_byte_len);
        cursor.finish(ChainMarker {
            features_byte_len,
            subtables_byte_len,
        })
    }
}

/// A chain in a `morx` table.
pub type Chain<'a> = TableRef<'a, ChainMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Chain<'a> {
    /// The default specification for subtables.
    pub fn default_flags(&self) -> u32 {
        let range = self.shape.default_flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Total byte count, including this header; must be a multiple of 4.
    pub fn chain_length(&self) -> u32 {
        let range = self.shape.chain_length_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of feature subtable entries.
    pub fn n_feature_entries(&self) -> u32 {
        let range = self.shape.n_feature_entries_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of subtables in the chain.
    pub fn n_subtables(&self) -> u32 {
        let range = self.shape.n_subtables_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Feature entries for this chain.
    pub fn features(&self) -> &'a [Feature] {
        let range = self.shape.features_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Array of chain subtables.
    pub fn subtables(&self) -> VarLenArray<'a, Subtable<'a>> {
        let range = self.shape.subtables_byte_range();
        VarLenArray::read(self.data.split_off(range.start).unwrap()).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Chain<'a> {
    fn type_name(&self) -> &str {
        "Chain"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("default_flags", self.default_flags())),
            1usize => Some(Field::new("chain_length", self.chain_length())),
            2usize => Some(Field::new("n_feature_entries", self.n_feature_entries())),
            3usize => Some(Field::new("n_subtables", self.n_subtables())),
            4usize => Some(Field::new(
                "features",
                traversal::FieldType::array_of_records(
                    stringify!(Feature),
                    self.features(),
                    self.offset_data(),
                ),
            )),
            5usize => Some(Field::new(
                "subtables",
                traversal::FieldType::var_array("Subtable", self.subtables(), self.offset_data()),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Chain<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Used to compute the sub-feature flags for a list of requested features and settings.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct Feature {
    /// The type of feature.
    pub feature_type: BigEndian<u16>,
    /// The feature's setting (aka selector).
    pub feature_settings: BigEndian<u16>,
    /// Flags for the settings that this feature and setting enables.
    pub enable_flags: BigEndian<u32>,
    /// Complement of flags for the settings that this feature and setting disable.
    pub disable_flags: BigEndian<u32>,
}

impl Feature {
    /// The type of feature.
    pub fn feature_type(&self) -> u16 {
        self.feature_type.get()
    }

    /// The feature's setting (aka selector).
    pub fn feature_settings(&self) -> u16 {
        self.feature_settings.get()
    }

    /// Flags for the settings that this feature and setting enables.
    pub fn enable_flags(&self) -> u32 {
        self.enable_flags.get()
    }

    /// Complement of flags for the settings that this feature and setting disable.
    pub fn disable_flags(&self) -> u32 {
        self.disable_flags.get()
    }
}

impl FixedSize for Feature {
    const RAW_BYTE_LEN: usize =
        u16::RAW_BYTE_LEN + u16::RAW_BYTE_LEN + u32::RAW_BYTE_LEN + u32::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for Feature {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "Feature",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("feature_type", self.feature_type())),
                1usize => Some(Field::new("feature_settings", self.feature_settings())),
                2usize => Some(Field::new("enable_flags", self.enable_flags())),
                3usize => Some(Field::new("disable_flags", self.disable_flags())),
                _ => None,
            }),
            data,
        }
    }
}

/// A subtable in a `morx` chain.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct SubtableMarker {
    data_byte_len: usize,
}

impl SubtableMarker {
    pub fn length_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn coverage_byte_range(&self) -> Range<usize> {
        let start = self.length_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn sub_feature_flags_byte_range(&self) -> Range<usize> {
        let start = self.coverage_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn data_byte_range(&self) -> Range<usize> {
        let start = self.sub_feature_flags_byte_range().end;
        start..start + self.data_byte_len
    }
}

impl MinByteRange for SubtableMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.data_byte_range().end
    }
}

impl<'a> FontRead<'a> for Subtable<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        let data_byte_len = cursor.remaining_bytes() / u8::RAW_BYTE_LEN * u8::RAW_BYTE_LEN;
        cursor.advance_by(data_byte_len);
        cursor.finish(SubtableMarker { data_byte_len })
    }
}

/// A subtable in a `morx` chain.
pub type Subtable<'a> = TableRef<'a, SubtableMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Subtable<'a> {
    /// Total subtable length, including this header.
    pub fn length(&self) -> u32 {
        let range = self.shape.length_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Coverage flags and subtable type.
    pub fn coverage(&self) -> u32 {
        let range = self.shape.coverage_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The 32-bit mask identifying which subtable this is (the subtable being executed if the AND of this value and the processed defaultFlags is nonzero).
    pub fn sub_feature_flags(&self) -> u32 {
        let range = self.shape.sub_feature_flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Data for specific subtable.
    pub fn data(&self) -> &'a [u8] {
        let range = self.shape.data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Subtable<'a> {
    fn type_name(&self) -> &str {
        "Subtable"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("length", self.length())),
            1usize => Some(Field::new("coverage", self.coverage())),
            2usize => Some(Field::new("sub_feature_flags", self.sub_feature_flags())),
            3usize => Some(Field::new("data", self.data())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Subtable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Entry payload in a contextual subtable state machine.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct ContextualEntryData {
    /// Index of the substitution table for the marked glyph (use 0xFFFF for
    /// none).
    pub mark_index: BigEndian<u16>,
    /// Index of the substitution table for the current glyph (use 0xFFFF for
    /// none)
    pub current_index: BigEndian<u16>,
}

impl ContextualEntryData {
    /// Index of the substitution table for the marked glyph (use 0xFFFF for
    /// none).
    pub fn mark_index(&self) -> u16 {
        self.mark_index.get()
    }

    /// Index of the substitution table for the current glyph (use 0xFFFF for
    /// none)
    pub fn current_index(&self) -> u16 {
        self.current_index.get()
    }
}

impl FixedSize for ContextualEntryData {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + u16::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for ContextualEntryData {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "ContextualEntryData",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("mark_index", self.mark_index())),
                1usize => Some(Field::new("current_index", self.current_index())),
                _ => None,
            }),
            data,
        }
    }
}

/// Entry payload in an insertion subtable state machine.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct InsertionEntryData {
    /// Zero-based index into the insertion glyph table. The number of glyphs
    /// to be inserted is contained in the currentInsertCount field in the
    /// flags (see below). A value of 0xFFFF indicates no insertion is to be done.
    pub current_insert_index: BigEndian<u16>,
    /// Zero-based index into the insertion glyph table. The number of glyphs
    /// to be inserted is contained in the markedInsertCount field in the
    /// flags (see below). A value of 0xFFFF indicates no insertion is to be
    /// done.
    pub marked_insert_index: BigEndian<u16>,
}

impl InsertionEntryData {
    /// Zero-based index into the insertion glyph table. The number of glyphs
    /// to be inserted is contained in the currentInsertCount field in the
    /// flags (see below). A value of 0xFFFF indicates no insertion is to be done.
    pub fn current_insert_index(&self) -> u16 {
        self.current_insert_index.get()
    }

    /// Zero-based index into the insertion glyph table. The number of glyphs
    /// to be inserted is contained in the markedInsertCount field in the
    /// flags (see below). A value of 0xFFFF indicates no insertion is to be
    /// done.
    pub fn marked_insert_index(&self) -> u16 {
        self.marked_insert_index.get()
    }
}

impl FixedSize for InsertionEntryData {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + u16::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for InsertionEntryData {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "InsertionEntryData",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new(
                    "current_insert_index",
                    self.current_insert_index(),
                )),
                1usize => Some(Field::new(
                    "marked_insert_index",
                    self.marked_insert_index(),
                )),
                _ => None,
            }),
            data,
        }
    }
}

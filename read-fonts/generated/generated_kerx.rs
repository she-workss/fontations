// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [kerx (Extended Kerning)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html) table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct KerxMarker {
    subtables_byte_len: usize,
}

impl KerxMarker {
    pub fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn padding_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn n_tables_byte_range(&self) -> Range<usize> {
        let start = self.padding_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn subtables_byte_range(&self) -> Range<usize> {
        let start = self.n_tables_byte_range().end;
        start..start + self.subtables_byte_len
    }
}

impl MinByteRange for KerxMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.subtables_byte_range().end
    }
}

impl TopLevelTable for Kerx<'_> {
    /// `kerx`
    const TAG: Tag = Tag::new(b"kerx");
}

impl<'a> FontRead<'a> for Kerx<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        let n_tables: u32 = cursor.read()?;
        let subtables_byte_len = {
            let data = cursor.remaining().ok_or(ReadError::OutOfBounds)?;
            <Subtable as VarSize>::total_len_for_count(data, n_tables as usize)?
        };
        cursor.advance_by(subtables_byte_len);
        cursor.finish(KerxMarker { subtables_byte_len })
    }
}

/// The [kerx (Extended Kerning)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html) table.
pub type Kerx<'a> = TableRef<'a, KerxMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Kerx<'a> {
    /// The version number of the extended kerning table (currently 2, 3, or 4)
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of subtables included in the extended kerning table.
    pub fn n_tables(&self) -> u32 {
        let range = self.shape.n_tables_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    pub fn subtables(&self) -> VarLenArray<'a, Subtable<'a>> {
        let range = self.shape.subtables_byte_range();
        VarLenArray::read(self.data.split_off(range.start).unwrap()).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Kerx<'a> {
    fn type_name(&self) -> &str {
        "Kerx"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("n_tables", self.n_tables())),
            2usize => Some(Field::new(
                "subtables",
                traversal::FieldType::var_array("Subtable", self.subtables(), self.offset_data()),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Kerx<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// A subtable in a `kerx` table.
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

    pub fn tuple_count_byte_range(&self) -> Range<usize> {
        let start = self.coverage_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn data_byte_range(&self) -> Range<usize> {
        let start = self.tuple_count_byte_range().end;
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

/// A subtable in a `kerx` table.
pub type Subtable<'a> = TableRef<'a, SubtableMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Subtable<'a> {
    /// The length of this subtable in bytes, including this header.
    pub fn length(&self) -> u32 {
        let range = self.shape.length_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Circumstances under which this table is used.
    pub fn coverage(&self) -> u32 {
        let range = self.shape.coverage_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The tuple count. This value is only used with variation fonts and should be 0 for all other fonts. The subtable's tupleCount will be ignored if the 'kerx' table version is less than 4.
    pub fn tuple_count(&self) -> u32 {
        let range = self.shape.tuple_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Subtable specific data.
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
            2usize => Some(Field::new("tuple_count", self.tuple_count())),
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

/// The type 0 `kerx` subtable.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct Subtable0Marker {
    pairs_byte_len: usize,
}

impl Subtable0Marker {
    pub fn n_pairs_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn search_range_byte_range(&self) -> Range<usize> {
        let start = self.n_pairs_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn entry_selector_byte_range(&self) -> Range<usize> {
        let start = self.search_range_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn range_shift_byte_range(&self) -> Range<usize> {
        let start = self.entry_selector_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn pairs_byte_range(&self) -> Range<usize> {
        let start = self.range_shift_byte_range().end;
        start..start + self.pairs_byte_len
    }
}

impl MinByteRange for Subtable0Marker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.pairs_byte_range().end
    }
}

impl<'a> FontRead<'a> for Subtable0<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let n_pairs: u32 = cursor.read()?;
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        let pairs_byte_len = (n_pairs as usize)
            .checked_mul(Subtable0Pair::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(pairs_byte_len);
        cursor.finish(Subtable0Marker { pairs_byte_len })
    }
}

/// The type 0 `kerx` subtable.
pub type Subtable0<'a> = TableRef<'a, Subtable0Marker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Subtable0<'a> {
    /// The number of kerning pairs in this subtable.
    pub fn n_pairs(&self) -> u32 {
        let range = self.shape.n_pairs_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The largest power of two less than or equal to the value of nPairs, multiplied by the size in bytes of an entry in the subtable.
    pub fn search_range(&self) -> u32 {
        let range = self.shape.search_range_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// This is calculated as log2 of the largest power of two less than or equal to the value of nPairs. This value indicates how many iterations of the search loop have to be made. For example, in a list of eight items, there would be three iterations of the loop.
    pub fn entry_selector(&self) -> u32 {
        let range = self.shape.entry_selector_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The value of nPairs minus the largest power of two less than or equal to nPairs. This is multiplied by the size in bytes of an entry in the table.
    pub fn range_shift(&self) -> u32 {
        let range = self.shape.range_shift_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Kerning records.
    pub fn pairs(&self) -> &'a [Subtable0Pair] {
        let range = self.shape.pairs_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Subtable0<'a> {
    fn type_name(&self) -> &str {
        "Subtable0"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("n_pairs", self.n_pairs())),
            1usize => Some(Field::new("search_range", self.search_range())),
            2usize => Some(Field::new("entry_selector", self.entry_selector())),
            3usize => Some(Field::new("range_shift", self.range_shift())),
            4usize => Some(Field::new(
                "pairs",
                traversal::FieldType::array_of_records(
                    stringify!(Subtable0Pair),
                    self.pairs(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Subtable0<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The type 0 `kerx` subtable kerning record.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct Subtable0Pair {
    /// The glyph index for the lefthand glyph in the kerning pair.
    pub left: BigEndian<GlyphId16>,
    /// The glyph index for the righthand glyph in the kerning pair.
    pub right: BigEndian<GlyphId16>,
    /// Kerning value.
    pub value: BigEndian<i16>,
}

impl Subtable0Pair {
    /// The glyph index for the lefthand glyph in the kerning pair.
    pub fn left(&self) -> GlyphId16 {
        self.left.get()
    }

    /// The glyph index for the righthand glyph in the kerning pair.
    pub fn right(&self) -> GlyphId16 {
        self.right.get()
    }

    /// Kerning value.
    pub fn value(&self) -> i16 {
        self.value.get()
    }
}

impl FixedSize for Subtable0Pair {
    const RAW_BYTE_LEN: usize =
        GlyphId16::RAW_BYTE_LEN + GlyphId16::RAW_BYTE_LEN + i16::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for Subtable0Pair {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "Subtable0Pair",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("left", self.left())),
                1usize => Some(Field::new("right", self.right())),
                2usize => Some(Field::new("value", self.value())),
                _ => None,
            }),
            data,
        }
    }
}

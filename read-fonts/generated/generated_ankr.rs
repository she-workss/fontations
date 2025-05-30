// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [anchor point](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ankr.html) table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct AnkrMarker {}

impl AnkrMarker {
    pub fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn flags_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn lookup_table_offset_byte_range(&self) -> Range<usize> {
        let start = self.flags_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }

    pub fn glyph_data_table_offset_byte_range(&self) -> Range<usize> {
        let start = self.lookup_table_offset_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }
}

impl MinByteRange for AnkrMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.glyph_data_table_offset_byte_range().end
    }
}

impl TopLevelTable for Ankr<'_> {
    /// `ankr`
    const TAG: Tag = Tag::new(b"ankr");
}

impl<'a> FontRead<'a> for Ankr<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<Offset32>();
        cursor.advance::<u32>();
        cursor.finish(AnkrMarker {})
    }
}

/// The [anchor point](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ankr.html) table.
pub type Ankr<'a> = TableRef<'a, AnkrMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Ankr<'a> {
    /// Version number (set to zero).
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Flags (currently unused; set to zero).
    pub fn flags(&self) -> u16 {
        let range = self.shape.flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset to the table's lookup table; currently this is always `0x0000000C`.
    ///
    /// Lookup values are two byte offsets into the glyph data table.
    pub fn lookup_table_offset(&self) -> Offset32 {
        let range = self.shape.lookup_table_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`lookup_table_offset`][Self::lookup_table_offset].
    pub fn lookup_table(&self) -> Result<LookupU16<'a>, ReadError> {
        let data = self.data;
        self.lookup_table_offset().resolve(data)
    }

    /// Offset to the glyph data table.
    pub fn glyph_data_table_offset(&self) -> u32 {
        let range = self.shape.glyph_data_table_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Ankr<'a> {
    fn type_name(&self) -> &str {
        "Ankr"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("flags", self.flags())),
            2usize => Some(Field::new(
                "lookup_table_offset",
                FieldType::offset(self.lookup_table_offset(), self.lookup_table()),
            )),
            3usize => Some(Field::new(
                "glyph_data_table_offset",
                self.glyph_data_table_offset(),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Ankr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct GlyphDataEntryMarker {
    anchor_points_byte_len: usize,
}

impl GlyphDataEntryMarker {
    pub fn num_points_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn anchor_points_byte_range(&self) -> Range<usize> {
        let start = self.num_points_byte_range().end;
        start..start + self.anchor_points_byte_len
    }
}

impl MinByteRange for GlyphDataEntryMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.anchor_points_byte_range().end
    }
}

impl<'a> FontRead<'a> for GlyphDataEntry<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let num_points: u32 = cursor.read()?;
        let anchor_points_byte_len = (num_points as usize)
            .checked_mul(AnchorPoint::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(anchor_points_byte_len);
        cursor.finish(GlyphDataEntryMarker {
            anchor_points_byte_len,
        })
    }
}

pub type GlyphDataEntry<'a> = TableRef<'a, GlyphDataEntryMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> GlyphDataEntry<'a> {
    /// Number of anchor points for this glyph.
    pub fn num_points(&self) -> u32 {
        let range = self.shape.num_points_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Individual anchor points.
    pub fn anchor_points(&self) -> &'a [AnchorPoint] {
        let range = self.shape.anchor_points_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for GlyphDataEntry<'a> {
    fn type_name(&self) -> &str {
        "GlyphDataEntry"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("num_points", self.num_points())),
            1usize => Some(Field::new(
                "anchor_points",
                traversal::FieldType::array_of_records(
                    stringify!(AnchorPoint),
                    self.anchor_points(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for GlyphDataEntry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Individual anchor point.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct AnchorPoint {
    pub x: BigEndian<i16>,
    pub y: BigEndian<i16>,
}

impl AnchorPoint {
    pub fn x(&self) -> i16 {
        self.x.get()
    }

    pub fn y(&self) -> i16 {
        self.y.get()
    }
}

impl FixedSize for AnchorPoint {
    const RAW_BYTE_LEN: usize = i16::RAW_BYTE_LEN + i16::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for AnchorPoint {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "AnchorPoint",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("x", self.x())),
                1usize => Some(Field::new("y", self.y())),
                _ => None,
            }),
            data,
        }
    }
}

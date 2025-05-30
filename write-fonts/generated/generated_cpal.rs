// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

pub use read_fonts::tables::cpal::PaletteType;

/// [CPAL (Color Palette Table)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-table-header) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cpal {
    /// Number of palette entries in each palette.
    pub num_palette_entries: u16,
    /// Number of palettes in the table.
    pub num_palettes: u16,
    /// Total number of color records, combined for all palettes.
    pub num_color_records: u16,
    /// Offset from the beginning of CPAL table to the first
    /// ColorRecord.
    pub color_records_array: NullableOffsetMarker<Vec<ColorRecord>, WIDTH_32>,
    /// Index of each palette’s first color record in the combined
    /// color record array.
    pub color_record_indices: Vec<u16>,
    /// Offset from the beginning of CPAL table to the [Palette Types Array][].
    ///
    /// This is an array of 32-bit flag fields that describe properties of each palette.
    ///
    /// [Palette Types Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-type-array
    pub palette_types_array: NullableOffsetMarker<Vec<PaletteType>, WIDTH_32>,
    /// Offset from the beginning of CPAL table to the [Palette Labels Array][].
    ///
    /// This is an array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with  each palette.
    /// Use 0xFFFF if no name ID is provided for a palette.
    ///
    /// [Palette Labels Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-labels-array
    pub palette_labels_array: NullableOffsetMarker<Vec<u16>, WIDTH_32>,
    /// Offset from the beginning of CPAL table to the [Palette Entry Labels Array][].
    ///
    /// This is an array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with  each palette
    /// entry, e.g. “Outline”, “Fill”. This set of palette entry labels applies
    /// to all palettes in the font. Use  0xFFFF if no name ID is provided for a
    /// palette entry.
    ///
    /// [Palette Entry Labels Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entry-label-array
    pub palette_entry_labels_array: NullableOffsetMarker<Vec<NameId>, WIDTH_32>,
}

impl Cpal {
    /// Construct a new `Cpal`
    pub fn new(
        num_palette_entries: u16,
        num_palettes: u16,
        num_color_records: u16,
        color_records_array: Option<Vec<ColorRecord>>,
        color_record_indices: Vec<u16>,
    ) -> Self {
        Self {
            num_palette_entries,
            num_palettes,
            num_color_records,
            color_records_array: color_records_array.into(),
            color_record_indices,
            ..Default::default()
        }
    }
}

impl FontWrite for Cpal {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = 0 as u16;
        version.write_into(writer);
        self.num_palette_entries.write_into(writer);
        self.num_palettes.write_into(writer);
        self.num_color_records.write_into(writer);
        self.color_records_array.write_into(writer);
        self.color_record_indices.write_into(writer);
        version
            .compatible(1u16)
            .then(|| self.palette_types_array.write_into(writer));
        version
            .compatible(1u16)
            .then(|| self.palette_labels_array.write_into(writer));
        version
            .compatible(1u16)
            .then(|| self.palette_entry_labels_array.write_into(writer));
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Cpal::TAG)
    }
}

impl Validate for Cpal {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Cpal", |ctx| {
            ctx.in_field("color_records_array", |ctx| {
                self.color_records_array.validate_impl(ctx);
            });
            ctx.in_field("color_record_indices", |ctx| {
                if self.color_record_indices.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl TopLevelTable for Cpal {
    const TAG: Tag = Tag::new(b"CPAL");
}

impl<'a> FromObjRef<read_fonts::tables::cpal::Cpal<'a>> for Cpal {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::Cpal<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Cpal {
            num_palette_entries: obj.num_palette_entries(),
            num_palettes: obj.num_palettes(),
            num_color_records: obj.num_color_records(),
            color_records_array: obj.color_records_array().to_owned_obj(offset_data),
            color_record_indices: obj.color_record_indices().to_owned_obj(offset_data),
            palette_types_array: obj.palette_types_array().to_owned_obj(offset_data),
            palette_labels_array: obj.palette_labels_array().to_owned_obj(offset_data),
            palette_entry_labels_array: obj.palette_entry_labels_array().to_owned_obj(offset_data),
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a> FromTableRef<read_fonts::tables::cpal::Cpal<'a>> for Cpal {}

impl<'a> FontRead<'a> for Cpal {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::cpal::Cpal as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl FontWrite for PaletteType {
    fn write_into(&self, writer: &mut TableWriter) {
        writer.write_slice(&self.bits().to_be_bytes())
    }
}

/// [CPAL (Color Record)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entries-and-color-records) record
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorRecord {
    /// Blue value (B0).
    pub blue: u8,
    /// Green value (B1).
    pub green: u8,
    ///     Red value (B2).
    pub red: u8,
    /// Alpha value (B3).
    pub alpha: u8,
}

impl ColorRecord {
    /// Construct a new `ColorRecord`
    pub fn new(blue: u8, green: u8, red: u8, alpha: u8) -> Self {
        Self {
            blue,
            green,
            red,
            alpha,
        }
    }
}

impl FontWrite for ColorRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.blue.write_into(writer);
        self.green.write_into(writer);
        self.red.write_into(writer);
        self.alpha.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("ColorRecord")
    }
}

impl Validate for ColorRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::cpal::ColorRecord> for ColorRecord {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::ColorRecord, _: FontData) -> Self {
        ColorRecord {
            blue: obj.blue(),
            green: obj.green(),
            red: obj.red(),
            alpha: obj.alpha(),
        }
    }
}

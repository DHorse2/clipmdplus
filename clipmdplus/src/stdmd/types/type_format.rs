// type_format.rs DataFormats
#![allow(dead_code, unused_imports, unused_variables)]
/// using oringinally System.Windows.Forms.DataFormats.Format
/// Summary:
///     Provides static, predefined System.Windows.Forms.Clipboard format names. Use
///     them to identify the format of data that you store in an System.Windows.Forms.IDataObject.
/// Clipboard Format
/// <summary>
/// Clipboard Format descriptions
/// </summary>

// other things, then struct doc
/// #region Clipboard Format
/// <summary>
/// Clipboard Format
/// </summary> string[] formats = new String[] see FormatList
#[derive(Clone, Debug, PartialEq)]
pub struct Format {
    name: String,
    id: usize,
    format: FormatType,
}
impl Format {}
// !------------------------------------------------------------
/// Clipboard Format Type enumeration
/// #region Clipboard Format Type
/// <summary>
/// Clipboard Format Type enumeration
/// </summary> string[] format_type = new String[]
#[derive(Clone, Debug, PartialEq)]
pub enum FormatType
// rename from formats_all
// todo add missing formats
// todo add code formats (mdmSRT?)
// todo check serialization per format
{
    /// Bitmap (bmp) image data.
    Bitmap,              // 2
    /// Comma separated text (csv).
    CommaSeparatedValue, // 101 (100 record data. todo Fix)
    /// todo Dib. 
    Dib,                 // 8
    /// todo Dif.
    Dif,                 // 5
    /// todo EnhancedMetafile.
    EnhancedMetafile,    // 14
    /// todo FileDrop.
    FileDrop,            // 15
    /// Html formatted markup text.
    Html,                // 201 (200 document & markup. todo md, uml)
    /// todo Locale.
    Locale,              // 16
    /// todo MetafilePict.
    MetafilePict,        // 3
    /// todo OemText.
    OemText,             // 7
    /// todo Palette.
    Palette,             // 9
    /// todo PenData.
    PenData,             // 10
    /// todo Riff.
    Riff,                // 11
    /// Rtf rich text format.
    Rtf,                 // 11a
    /// todo Serializable.
    Serializable,        // 301
    /// todo StringFormat.
    StringFormat,        // 102
    /// todo SymbolicLink.
    SymbolicLink,        // 4
    /// Text data.
    Text,                // 1
    /// todo Tiff.
    Tiff,                // 6
    /// Unicode Text data.
    UnicodeText,         // 13
    /// WaveAudio (wav) audio data.
    WaveAudio,           // 12
}
// !------------------------------------------------------------
/// Clipboard Format Type Name enumeration
/// #region Clipboard Formats
/// <summary>
/// Clipboard Formats enumeration
/// </summary> string[] formatsAllDesc = new String[]
/*
{
    "Bitmap",
    "CommaSeparatedValue",
    "Dib",
    "Dif",
    "EnhancedMetafile",
    "FileDrop",
    "Html",
    "Locale",
    "MetafilePict",
    "OemText",
    "Palette",
    "PenData",
    "Riff",
    "Rtf",
    "Serializable",
    "StringFormat",
    "SymbolicLink",
    "Text",
    "Tiff",
    "UnicodeText",
    "WaveAudio"
};
 */
// !------------------------------------------------------------
/// Clipboard Format List vector
/// #region Clipboard Format List
/// <summary>
/// Clipboard FormatList vector
/// </summary> string[] format_list = new String[]#[derive(Clone, Debug, PartialEq)]
pub struct FormatList {
    /// FormatList,format contains a vectors of Format objects.
    format: Vec<Format>,
    // or just "vec<Format>",
}

impl FormatList {
    /// Build the format list using standard windows format.
    /// todo Add Linux clipboard data types or build an alternate OS dependant list.
    fn new() -> FormatList {
        FormatList {
            format: vec![
                Format {
                    name: "UnicodeText".to_string(),
                    format: FormatType::UnicodeText,
                    id: 13,
                },
                Format {
                    name: "Text".to_string(),
                    format: FormatType::Text,
                    id: 1,
                },
                Format {
                    name: "Bitmap".to_string(),
                    format: FormatType::Bitmap,
                    id: 2,
                },
                Format {
                    name: "MetafilePict".to_string(),
                    format: FormatType::MetafilePict,
                    id: 3,
                },
                Format {
                    name: "EnhancedMetafile".to_string(),
                    format: FormatType::EnhancedMetafile,
                    id: 14,
                },
                Format {
                    name: "Dif".to_string(),
                    format: FormatType::Dif,
                    id: 5,
                },
                Format {
                    name: "Tiff".to_string(),
                    format: FormatType::Tiff,
                    id: 6,
                },
                Format {
                    name: "OemText".to_string(),
                    format: FormatType::OemText,
                    id: 7,
                },
                Format {
                    name: "Dib".to_string(),
                    format: FormatType::Dib,
                    id: 8,
                },
                Format {
                    name: "Palette".to_string(),
                    format: FormatType::Palette,
                    id: 9,
                },
                Format {
                    name: "PenData".to_string(),
                    format: FormatType::PenData,
                    id: 10,
                },
                Format {
                    name: "Riff".to_string(),
                    format: FormatType::Riff,
                    id: 11,
                },
                Format {
                    name: "WaveAudio".to_string(),
                    format: FormatType::WaveAudio,
                    id: 12,
                },
                Format {
                    name: "SymbolicLink".to_string(),
                    format: FormatType::SymbolicLink,
                    id: 4,
                },
                Format {
                    name: "FileDrop".to_string(),
                    format: FormatType::FileDrop,
                    id: 15,
                },
                Format {
                    name: "Locale".to_string(),
                    format: FormatType::Locale,
                    id: 16,
                },
                Format {
                    name: "CommaSeparatedValue".to_string(),
                    format: FormatType::CommaSeparatedValue,
                    id: 101,
                }, // row data
                Format {
                    name: "Html".to_string(),
                    format: FormatType::Html,
                    id: 201,
                }, // doc and markup
                Format {
                    name: "Serializable".to_string(),
                    format: FormatType::Serializable,
                    id: 301,
                }, // Serializable
                Format {
                    name: "StringFormat".to_string(),
                    format: FormatType::StringFormat,
                    id: 102,
                }, // Dunno
            ],
        }
    }
    // formatCount = formatList.Length;  // [16]
}
// #endregion
// !------------------------------------------------------------

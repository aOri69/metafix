use crate::parser::exif::error::ExifError;

/// Represents the different data types that TIFF/EXIF tags can have.
/// Each variant corresponds to a specific type used in the TIFF specification for image metadata.
///
/// Variants:
/// - Byte (1): An 8-bit unsigned integer.
/// - Ascii (2): An ASCII string (null-terminated).
/// - Short (3): A 16-bit unsigned integer.
/// - Long (4): A 32-bit unsigned integer.
/// - Rational (5): A pair of 32-bit unsigned integers representing a fraction (numerator/denominator).
/// - Undefined (7): An 8-bit byte that can hold any value and is not specifically defined.
/// - Slong (9): A 32-bit signed integer.
/// - Srational (10): A pair of 32-bit signed integers representing a signed fraction.
///
/// Implements [`TryFrom<u16>`] to convert from the numeric TIFF tag type indicator to TagType,
/// returning an error for unknown values.
///
/// The size() method returns the size in bytes of one element of the given type:
/// - BYTE, ASCII, UNDEFINED: 1 byte
/// - SHORT: 2 bytes
/// - LONG, SLONG: 4 bytes
/// - RATIONAL, SRATIONAL: 8 bytes (two 4-byte integers)
#[derive(Debug)]
pub enum TagType {
    Byte = 1,
    Ascii = 2,
    Short = 3,
    Long = 4,
    Rational = 5,
    Undefined = 7,
    Slong = 9,
    Srational = 10,
}

impl TryFrom<u16> for TagType {
    type Error = ExifError;

    /// Implements the [`std::convert::TryFrom<u16>`] trait for `TagType`.
    ///
    /// Allows conversion from a raw TIFF tag type value into the corresponding enum variant.
    /// Returns [`ExifError::UnknownIfdTag`] if the tag type is unrecognized.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Byte,
            2 => Self::Ascii,
            3 => Self::Short,
            4 => Self::Long,
            5 => Self::Rational,
            7 => Self::Undefined,
            9 => Self::Slong,
            10 => Self::Srational,
            val => {
                return Err(ExifError::UnknownIfdTag(val));
            }
        })
    }
}

impl TagType {
    /// Returns the size in bytes of a single element of this type.
    /// 1 | 2 | 7 => 1, // BYTE, ASCII, UNDEFINED
    /// 3 => 2,         // SHORT
    /// 4 | 9 => 4,     // LONG, SLONG
    /// 5 | 10 => 8,    // RATIONAL, SRATIONAL
    /// _ => 0,         // Unknown...
    pub fn size(&self) -> usize {
        match self {
            TagType::Byte => 1,
            TagType::Ascii => 1,
            TagType::Short => 2,
            TagType::Long => 4,
            TagType::Rational => 8,
            TagType::Undefined => 1,
            TagType::Slong => 4,
            TagType::Srational => 8,
        }
    }
}

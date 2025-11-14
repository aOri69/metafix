use thiserror::Error;

const SEGMENT_START: u8 = 0xFF;
// const JFIF_MARKER: u8 = 0xE0;
const EXIF_MARKER: u8 = 0xE1;
const EXIF_HEADER: &[u8; 6] = b"Exif\0\0";
const VALID_SIGNATURE: u16 = 0x002A;
const EXIF_TAG_LEN: usize = 12;

#[derive(Error, Debug)]
pub enum ExifError {
    // #[error(transparent)]
    // Io(#[from] std::io::Error),
    #[error("{0} Segment marker not found")]
    MarkerNotFound(u8),
    #[error("Exif\\0\\0 not found")]
    ExifStartNotFound,
    #[error("Wrong endian: {0:02X} {1:02X} ")]
    WrongEndian(u8, u8),
    #[error("Wrong TIFF signature: {0:02X} ")]
    WrongSignature(u16),
    #[error("Unknown IFD tag: {0}")]
    UnknownIfdTag(u16),
    #[error("Other error: {0}")]
    Other(String),
}

enum Endian {
    Big,
    Little,
}

impl std::fmt::Debug for Endian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Big => write!(f, "Big"),
            Self::Little => write!(f, "Little"),
        }
    }
}

pub struct ExifSegment<'a> {
    start: usize,
    tiff_start: usize,
    len: usize,
    endian: Endian,
    ifd0_offset: usize,
    ifd0_start: usize,
    tags_amount: usize,
    data: &'a [u8],
    tags: Vec<&'a [u8]>,
    vtags: Vec<ExifTag>,
}

impl std::fmt::Debug for ExifSegment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags_str = self
            .tags
            .iter()
            .map(|byte| {
                format!(
                    "{}",
                    byte.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            })
            .collect::<Vec<_>>();

        f.debug_struct("ExifSegment")
            .field("start", &self.start)
            .field("tiff_start", &self.tiff_start)
            .field("len", &self.len)
            .field("endian", &self.endian)
            .field("ifd0_offset", &self.ifd0_offset)
            .field("ifd0_start", &self.ifd0_start)
            .field("tags_amount", &self.tags_amount)
            // .field("data", &self.data)
            .field("tags", &format_args!("{:#?}", tags_str))
            .finish()
    }
}

impl ExifSegment<'_> {}

impl<'a> TryFrom<&'a [u8]> for ExifSegment<'a> {
    type Error = ExifError;

    fn try_from(file_data: &'a [u8]) -> Result<Self, Self::Error> {
        let segment_position = file_data
            .windows(2)
            .position(|window| window == [SEGMENT_START, EXIF_MARKER])
            .ok_or(ExifError::MarkerNotFound(EXIF_MARKER))?;
        // Next two bytes are the size of the segment will be the full length of APP1 segment
        let len = calculate_length_from_bytes([
            file_data[segment_position + 2],
            file_data[segment_position + 2 + 1],
        ]);

        let header = &file_data[segment_position + 4..segment_position + 10];
        if header != EXIF_HEADER {
            return Err(ExifError::ExifStartNotFound);
        }

        let tiff_start = segment_position + 2 + 2 + 6; // 2 - 0xFFE1 marker 2 - length field, 6 - "Exif\0\0"
        let endian = match &file_data[tiff_start..tiff_start + 2] {
            b"II" => Endian::Little,
            b"MM" => Endian::Big,
            [first_byte, second_byte, ..] => {
                return Err(ExifError::WrongEndian(*first_byte, *second_byte));
            }
            _ => return Err(ExifError::Other("Wrong endian array of bytes".to_owned())),
        };

        let (signature, ifd0_offset, tags_amount, tags, vtags) = match endian {
            Endian::Big => {
                let signature =
                    u16::from_be_bytes([file_data[tiff_start + 2], file_data[tiff_start + 3]]);
                let ifd0_offset = u32::from_be_bytes([
                    file_data[tiff_start + 4],
                    file_data[tiff_start + 5],
                    file_data[tiff_start + 6],
                    file_data[tiff_start + 7],
                ]) as usize;
                let tags_amount = u16::from_be_bytes([
                    file_data[tiff_start + ifd0_offset],
                    file_data[tiff_start + ifd0_offset + 1],
                ]) as usize;

                let mut tags = Vec::with_capacity(tags_amount);
                let mut vtags = Vec::with_capacity(tags_amount);
                let ifd0_start = tiff_start + ifd0_offset as usize + 2;
                for tag_number in 0..tags_amount {
                    let tag_start = ifd0_start + tag_number * 12;
                    let tag_end = ifd0_start + tag_number * 12 + 12;
                    tags.push(&file_data[tag_start..tag_end]);
                }

                (signature, ifd0_offset, tags_amount, tags, vtags)
            }
            Endian::Little => {
                let signature =
                    u16::from_le_bytes([file_data[tiff_start + 2], file_data[tiff_start + 3]]);
                let ifd0_offset = u32::from_le_bytes([
                    file_data[tiff_start + 4],
                    file_data[tiff_start + 5],
                    file_data[tiff_start + 6],
                    file_data[tiff_start + 7],
                ]) as usize;
                let tags_amount = u16::from_le_bytes([
                    file_data[tiff_start + ifd0_offset],
                    file_data[tiff_start + ifd0_offset + 1],
                ]) as usize;

                let mut tags = Vec::with_capacity(tags_amount);
                let mut vtags = Vec::with_capacity(tags_amount);
                let ifd0_start = tiff_start + ifd0_offset as usize + 2;
                for tag_number in 0..tags_amount {
                    let tag_start = ifd0_start + tag_number * EXIF_TAG_LEN;
                    let tag_end = ifd0_start + tag_number * EXIF_TAG_LEN + EXIF_TAG_LEN;
                    tags.push(&file_data[tag_start..tag_end]);

                    let tag = ExifTag {
                        id: [file_data[tag_start], file_data[tag_start + 1]],
                        data_type: TagType::try_from(u16::from_le_bytes([
                            file_data[tag_start + 2],
                            file_data[tag_start + 2 + 1],
                        ]))
                        .unwrap(),
                        count: [
                            file_data[tag_start + 4],
                            file_data[tag_start + 4 + 1],
                            file_data[tag_start + 4 + 2],
                            file_data[tag_start + 4 + 3],
                        ],
                        value_offset: Default::default(),
                        value: Default::default(),
                    };
                    vtags.push(tag);
                }

                (signature, ifd0_offset, tags_amount, tags, vtags)
            }
        };

        if signature != VALID_SIGNATURE {
            return Err(ExifError::WrongSignature(signature));
        }

        let data = &file_data[segment_position + 4..segment_position + 4 + len];

        // Now it is a valid Exif segment
        Ok(Self {
            start: segment_position + 4,
            tiff_start,
            len,
            endian,
            ifd0_offset,
            ifd0_start: tiff_start + ifd0_offset as usize,
            tags_amount,
            data,
            tags,
            vtags,
        })
    }
}

fn calculate_length_from_bytes(bytes: [u8; 2]) -> usize {
    ((bytes[0] as usize) << 8) | (bytes[1] as usize)
}

enum TagType {
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
    fn size(type_id: u16) -> usize {
        match type_id {
            1 | 2 | 7 => 1, // BYTE, ASCII, UNDEFINED
            3 => 2,         // SHORT
            4 | 9 => 4,     // LONG, SLONG
            5 | 10 => 8,    // RATIONAL, SRATIONAL
            _ => 0,         // неизвестный
        }
    }
}

struct ExifTag {
    id: [u8; 2],
    data_type: TagType,
    count: [u8; 4],
    value_offset: [u8; 4],
    value: Vec<u8>,
}

#[cfg(test)]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn exif_cbr_test() {
        // arrange
        let tmp = metafix_test_fixtures::get_dir_with_fixtures("simple_album").unwrap();
        let clutch_file = tmp.path().join("simple_album").join("cbr.jpg");
        dbg!(&clutch_file);
        let file_data = std::fs::read(clutch_file).unwrap();
        // act
        let exif = ExifSegment::try_from(file_data.as_slice()).unwrap();
        dbg!(exif);
        // assert
    }
}

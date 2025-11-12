use thiserror::Error;

use crate::parser::{FileParser, ParseError};

#[derive(Error, Debug)]
pub enum JpegError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug)]
pub struct Meta;
pub struct Parser;

impl FileParser for Parser {
    type Output = Meta;

    fn parse(path: &std::path::Path) -> Result<Self::Output, ParseError> {
        // todo!("JPEG parser")
        let file_data = std::fs::read(path).map_err(JpegError::Io)?;
        dbg!(path.display());

        let exif = ExifSegment::try_from(file_data.as_slice())?;
        dbg!(exif);
        // let app0 = get_segment(&file_data, JFIF_MARKER)?;
        // if let Ok(app1) = get_segment(&file_data, EXIF_MARKER) {
        //     let r = file_data.get(app1.start..app1.start + app1.len).unwrap();
        //     for b in r {
        //         print!("{:02X} ", b);
        //     }
        // }

        Err(ParseError::WrongType(path.to_path_buf()))
    }
}

const SEGMENT_START: u8 = 0xFF;
// const JFIF_MARKER: u8 = 0xE0;
const EXIF_MARKER: u8 = 0xE1;
const EXIF_HEADER: &[u8; 6] = b"Exif\0\0";
const VALID_SIGNATURE: u16 = 0x002A;

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

struct ExifSegment<'a> {
    start: usize,
    len: usize,
    endian: Endian,
    data: &'a [u8],
    data_raw: &'a [u8],
}

impl std::fmt::Debug for ExifSegment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ExifSegment {{")?;
        writeln!(f, "    start: {},", self.start)?;
        writeln!(f, "    len: {},", self.len)?;
        writeln!(f, "    endian: {:?},", self.endian)?;
        writeln!(f, "    data: [")?;

        for (i, byte) in self.data_raw.iter().enumerate() {
            if i % 16 == 0 {
                write!(f, "        ")?; // indent column start
            }
            write!(f, "{:02X} ", byte)?;
            if i % 16 == 15 || i == self.data.len() - 1 {
                writeln!(f)?;
            }
        }

        writeln!(f, "    ]")?;
        writeln!(f, "}}")
    }
}

impl ExifSegment<'_> {}

impl<'a> TryFrom<&'a [u8]> for ExifSegment<'a> {
    type Error = JpegError;

    fn try_from(file_data: &'a [u8]) -> Result<Self, Self::Error> {
        let segment_position = file_data
            .windows(2)
            .position(|window| window == [SEGMENT_START, EXIF_MARKER])
            .ok_or(JpegError::Other(format!(
                "{EXIF_MARKER:02X} segment not found"
            )))?;
        // Next two bytes are the size of the segment will be the full length of APP1 segment
        let len = calculate_length_from_bytes([
            file_data[segment_position + 2],
            file_data[segment_position + 2 + 1],
        ]);

        let header = &file_data[segment_position + 4..segment_position + 10];
        if header != EXIF_HEADER {
            return Err(JpegError::Other(format!("Exif\\0\\0 not found")));
        }

        let tiff_start = segment_position + 2 + 2 + 6; // 2 - 0xFFE1 marker 2 - length field, 6 - "Exif\0\0"
        let endian = match &file_data[tiff_start..tiff_start + 2] {
            b"II" => Endian::Little,
            b"MM" => Endian::Big,
            _ => return Err(JpegError::Other(format!("Unknown endian format"))),
        };

        let signature = match endian {
            Endian::Big => {
                u16::from_be_bytes([file_data[tiff_start + 2], file_data[tiff_start + 3]])
            }
            Endian::Little => {
                u16::from_le_bytes([file_data[tiff_start + 2], file_data[tiff_start + 3]])
            }
        };

        if signature != VALID_SIGNATURE {
            return Err(JpegError::Other(format!(
                "Signature in TIFF header of Exif segment is invalid"
            )));
        }

        let data = &file_data[tiff_start..tiff_start + (len - 6)]; // minus 6 bytes because of Exif\0\0
        //TODO: Check that the data slice is with the correct offset
        let data_raw = &file_data[segment_position..segment_position + len - 1]; // seems that this is the correct offset

        // Now it is a valid Exif segment
        Ok(Self {
            start: segment_position + 2,
            len,
            endian,
            data,
            data_raw,
        })
    }
}

struct Segment {
    marker: u8,
    start: usize,
    len: usize,
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Segment {:02X} start: {} len: {}",
            self.marker, self.start, self.len
        )
    }
}

fn get_segment(file_data: &[u8], segment_marker: u8) -> Result<Segment, JpegError> {
    let segment_position = file_data
        .windows(2)
        .position(|window| window == [0xFF, segment_marker])
        .ok_or(JpegError::Other(format!(
            "{segment_marker:02X} segment not found"
        )))?;
    // Next two bytes are the size of the segment
    let segment_size = calculate_length_from_bytes([
        *file_data
            .get(segment_position + 2)
            .ok_or(JpegError::Other("first size byte not found".into()))?,
        *file_data
            .get(segment_position + 2 + 1)
            .ok_or(JpegError::Other("second size byte not found".into()))?,
    ]);

    Ok(Segment {
        marker: segment_marker,
        start: segment_position + 2,
        len: segment_size,
    })
}

fn calculate_length_from_bytes(bytes: [u8; 2]) -> usize {
    println!("{:02X}", bytes[0]);
    println!("{:02X}", bytes[1]);
    ((bytes[0] as usize) << 8) | (bytes[1] as usize)
}

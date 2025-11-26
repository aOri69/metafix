pub mod byte_order;
pub mod error;
pub mod segment;
pub mod tag;

pub use byte_order::*;
pub use segment::*;
pub use tag::*;

const SEGMENT_START: u8 = 0xFF;
const EXIF_MARKER: u8 = 0xE1;
const EXIF_HEADER: &[u8; 6] = b"Exif\0\0";
const VALID_SIGNATURE: u16 = 0x002A;
const EXIF_TAG_LEN: usize = 12;

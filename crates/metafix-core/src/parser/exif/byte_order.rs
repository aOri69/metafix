use thiserror::Error;

#[derive(Error, Debug)]
pub enum ByteOrderError {
    #[error("Wrong endian: {0:02X} {1:02X} ")]
    WrongEndian(u8, u8),
    #[error("No endian bytes were found")]
    NoEndian,
    #[error("Not enough bytes to convert the slice to number")]
    NotEnoughBytes,
}

pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl std::fmt::Debug for ByteOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BigEndian => write!(f, "Big"),
            Self::LittleEndian => write!(f, "Little"),
        }
    }
}

impl TryFrom<&[u8]> for ByteOrder {
    type Error = ByteOrderError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let byte_order_bytes = value.get(0..2);
        match byte_order_bytes {
            Some(b"II") => Ok(Self::LittleEndian),
            Some(b"MM") => Ok(Self::BigEndian),
            Some([first_byte, second_byte, ..]) => {
                Err(ByteOrderError::WrongEndian(*first_byte, *second_byte))
            }
            _ => Err(ByteOrderError::NoEndian),
        }
    }
}

impl ByteOrder {
    pub fn read_u16(&self, bytes: &[u8]) -> Result<u16, ByteOrderError> {
        let bytes = bytes.get(0..2).ok_or(ByteOrderError::NotEnoughBytes)?;
        match self {
            ByteOrder::BigEndian => Ok(u16::from_be_bytes([bytes[0], bytes[1]])),
            ByteOrder::LittleEndian => Ok(u16::from_le_bytes([bytes[0], bytes[1]])),
        }
    }

    pub fn read_u32(&self, bytes: &[u8]) -> Result<u32, ByteOrderError> {
        let bytes = bytes.get(0..4).ok_or(ByteOrderError::NotEnoughBytes)?;
        match self {
            ByteOrder::BigEndian => {
                Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            ByteOrder::LittleEndian => {
                Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
        }
    }
}

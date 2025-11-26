use crate::parser::exif::{
    ByteOrder, EXIF_HEADER, EXIF_MARKER, EXIF_TAG_LEN, SEGMENT_START, TagType, VALID_SIGNATURE,
    error::ExifError,
};

pub struct ExifSegment {
    start: usize,
    tiff_start: usize,
    len: usize,
    endian: ByteOrder,
    ifd0_offset: usize,
    ifd0_start: usize,
    tags_amount: usize,
    tags: Vec<[u8; 12]>,
}

impl std::fmt::Debug for ExifSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tags_str = self
            .tags
            .iter()
            .map(|byte| {
                byte.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_string()
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

impl ExifSegment {}

impl<'a> TryFrom<&'a [u8]> for ExifSegment {
    type Error = ExifError;

    fn try_from(file_data: &'a [u8]) -> Result<Self, Self::Error> {
        // APP1 segment
        let segment_position = file_data
            .windows(2)
            .position(|window| window == [SEGMENT_START, EXIF_MARKER])
            .ok_or(ExifError::MarkerNotFound(EXIF_MARKER))?;
        // Size of the segment will be the full length of APP1 segment
        let segment_length = ((file_data[segment_position + 2] as usize) << 8)
            | (file_data[segment_position + 2 + 1] as usize);

        let header = &file_data[segment_position + 4..segment_position + 10];
        if header != EXIF_HEADER {
            return Err(ExifError::ExifStartNotFound);
        }

        let tiff_start = segment_position + 2 + 2 + 6; // 2 - 0xFFE1 marker 2 - length field, 6 - "Exif\0\0"

        let endian = ByteOrder::try_from(&file_data[tiff_start..tiff_start + 2])?;

        let signature = endian.read_u16(
            file_data
                .get(tiff_start + 2..tiff_start + 4)
                .ok_or(ExifError::Other("Cannot read EXIF signature"))?,
        )?;
        let ifd0_offset = endian.read_u32(
            file_data
                .get(tiff_start + 4..tiff_start + 8)
                .ok_or(ExifError::Other("Cannot read EXIF IFD0 offset"))?,
        )?;

        println!("sig - {} ifd0 - {}", signature, ifd0_offset);

        println!(
            "{}",
            file_data
                .get(tiff_start + 2..tiff_start + 4)
                .ok_or(ExifError::Other("Cannot read EXIF signature"))?
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!(
            "{}",
            file_data
                .get(tiff_start + 4..tiff_start + 8)
                .ok_or(ExifError::Other("Cannot read EXIF IFD0 offset"))?
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );

        let (signature, ifd0_offset, tags_amount, tags) = match endian {
            ByteOrder::BigEndian => {
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
                let ifd0_start = tiff_start + ifd0_offset + 2;
                for tag_number in 0..tags_amount {
                    let tag_start = ifd0_start + tag_number * 12;
                    let tag_end = ifd0_start + tag_number * 12 + 12;
                    tags.push(
                        file_data[tag_start..tag_end]
                            .try_into()
                            .expect("Expected to parse"),
                    );
                }

                (signature, ifd0_offset, tags_amount, tags)
            }
            ByteOrder::LittleEndian => {
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
                let ifd0_start = tiff_start + ifd0_offset + 2;
                for tag_number in 0..tags_amount {
                    let tag_start = ifd0_start + tag_number * EXIF_TAG_LEN;
                    let tag_end = ifd0_start + tag_number * EXIF_TAG_LEN + EXIF_TAG_LEN;
                    tags.push(
                        file_data[tag_start..tag_end]
                            .try_into()
                            .expect("Expected to parse"),
                    );

                    let id = [file_data[tag_start], file_data[tag_start + 1]];
                    let data_type = TagType::try_from(u16::from_le_bytes([
                        file_data[tag_start + 2],
                        file_data[tag_start + 2 + 1],
                    ]))
                    .unwrap();
                    let count = u32::from_le_bytes([
                        file_data[tag_start + 4],
                        file_data[tag_start + 4 + 1],
                        file_data[tag_start + 4 + 2],
                        file_data[tag_start + 4 + 3],
                    ]) as usize;
                    let value = if data_type.size() * count > 4 {
                        let value_offset = u32::from_le_bytes([
                            file_data[tag_start + 8],
                            file_data[tag_start + 9],
                            file_data[tag_start + 10],
                            file_data[tag_start + 11],
                        ]) as usize;
                        &file_data[tiff_start + value_offset..tiff_start + value_offset + count]
                    } else {
                        &file_data[tag_start + 8..tag_end]
                    };

                    println!("{:?}, {:?}, {}", id, data_type, count);
                    let v = value
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    println!("{}", v);
                }

                (signature, ifd0_offset, tags_amount, tags)
            }
        };

        if signature != VALID_SIGNATURE {
            return Err(ExifError::WrongSignature(signature));
        }

        let _data = &file_data[segment_position + 4..segment_position + 4 + segment_length];

        // Now it is a valid Exif segment
        Ok(Self {
            start: segment_position + 4,
            tiff_start,
            len: segment_length,
            endian,
            ifd0_offset,
            ifd0_start: tiff_start + ifd0_offset,
            tags_amount,
            tags,
        })
    }
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
        println!("{}", clutch_file.display());
        let file_data = std::fs::read(clutch_file).unwrap();
        // act
        let exif = ExifSegment::try_from(file_data.as_slice()).unwrap();
        println!("{:#?}", exif);
        // assert
    }

    #[test]
    fn exif_clutch_test() {
        // arrange
        let tmp = metafix_test_fixtures::get_dir_with_fixtures("simple_album").unwrap();
        let clutch_file = tmp.path().join("simple_album").join("clutch.jpg");
        println!("{}", clutch_file.display());
        let file_data = std::fs::read(clutch_file).unwrap();
        // act
        let exif = ExifSegment::try_from(file_data.as_slice()).unwrap();
        println!("{:#?}", exif);
        // assert
    }

    #[test]
    fn exif_dog_test() {
        // arrange
        let tmp = metafix_test_fixtures::get_dir_with_fixtures("simple_album").unwrap();
        let clutch_file = tmp.path().join("simple_album").join("clutch.jpg");
        println!("{}", clutch_file.display());
        let file_data = std::fs::read(clutch_file).unwrap();
        // act
        let exif = ExifSegment::try_from(file_data.as_slice()).unwrap();
        println!("{:#?}", exif);
        // assert
    }
}

use crate::parser::exif::{
    ByteOrder, EXIF_HEADER, EXIF_MARKER, EXIF_TAG_LEN, SEGMENT_START, Tag, TagType,
    VALID_SIGNATURE, error::ExifError,
};

#[derive(Debug)]
pub struct ExifSegment {
    start: usize,
    tiff_start: usize,
    len: usize,
    endian: ByteOrder,
    ifd0_offset: usize,
    ifd0_start: usize,
    tags_amount: usize,
    tags: Vec<Tag>,
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

        let endian = ByteOrder::try_from(
            file_data
                .get(tiff_start..=tiff_start + 2)
                .ok_or(ExifError::SliceRead("Cannot read Endian bytes"))?,
        )?;

        let signature = endian.read_u16(
            file_data
                .get(tiff_start + 2..tiff_start + 4)
                .ok_or(ExifError::SliceRead("Cannot read EXIF signature"))?,
        )?;

        if signature != VALID_SIGNATURE {
            return Err(ExifError::WrongSignature(signature));
        }

        let ifd0_offset = endian.read_u32(
            file_data
                .get(tiff_start + 4..tiff_start + 8)
                .ok_or(ExifError::SliceRead("Cannot read EXIF IFD0 offset"))?,
        )? as usize;
        let tags_amount = endian.read_u16(
            file_data
                .get(tiff_start + ifd0_offset..tiff_start + ifd0_offset + 2)
                .ok_or(ExifError::SliceRead("Failed to read Tags amount"))?,
        )? as usize;

        let mut tags = Vec::with_capacity(tags_amount);
        let ifd0_start = tiff_start + ifd0_offset + 2;
        for tag_number in 0..tags_amount {
            let tag_start = ifd0_start + tag_number * EXIF_TAG_LEN;
            let tag_end = ifd0_start + tag_number * EXIF_TAG_LEN + EXIF_TAG_LEN;

            let id: [u8; 2] = file_data
                .get(tag_start..=tag_start + 1)
                .ok_or(ExifError::SliceRead("Tag ID"))?
                .try_into()?;
            let data_type = TagType::try_from(
                endian.read_u16(
                    file_data
                        .get(tag_start + 2..=tag_start + 2 + 1)
                        .ok_or(ExifError::SliceRead("Data Type"))?,
                )?,
            )?;
            let count = endian.read_u32(
                file_data
                    .get(tag_start + 4..=tag_start + 4 + 3)
                    .ok_or(ExifError::SliceRead("Count"))?,
            )? as usize;

            let value = if data_type.size() * count > 4 {
                let value_offset = endian.read_u32(
                    file_data
                        .get(tag_start + 8..tag_start + 12)
                        .ok_or(ExifError::SliceRead("Value Offset"))?,
                )? as usize;
                file_data
                    .get(tiff_start + value_offset..=tiff_start + value_offset + count)
                    .ok_or(ExifError::SliceRead("Value"))?
            } else {
                file_data
                    .get(tag_start + 8..=tag_end)
                    .ok_or(ExifError::SliceRead("Value Short"))?
            };

            tags.push(Tag {
                id,
                kind: data_type,
                value: value.to_vec(),
            });
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
        let clutch_file = tmp.path().join("simple_album").join("dog.jpg");
        println!("{}", clutch_file.display());
        let file_data = std::fs::read(clutch_file).unwrap();
        // act
        let exif = ExifSegment::try_from(file_data.as_slice()).unwrap();
        println!("{:#?}", exif);
        // assert
    }
}

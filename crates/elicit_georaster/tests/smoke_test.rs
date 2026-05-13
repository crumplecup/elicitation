//! Smoke tests for `elicit_georaster`.

use elicit_georaster::{ColorType, Coordinate, GeoTiffReader, RasterValue};
use std::io::Cursor;
use tiff::encoder::{TiffEncoder, colortype};

fn gray8_tiff_bytes(width: u32, height: u32, data: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut encoder = TiffEncoder::new(&mut cursor).expect("create TIFF encoder");
        encoder
            .write_image::<colortype::Gray8>(width, height, data)
            .expect("write TIFF image");
    }
    cursor.into_inner()
}

fn multipage_tiff_bytes() -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut encoder = TiffEncoder::new(&mut cursor).expect("create TIFF encoder");
        encoder
            .write_image::<colortype::Gray16>(2, 2, &[1_u16, 2, 3, 4])
            .expect("write first page");
        encoder
            .write_image::<colortype::Gray8>(3, 3, &[9_u8, 8, 7, 6, 5, 4, 3, 2, 1])
            .expect("write second page");
    }
    cursor.into_inner()
}

#[test]
fn reader_reads_known_gray8_pixels() {
    let bytes = gray8_tiff_bytes(2, 2, &[1, 2, 3, 4]);
    let reader = GeoTiffReader::open(Cursor::new(bytes)).expect("open GeoTIFF");

    assert_eq!(reader.images().len(), 1);
    let info = reader.image_info();
    assert_eq!(info.dimensions, Some((2, 2)));
    assert_eq!(info.colortype, Some(ColorType::Gray(8)));
    assert_eq!(info.samples, 1);
    assert_eq!(reader.read_pixel(0, 0), RasterValue::U8(1));
    assert_eq!(reader.read_pixel(1, 0), RasterValue::U8(2));
    assert_eq!(reader.read_pixel(0, 1), RasterValue::U8(3));
    assert_eq!(
        reader.read_pixel_at_location(Coordinate { x: 0.0, y: 0.0 }),
        RasterValue::NoData
    );
    assert!(reader.origin().is_none());
    assert!(reader.pixel_size().is_none());
}

#[test]
fn reader_collects_pixel_windows() {
    let bytes = gray8_tiff_bytes(2, 2, &[1, 2, 3, 4]);
    let reader = GeoTiffReader::open(Cursor::new(bytes)).expect("open GeoTIFF");

    let pixels = reader.pixels(0, 0, 2, 2);
    assert_eq!(pixels.len(), 4);
    assert_eq!(
        pixels.items,
        vec![
            (0, 0, RasterValue::U8(1)),
            (1, 0, RasterValue::U8(2)),
            (0, 1, RasterValue::U8(3)),
            (1, 1, RasterValue::U8(4)),
        ]
    );
}

#[test]
fn reader_tracks_current_image() {
    let bytes = multipage_tiff_bytes();
    let mut reader = GeoTiffReader::open(Cursor::new(bytes)).expect("open multipage TIFF");

    assert_eq!(reader.image_info().dimensions, Some((3, 3)));
    reader.seek_to_image(0).expect("seek to first page");
    assert_eq!(reader.image_info().dimensions, Some((2, 2)));
}

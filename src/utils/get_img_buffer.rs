extern crate image;

use image::{open, GenericImageView, ImageBuffer};

pub type Buffer = ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;

pub struct Size {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub ow: u32,
    pub oh: u32,
}

pub fn get_img_buffer(path: &str) -> (Buffer, Size) {
    let img = open(path).unwrap();
    let (ow, oh) = img.dimensions();
    let img_buf: Buffer = ImageBuffer::new(ow, oh);
    let mut no_empty_info: Vec<(u32, u32)> = vec![];
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img_buf.enumerate_pixels() {
        let raw_data = img.get_pixel(x, y);
        if !isEmpty(&raw_data) {
            no_empty_info.push((x, y))
        }
    }
    no_empty_info.sort_by(|a, b| {
        let ax = a.0;
        let bx = b.0;
        ax.cmp(&bx)
    });
    let x1 = no_empty_info.first().unwrap().0;
    let x2 = no_empty_info.last().unwrap().0;

    no_empty_info.sort_by(|a, b| {
        let ay = a.1;
        let by = b.1;
        ay.cmp(&by)
    });

    let y1 = no_empty_info.first().unwrap().1;
    let y2 = no_empty_info.last().unwrap().1;
    let w = x2 - x1;
    let h = y2 - y1;
    let mut img_buf = ImageBuffer::new(w, h);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = img.get_pixel(x1 + x, y1 + y);
    }
    let size = Size {
        x: x1,
        y: y1,
        w,
        h,
        ow,
        oh,
    };
    (img_buf, size)
}

pub type List<'a> = Vec<((u32, u32), &'a Buffer)>;
type Info<'a> = ((u32, u32), List<'a>);
pub fn combine(info: Info) -> Buffer {
    let ((width, height), list) = info;
    let mut all_buffer = ImageBuffer::new(width, height);
    for item in &list {
        let ((x, y), buffer) = item;
        for (ix, iy, pixel) in buffer.enumerate_pixels() {
            all_buffer.put_pixel(x + ix, y + iy, *pixel);
        }
    }
    return all_buffer;
}


fn isEmpty(pixel: &image::Rgba<u8>) -> bool {
    let [r, g, b, a] = &pixel.data;
    if r + g + b + a == 0 {
        return true;
    }
    false
}

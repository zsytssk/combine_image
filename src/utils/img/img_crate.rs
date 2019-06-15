
// use image::{open, GenericImageView, ImageBuffer};
use raster::{open, Color, Image};

pub type Buffer = Image;
pub type List<'a> = Vec<((i32, i32), &'a Buffer)>;
pub type Info<'a> = ((i32, i32), List<'a>);

pub struct Size {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub ow: i32,
    pub oh: i32,
}

pub fn get_img_buffer(path: &str) -> (Buffer, Size) {
    let img = open(path).unwrap();
    let ow = img.width;
    let oh = img.height;

    let mut no_empty_info: Vec<(i32, i32)> = vec![];

    // Iterate over the coordinates and pixels of the image
    for x in 0..ow {
        for y in 0..oh {
            let raw_data = img.get_pixel(x, y).unwrap();
            if !is_empty(&raw_data) {
                no_empty_info.push((x, y))
            }
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
    let mut img_buf = Image::blank(w, h);
    for x in 0..w {
        for y in 0..h {
            img_buf
                .set_pixel(x, y, img.get_pixel(x1 + x, y1 + y).unwrap())
                .unwrap();
        }
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

pub fn combine(info: Info) -> Buffer {
    let ((width, height), list) = info;
    let mut all_buffer = Image::blank(width, height);
    for item in &list {
        let ((x, y), buffer) = item;

        for ix in 0..buffer.width {
            for iy in 0..buffer.height {
                all_buffer
                    .set_pixel(x + ix, y + iy, buffer.get_pixel(ix, iy).unwrap())
                    .unwrap();
            }
        }
    }
    return all_buffer;
}

pub fn save(img: Buffer, path: &str) {
    raster::save(&img, path).unwrap();
}
pub fn size(img: &Buffer) -> (i32, i32) {
    (*&img.width, *&img.height)
}

fn is_empty(pixel: &Color) -> bool {
    let Color { r, g, b, a } = pixel;
    if r + g + b + a == 0 {
        return true;
    }
    false
}

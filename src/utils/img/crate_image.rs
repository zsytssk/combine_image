use image::{open, DynamicImage, GenericImageView, ImageBuffer, ImageResult};

pub type Buffer = ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;
pub type List<'a> = Vec<((i32, i32), &'a Buffer)>;
pub type Info<'a> = ((i32, i32), List<'a>);

pub struct Size {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub ow: u32,
    pub oh: u32,
}

pub async fn get_img_buffer(path: &str) -> (Buffer, Size) {
    let img = open_img(path).await.unwrap();
    let (ow, oh) = img.dimensions();
    let mut no_empty_info: Vec<(u32, u32)> = vec![];
    for x in 0..ow {
        for y in 0..oh {
            let raw_data = img.get_pixel(x, y);
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

pub fn combine(info: Info) -> Buffer {
    let ((width, height), list) = info;
    let mut all_buffer = ImageBuffer::new(width as u32, height as u32);
    for item in &list {
        let ((x, y), buffer) = item;
        for (ix, iy, pixel) in buffer.enumerate_pixels() {
            all_buffer.put_pixel((*x as u32) + ix, (*y as u32) + iy, *pixel);
        }
    }
    return all_buffer;
}

pub async fn save(img: Buffer, path: String) {
    img.save(path).unwrap();
}
pub async fn open_img(path: &str) -> ImageResult<DynamicImage> {
    let img = open(path);
    img
}
pub fn size(img: &Buffer) -> (i32, i32) {
    let (w, h) = img.dimensions();
    (w as i32, h as i32)
}

fn is_empty(pixel: &image::Rgba<u8>) -> bool {
    let image::Rgba([r, g, b, a]) = &pixel;
    if r + g + b + a == 0 {
        return true;
    }
    false
}

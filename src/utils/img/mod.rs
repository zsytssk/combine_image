mod img_raster;

pub type Buffer = img_raster::Buffer;
pub type Size = img_raster::Size;
pub type List<'a> = img_raster::List<'a>;
pub type Info<'a> = img_raster::Info<'a>;

pub fn get_img_buffer(path: &str) ->(Buffer, Size) {
    img_raster::get_img_buffer(path)
}
pub fn combine(info: Info) -> Buffer {
    img_raster::combine(info)
}
pub fn save(img: Buffer, path: &str)  {
    img_raster::save(img, path)
}
pub fn size(img: &Buffer) -> (i32, i32) {
    img_raster::size(img)
}
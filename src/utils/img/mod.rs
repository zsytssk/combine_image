mod crate_image;

pub type Buffer = crate_image::Buffer;
pub type Size = crate_image::Size;
pub type List<'a> = crate_image::List<'a>;
pub type Info<'a> = crate_image::Info<'a>;

pub async fn get_img_buffer(path: &str) -> (Buffer, Size) {
    crate_image::get_img_buffer(path).await
}
pub fn combine(info: Info) -> Buffer {
    crate_image::combine(info)
}
pub async fn save(img: Buffer, path: String) {
    crate_image::save(img, path).await;
}
pub fn size(img: &Buffer) -> (i32, i32) {
    crate_image::size(img)
}

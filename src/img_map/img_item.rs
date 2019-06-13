use crate::utils::get_img_buffer::{Buffer, Size};
use crate::utils::path::file_name;
use std::fmt;

pub struct ImgItem {
    pub name: String,
    pub buffer: Buffer,
    pub x: u32,
    pub y: u32,
    pub ori_size: Size,
}
impl ImgItem {
    pub fn new(name: String, buffer: Buffer, ori_size: Size) -> ImgItem {
        let name = file_name(&name).to_string();
        ImgItem {
            name,
            buffer,
            x: 0,
            y: 0,
            ori_size,
        }
    }
    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

impl fmt::Debug for ImgItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgItem {{{}: ({},{})}}", self.name, self.x, self.y)
    }
}

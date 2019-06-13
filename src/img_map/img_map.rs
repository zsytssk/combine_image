use super::super::utils::{
    get_img_buffer::{combine, Buffer, List},
    pack_rect::{run as pack, RectList},
};

use super::img_item::ImgItem;
use super::to_json::to_json;
use crate::state;

#[derive(Debug)]
pub struct ImgMap {
    pub name: String,
    pub list: Vec<ImgItem>,
    pub width: u32,
    pub height: u32,
}

impl ImgMap {
    pub fn new(name: String, list: Vec<ImgItem>) -> ImgMap {
        ImgMap {
            name,
            list,
            width: 0,
            height: 0,
        }
    }
    pub fn set_img_pos(&mut self) {
        let list = &mut self.list;
        let mut rect_list: RectList = vec![];
        let state = &(&state::STATE).lock().unwrap();
        let space_width = state.space_width;
        let space_height = state.space_height;
        for item in list.iter() {
            let (w, h) = item.buffer.dimensions();
            rect_list.push(((w + space_width) as i32, (h + space_height) as i32))
        }
        let ((width, height), pos_list) = pack(rect_list);

        for (i, item) in list.iter_mut().enumerate() {
            let (x, y) = pos_list[i];
            item.set_pos(x as u32, y as u32);
        }
        self.set_size(width as u32, height as u32);
    }
    pub fn combine(&mut self) -> Buffer {
        self.set_img_pos();

        let w = self.width;
        let h = self.height;
        let list = &self.list;
        let mut buffer_list: List = vec![];
        for item in list {
            let x = item.x;
            let y = item.y;
            let buffer = &item.buffer;
            buffer_list.push(((x, y), buffer));
        }
        return combine(((w, h), buffer_list));
    }
    pub fn set_size(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }
    pub fn to_json(&mut self) -> String {
        to_json(self)
    }
}

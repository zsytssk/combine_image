mod img_item;
mod img_map;
mod to_json;

use super::utils::{
    img::{get_img_buffer, size},
    walk_path::PathMap,
};

use img_item::ImgItem;
use img_map::ImgMap;
use std::cmp::Ordering;

pub async fn run(path: PathMap) -> ImgMap {
    let (name, list) = path.data();
    let mut img_list: Vec<ImgItem> = vec![];

    for item in list {
        let (buffer, no_empty_info) = get_img_buffer(&item).await;
        let img_item = ImgItem::new(item, buffer, no_empty_info);
        img_list.push(img_item);
    }

    let mut same_size = true;
    img_list.sort_by(|a, b| {
        let (aw, ah) = size(&a.buffer);
        let (bw, bh) = size(&b.buffer);
        let result = (bw * bh).cmp(&(aw * ah));
        match result {
            Ordering::Equal => {}
            _ => {
                same_size = false;
            }
        }
        result
    });

    ImgMap::new(name, img_list)
}

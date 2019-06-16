use super::img_map::ImgMap;

use crate::state;
use crate::utils::img::size;
use crate::utils::path;

pub fn to_json(img_map: &mut ImgMap) -> String {
    let image = format!("{}.png", path::file_name(&img_map.name));
    let state = (&state::STATE).lock().unwrap();
    let src = &state.src;
    let pre_prefix = &state.prefix;
    let mut prefix = path::relative(&img_map.name, &src).unwrap();
    prefix = format!("{}{}/", pre_prefix, prefix);
    prefix = path::normalize(&prefix);
    drop(state);

    let meta = format!("{{\"image\": \"{}\", \"prefix\": \"{}\"}}", image, prefix);
    let mut frames = "".to_owned();
    let list = &img_map.list;
    for (index, item) in list.iter().enumerate() {
        let buffer = &item.buffer;
        let (w, h) = size(buffer);
        let ori_size = &item.ori_size;
        let frame = format!(
            " {{ \"h\": {}, \"idx\": 0, \"w\": {}, \"x\": {}, \"y\": {} }}",
            w, h, item.x, item.y
        );
        let source_size = format!(" {{ \"h\": {}, \"w\": {} }}", ori_size.oh, ori_size.ow,);
        let sprite_source_size = format!(" {{ \"x\": {}, \"y\": {} }}", ori_size.x, ori_size.y);

        let item_str = format!(
            "\"{}\": {{\"frame\": {}, \"sourceSize\": {}, \"spriteSourceSize\": {}}}",
            item.name, frame, source_size, sprite_source_size
        );
        if index == 0 {
            frames += &item_str;
        } else {
            frames = format!("{}, {}", frames, item_str);
        }
    }

    format!("{{\"frames\":{{{}}}, \"meta\": {}}}", frames, meta)
}

use rect_packer::DensePacker;

pub type RectList = Vec<(i32, i32)>;
pub type Result = ((i32, i32), RectList);

pub fn run(rect_list: RectList) -> Result {
    let (sw, sh) = rect_list[0];
    let mut result: RectList = vec![];
    let mut packer = DensePacker::new(sw, sh);

    for rect in rect_list {
        let (w, h) = rect;
        if !packer.can_pack(w, h, false) {
            let (ow, oh) = packer.size();
            let (nw, nh) = (ow + w, oh + h);

            /* 判断宽高, 那个小 先增加哪一个  */
            if ow < oh {
                packer.resize(nw, oh);
            } else {
                packer.resize(ow, nh);
            }
            if !packer.can_pack(w, h, false) {
                packer.resize(nw, nh);
            }
        }
        if let Some(rect) = packer.pack(w, h, false) {
            let rx = rect.x;
            let ry = rect.y;
            result.push((rx, ry))
        } else {
            panic!("Failed to pack texture");
        }
    }

    (packer.size(), result)
}

#[test]
fn test_run() {
    let rectangles = vec![(50, 500), (350, 100), (255, 100)];
    run(rectangles);
}

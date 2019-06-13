## borrow check

```rs
let list = &mut self.list; // let mut list = self.list 会将 self.list 给了list 显然不行
let mut rect_list: RectList = vec![];
/**
 *  直接使用 list 就会 move 掉,
 *  使用&list, 报错 因为&list 不是 iterator
 *  为什么list.iter()这个不会丢掉
 */
for item in list.iter() {
    let (w, h) = item.buffer.dimensions();
    rect_list.push((w as i32, h as i32))
}
let ((width, height), pos_list) = pack(rect_list);

for (i, item) in list.iter_mut().enumerate() {
    let (x, y) = pos_list[i];
    item.set_pos(x as u32, y as u32);
}
self.set_size(width as u32, height as u32);
```

## 2019-04-17 09:00:20

- @ques 如何获取 dir_name
- @ques 如何将字符串写入文件...
- @ques format 一个个的不方便 有没有更好的方法

  - 找不到只能字符串 replace 了

- @todo combine 可以先比较宽高 增加小的 如果不行 再增加相应的高...

- @ques 能不能将读取文件 写入文件放到 别的进程中...
  - 主进程只用来计算...

* rust js 学习过程

* @ques io 请求 能做成异步的吗

  - 放在另外的进程里面

* @ques rust mod absolute in libs

* @ques 我怎么把文件的路劲传过来
* @ques 如何获取路径 dir_name file_name

- @todo
  - 保存名称

* @todo 从大到小排列文件...
* @todo 计算函数的执行时间...
  - 最耗时间的地方都在读取文件+保存文件
    - 保存文件更大

## 2019-04-03 09:00:46

```rs
for item in list.iter() {
    let (w, h) = item.buffer.dimensions();
    rect_list.push((w as i32, h as i32))
}
let ((width, height), pos_list) = pack(rect_list);

for (i, item) in list.iter_mut().enumerate() {
    let (x, y) = pos_list[i];
    item.set_pos(x as u32, y as u32);
}
```

- @ques 如何设置 list.iter().enumerate

  - mut

- @bug 空白区域越来越大

  - 裁剪 空白区域

- @todo 忽略非 png 图片

  - 如果合并的文件过大.. 分成多个文件

- @ques 太慢了如何处理

  - 能不能保存文件把做成异步的 io 性能问题
  - multi_thread

- @ques rust 本身卡不卡

- @ques 如何监听文件的修改

- @todo 从大到小排列文件...

```rs
image::save_buffer(&Path::new("image.png"), buffer, 800, 600, image::RGBA(8))
```

## 2019-04-02 17:48:20

- @todo 保存的文件位置
- @todo 图片排布的算法...!!!

  - https://pdfs.semanticscholar.org/1120/310a8334043e0ff03d065ca9700f36746f18.pdf
  - rectangle packing algorithm rust

- https://github.com/mapbox/potpack
- @todo run to_json_str

- [diary] 真是胆小如鼠

- @ques 如果我要直接将 ImgMap 转化为 json
  - 显然格式不支持
  - 我需要重新创建一个 struct...
  - 或者能不能将 struct 传到一个 fun 中自动就转化成了字符串 [最好如此]

## 2019-04-02 14:27:45

- @ques rust mod ab

- @ques rust convert struct to json

  - 如何转化为字符串
  - 如何将字符保存为本地文件

- @todo 生成 json 文件...

- @ques 文件比 laya 的大如何处理

  - 比较原宽高
  - 图片之间有没有间隙

- @ques cargo compress image

  - https://medium.com/@pjcoup/10-minute-png-compressor-in-rust-e8244e4c230f

- @ques 能不能将 image.save open .. 变成异步的...

  - rust async io

- Rectangular spacing algorithm?

- @note rust 类型是可以自动转化的

- @ques [lodepng-rust](https://github.com/kornelski/lodepng-rust)

## 2019-04-01 14:28:43

- @quesa 这速度还是很慢

  - 也许是 inject_img 这时候一个像素像素去做太耗时间了
  - ***
  - 也许是读取文件的时候太慢了...
  - ***
  - all_buffer.save 这时候太慢了...
  - ***
  - 极限在读取和保存文件 在 io...
    - 最好能用异步的处理
    - 可以试试在好的机器上看的效果

- @ques 怎么测试性能...

- 下面是威慑呢么??

```rs
https://stackoverflow.com/questions/31661713/borrowed-value-does-not-live-long-enough-when-pushing-into-a-vector
let mut buffer_list: Vec<&Buffer> = vec![];
for item in list { // &list 就可以 为什么...
    buffer_list.push(&item.buffer);
}
combine(&name, buffer_list);
```

- @todo stackoverflow 鼠标效果 曾小平
- @todo path get file name...

- @ques PathMap data 返回类型能不能 直接使用...

  - Self.

- @ques rust 能不能 rust destructor struct

- @ques get img file
  - png jpg ...

## 2019-04-01 09:18:02

- @ques `ig_map` 怎么没有办法 mod utils

* @ques 子目录应该如何处理

* @ques 怎么在 walk_path 中将 is_dir 变成 path_map 传进去

  - 这肯定要做成一个递归的...
  - 我想要做成一个一维数组...

* @ques rust path normalize ?

* @ques src/img_map 怎么找到 src/utils

* @ques 能不能将图片直接转化成字符串, 这样就不用组合图片了
  - 每一个图片就是一个字符串..., 所有图片组成一个文件...
  - 而且这可以通过 gzip 压缩到原来的 1/3...

## 2019-04-01 09:02:55

- [排列算法]
  - 矩形用对角线两个点代替
  - 矩形形成右下两个边界, 不停的添加边界会不断的延伸...
  - 边界位置就是新矩形的位置

## 2019-03-28 10:00:00

- @ques 怎么把 Str 转化为 String

- @ques 类型转化 真的很烦

  - 到底应该使用什么类型...
  - string or String
  - http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html

- @ques rls 怎么 format

  - 又工作了

- @ques box 应该如何使用...

- @ques 连接 socket 的时候要不要记录...

- @ques 如果我使用 &str ownership 我应该如何去控制...

* @ques 我这大脑好像一时出现很多东西, 无法一一的释放出来...

- @ques walk_path::run 我应该如何返回自己需要的类型...

  - 怎么把别人的错误包裹然后返回...

- @ques 我到底应不应该返回错误... 还是直接 panic

## 2019-03-27 15:38:14

- @ques 怎么将文件夹中的文件转化为&str

* @ques 怎么将 mod 放在一个目录中

- @todo 所有的文件需要一个结构保存

  - 文件夹:> 所有文件...

- @ques rust 怎么放置 test 文件

## 2019-03-27 14:50:27

- @ques 怎么遍历文件夹的所有图片

- @ques 怎么合并图片...
  - 怎么由乱起八糟的矩形 生成一个大的形状包括所有的形状

* @ques 做成 npm 模块...

- @ques rls 如何 format

* @ques rust cargo concat image

* 快速的合并图片...

* texturePacker rust 版本

- @todo
  - 遍历文件夹 找到所有的图片
  - 将文件转化为 bufferData
  - 将 bufferData 通过某种逻辑合并在一起
    - 生成相应的 json 数据
  - 生成 图片 + json 格式

* @ques 会不会网上已经存在这样的逻辑了
  - 组合矩形
  - https://zhidao.baidu.com/question/536689804.html
  - https://stackoverflow.com/questions/1213394/what-algorithm-can-be-used-for-packing-rectangles-of-different-sizes-into-the-sm

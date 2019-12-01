- mac `chmod +x ./script/genAtlas/combine_image`

## 测试命令

- test all bin
  cargo run --release D:\zsytssk\job\legend\legend_demo\dlc D:\zsytssk\job\legend\legend_demo\bin\resources atlas 2 2 resources/

- test bin
  cargo run --release D:\zsytssk\job\legend\legend_demo\dlc\npc\30001 D:\zsytssk\job\legend\legend_demo\bin\resources\npc\30001 atlas 2 2 resources/npc/30001/

* test desktop

cargo run --release C:\Users\zsyts\Desktop\test\dlc C:\Users\zsyts\Desktop\test\dist atlas 5 5 resources/

cargo run --release C:\Users\zhangshiyang\Desktop\test\dlc C:\Users\zhangshiyang\Desktop\test\dist atlas 5 5 resources/

- test local

cargo run --release ./test/src ./test/dist json 2 2 resources/

## 学习 rust

### 了解基础 核心思想

- ownerShip lifetime... + thread future

  - 记下在什么地方找到...

- future
  - stream tokio

### 常用知识

- http + TCP + socket ...

### 实践

- 示例 + 自己的实践

- 自己的应用...

## image::save_buffer

```rs
let (w, h) = buffer.dimensions();
let buffer = buffer.into_raw();
// let buffer = [..buffer];
image::save_buffer(
    &Path::new("./test/dist/test1.png"),
    &buffer,
    w,
    h,
    image::RGBA(8),
);
```

## 排列算法

- 所有的加在一起 宽 + 高

  - 如果宽大, 优先竖排 | 反之相反处理
  - 如果出现多余空间, 就用最小的往里填充...
  - 一直如此

- 取最大的

- @ques 我怎么知道会形成空白区域

- 不停的减少空白区域, 形成新的空白区域 先处理最大的空包区域

- 最大化空白区域...

- 将类式的集中处理...

- 如果是相同的如何处理...

  - 正方形 矩形...

- 搭配形成正方形...

- 这是一个典型的数学问题...

- 乐约数取整...

- 矩形

```js
w, h, a;

y = sqrt((a * w) / h);
x = sqrt((a * h) / w);
```

## other

https://crates.io/crates/image

```

```

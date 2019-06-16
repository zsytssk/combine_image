- learn rust thread pool

  - 固定 thread 的数量

- 内存的占用...

  - thread pool

- 全局变量

## 2019-06-15 11:43:18

- @ques 使用 trait 反而使我这代码变得复杂了

* rust 能不能 pub submodule struct

* function where 只能跟 trait 而不能 是 type alias

* rust 能不能 export as

* [generic associated types are unstable] https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
  - type alias within trait

```rs
pub trait TestTrait {
    type Item;
    type Info = Self::Item;
}

// export * from './...'
pub fn == submodel.xxxfun;
```

- @todo 像 raster 一样将所有的图片处理 全部放在一个函数中...
  - 所有的数字全部使用 i32

## 2019-06-14 10:37:48

- 将文件保存 看家里电脑有多快

- imageproc raster

  - image 15420
  - raster 13879 - 可能是类型转化的结果...
    - 能不能变成透明

- simd 到底是什么...

- rust get vector biggest

## 2019-06-12 21:09:52

- @todo 裁剪空白区域

  - 如果都是一样大, 裁剪都要一样大
  - 不然也需要裁剪

- @ques 还有什么优化方法

  - futures
  - 细节优化

  - https://crates.io/crates/state

- 本地 laya 的速度 vs wsl 上面的速度

- @ques 受伤的村名 npc 怎么跑到页面上面去了

  - 陆鑫 是不是做了特殊的处理...

- @ques 裁剪的功能 要不放在另外的函数中
  - 只有用到的功能才会计算

## 2019-06-12 10:19:09

- @todo 测试 login 文件能不能用

  - prefix

- @ques 顶级目录下文件出错

- @ques 将 exe 放在项目中间, 用 node 调用...

- @todo 能不能做一个类似 laya 发布的功能...

  - 这样会不会很快...
  - ignore file
  - 图片最大大小
  - 超过默认大小不打包...
  - ***
  - 需要读取 laya 的配置文件...

- 对比 git 文件找出修改文件...

- cargo run --release D:\zsytssk\job\legend\legend_demo\dlc C:\Users\zhangshiyang\Desktop\test\test json 5 5 test/

- @todo 能不能指定 thread 的数目

## 2019-06-11 17:39:58

- @ques 在 paths 下面 for loop 怎么样
  - 然后测试下 thread
- @ques rust path remove parent folder

- @todo 支持多进程

- @todo 怎么一级一级的创建文件夹

  - split("/") ...

- @ques 提交代码...

- @todo 设置间隔

- @todo 合成的图片能不能用..

- @todo 如何设置 global 变量

- @todo rust destructure vector

* 只读取 png 图片...

* 通过 git 读取, 支取转义修改文件

* RUST_BACKTRACE=1

* 报错如果直接在 src 下

## 2019-06-11 11:44:07

- @ques `ref h`

- @ques hash

  - 是 id 吗

- @ques lazy_static
- @ques writeln ??

- @ques blob file hahs data 到底是做什么的...

  - git commit 到底是怎么组织的...
  - 文件的内容是怎么保存的...

- @ques `parent.iter()s`

- @ques 能不能想什么 写什么 不会出错...

## 2019-05-21 09:15:13

- @ques rust edition...

- @todo 如何建立一个简单的 socket 连接...

  - 客户端+服务端...
  - https://github.com/tokio-rs/tokio/blob/master/tokio/examples/chat.rs

- @ques usize 怎么读, 为什么这么读...

- @ques RUST BOOK

* @ques 怎么能学好 rust
  - 了解基础 核心思想
    - ownerShip lifetime... + thread future
  - 常用知识
    - http + net + socket ...
  - 示例 + 自己的实践
  - 自己的应用...

## 2019-05-16 09:00:51

- [Error Handling in Rust](https://dev.to/saiumesh/error-handling-in-rust-programming-language-3g4)

* @ques generic type

* @ques generic type

## 2019-05-13 09:14:48

- [Nicer error handling](https://www.snoyman.com/blog/2018/12/rust-crash-course-07-async-futures-tokio)
- [wasm-with-rust](https://medium.com/tech-lah/webassembly-part-ii-a-wasm-with-rust-2356dbc6526e)

## 2019-05-08 08:58:36

- @ques future 真的很麻烦, 我怎么也得执行两次 future 把
  - 怎么把结果返回去??

* @note and_then 和 map 的区别
  - map 返回前一个 future
  - and_then 返回自己的 future..

- @ques `tokio::spawn` `future::poll_fn` 这两个是做什么的???
- @ques `future::Either` 是做什么的

- @ques loop 一个函数执行了两次怎么处理...

- @ques 在哪里可以看到 future 的核心解释 而不是一个个的例子
  - 不会核心的内容 看再多的例子也是雾里看花

## 2019-05-05 09:50:38

- 学习打字都十分的额迟钝...

  - @ques 将一切都变成感觉...
  - 让感觉带动行为...
  - ...

- @ques 怎么用 rust 建立一个 http 请求...

  - tcp -- http 有什么关系..

- @ques `b"str"` 是什么意思

  - 将字符串 变成 buffer 吗

- @todo [感情] @zeng

- @note future 如何使用

  - 在独立的地方 一个个小任务变成 future
  - 然后在一个地方 使用 tokio 引用

## 2019-05-05 08:55:06

- @ques 好像不能像 js 一样将一个任务分配出去 做成异步再拿到结果...

  - tokio future 不适合做这个??

- @ques rust std future 怎么 x

- @note try_ready!

- @todo env::args().collect();

- @todo rust split vec

## 2019-04-30 08:51:57

- @ques 除了 and_then 之外 future 还有什么方法

* @ques 怎么结束 future

* @ques 怎么结束 stream

* @ques play IBM 第一台量子计算机

* @note rust 的 future 逻辑很绕, 不容易理解
  - 可能是内部实现的方案
  - 难道我还要理解 futures 里面的逻辑吗
  - 也许真正的 std::futures 不会这样...
  - 而且怎么复杂只是为了建立一个 future...

- @ques MyOk 为什么执行两次

```rs

struct MyOk<T>(Option<T>, u32);
impl<T> MyOk<T> {
    fn new(t: T) -> MyOk<T> {
        MyOk(Some(t), 0)
    }
}

impl<T> Future for MyOk<T> {
    type Item = T;
    type Error = ();
    fn poll(&mut self) -> Poll<T, ()> {
        let state = self.1;
        println!("MyOk state  = {}", self.1);
        if state == 0 {
            self.1 = 1;
            let task = futures::task::current();
            task.clone().notify();
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(self.0.take().unwrap()))
        }
    }
}

fn main() {
    let name = String::from("Alice");
    let future = MyOk::new(name).and_then(|name| {
        println!("Name: {}", name);
        MyOk::new(())
    });
    println!("test");
    tokio::run(future);
}

```

## 2019-04-24 10:01:53

- @ques atomic 是做什么的
- @ques Arc 是做什么的
  - 直接跨 thread 多个 mutation ref
- @ques `thread::{sleep, spawn}` 怎么用..

## 2019-04-24 09:29:23

- @todo rust image ImageFormat

- @learn future tokio...

- @ques 这样写的函数到底有没有用...

  - 怎么避免 future 之间相互堵塞..
  - 除非返回 future, 在调用的地方 promiseAll
  - 能不能像 promise 一样 `return new future(resolve, reject...)`

- @ques task done ??

* @ques image 能直接 load buffer 吗

* @ques 能 tokio future 转换为 seek 吗

* @ques 怎么把异步的东西取出来...

* @ques 异步 读取文件 + 保存文件

* @ques 采集谁做... @张道青

## 2019-04-22 09:07:15

- @ques 监听 内存 cpu 消耗

- @ques thread 更高级的操作...

  - 执行函数...
  - 共享数据...

- @ques combine_img 更适合做这个...

* thread 可以将速度提升一倍

  - 如何测试 release 的速度
  - 接近一倍

* 异步函数怎么样呢 tokio
  - tokio file io

## 2019-04-18 09:01:07

- @ques 如何将任务分配到多进程

  - 将文件读取 + 写入 分配到额外的进程中

- @ques 多进程的代码如何组织

  - 最理想的是将文件读取和写入从主进程中分离

- @ques 我如何将 buffer 传递给其他进程

  - 进程之间交互是否只能是字符串...
  - buffer 转化为字符串...
  - mpsc only can pass string?

- @note 我要切一个分支做这个...

- @ques 多进程会导致设计变复杂

  - 需要多个文件夹测试效果...

- @ques 内存会不会占用太多 直接崩溃

  - 能不能处理这个问题...

- @ques 需要一个缓存池

  - 当前还有数据正在处理

- @ques jpg 能不能和 png 打包在一起

- @ques 如果写入文件都在一个进程那么也会导致堵塞
  - 能不能分到多个进程
  - 进程是为了处理 cpu 性能
  - 这 io 瓶颈好像没有什么意义

## 2019-04-17 09:17:21

- @note 发布还差什么...

* @todo 写入 atlas.json

* @todo release 之后会不会变的更快...

  - 太快了...

* @todo combine 优化

- @note release api :> 源文件 目标文件...

* @note `cargo run --release`

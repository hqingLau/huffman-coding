【toy】哈夫曼编解码文件

`github`地址：[链接](https://github.com/hqingLau/huffman-coding)

> 不应该叫解压压缩，应该叫编解码，当字符种类较多的时候，有些字符需要很长的码表示，压缩效果并不好。

测试：

<img src="https://orzlinux.cn/img/2022-12-fa7e2dba76ad0d34.png" alt="image-20221223111747576" style="zoom:50%;" />

## 整体方案

- 以二进制方式读取整个文件，每8位将其视为`u8`类型的字符，直至最后一位。
- 构建哈夫曼树，得到`字符-编码`映射表。
- 将映射表长度及映射表写入将要生成的压缩文件。即再次读取整个文件，每8位进行一次映射，将映射后的新字符写入编码文件（每满8位写一次）。
- 解压，根据文件格式，读取`字符-编码`映射表，和压缩数据，映射后写入新解压的文件。

`main` 函数代码：

```rust
fn main() {
    // 要压缩的文件名
    let filename = "./bbe.txt";

    // 统计u8频率
    let u8_count_map = u8_count::get_u8_count_map(filename);
    if let Err(e) =  u8_count_map {
        println!("统计u8失败：{:?}", e);
        return;
    }

    let u8_count_map = u8_count_map.unwrap();

    // 构建Huffman Tree
    let hf_tree = huffman_tree::HuffmanNode::build(&u8_count_map);

    // 根据Huffman Tree和源文件生成压缩文件
    let newfilename = zipfile::huffman_zip(hf_tree, filename);
    match newfilename {
        Ok(newfilename) => {
            println!("压缩成功，压缩文件：{}", newfilename);
        },
        Err(e) => {
            println!("压缩失败：{}", e.to_owned())
        }
    }

    // 要解压的文件名

    let unzipfilename = filename.to_string() + ".huuuf";
    let afterzip_filename = unzipfile::huffman_unzip(&unzipfilename);
    match afterzip_filename {
        Ok(afterzip_filename) => {
            println!("解压成功，生成文件：{}", afterzip_filename);
        },
        Err(e) => {
            println!("解压失败：{}", e.to_owned())
        }
    }
}
```





## 统计字符频率

这个是读取一次文件，每8位视为一个`u8`类型，统计一次频率。结果放入一个`map`中。

详见`u8_count.rs`

## 构建哈夫曼树

定义结构体：

```rust
pub struct  HuffmanTree {
    pub root: Rc<RefCell<HuffmanNode>>,
    pub u8_path_map: HashMap<u8,Vec<bool>>,
}
pub struct HuffmanNode {
    is_leaf: bool,
    u8_data: u8,       // u8字符
    path: Vec<bool>,   // 字符对应的路径
    left_child: Option<Rc<RefCell<HuffmanNode>>>,
    right_child: Option<Rc<RefCell<HuffmanNode>>>,
}
```

之后按照Huffman构建的常规思路进行构建，构建好之后通过`DFS`遍历，并记录路径，生成`u8`字符和路径的对应表。

详见`huffman_tree.rs`

## 将映射表长度及映射表写入将要生成的压缩文件

这个就是要自定义文件格式，将`u8`字符和路径的对应表、压缩数据、零填充（因为压缩数据不一定正好是8倍数）写入新文件。

详见`zipfile.rs`

我定义的文件格式是：

```shell
// 自定义一下文件格式
// | 压缩数据0填充个数 |  u8_path_map的大小x | u8_path_map  | 压缩数据 |
// |    3bit          |   13bit            |    x个bit    |      --  |
```

这个还涉及到map的序列化，详见`stringify.rs`

## 解压

根据文件格式读取，映射，写入新文件。

详见`unzipfile.rs`

## 总结

作为练手还是不错的。
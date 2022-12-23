【toy】哈夫曼编解码文件

## 整体方案

- 以二进制方式读取整个文件，每8位将其视为u8类型的字符，直至最后一位。
- 构建哈夫曼树，得到`字符-编码`映射表。
- 将映射表长度及映射表写入将要生成的压缩文件。
- 再次读取整个文件，每8位进行一次映射，将映射后的新字符写入压缩文件（每满8位写一次）。

> 针对纯文本效果并不佳，例如52个英文字母，最长的需要51位来表示，本来一个u8的事，需要6-7个u8，其他地方压缩了也不一定能弥补这个增加。
> 
> 对于部分文本有效。
> 
> 针对图片之类的二进制文件更是完全没用，原因同上。
>
> 大概需要优化什么地方，或者适用场景受限，学习Rust练习之用，暂且不管。

## 细节

### 统计字符频率

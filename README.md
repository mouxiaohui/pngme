# PNG 文件隐写术

参考: https://picklenerd.github.io/pngme_book/introduction.html

## 使用

```shell
pngme [图标路径] [Key(四个字节)] [内容] [输出路径(可选)]

# 存入内容
pngme encode ./img/test.png Rust 你好世界！
# 读取内容
pngme decode ./img/test.png Rust
# 移除内容
pngme remove ./img/test.png Rust
# 打印图片中所有文本内容
pngme print ./img/test.png
```

## PNG 文件结构

### 文件头

PNG 文件头位置总是由位固定的字节来描述的

- 十进制数: 137 80 78 71 13 10 26 10
- 十六进制数: 89 50 4E 47 0D 0A 1A 0A

### 数据块

去掉了 png 图片等前 8 个字节，剩下的就是存放 png 数据的数据块，我们通常称之为 chunk。

数据块格式
|描述|长度|说明|
|---|---|---|
|数据块内容长度|4 字节|指定数据块中数据域的长度，其长度不超过(231－1)字节|
|数据块类型|4 字节|数据块类型码由 ASCII 字母(A-Z 和 a-z)组成|
|数据块内容|不定字节|存储按照 Chunk Type Code 指定的数据|
|crc 冗余校验码|4 字节|存储用来检测是否有错误的循环冗余码|

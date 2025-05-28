# create_placeholder_image

创建占位图片

```
占位图生成工具 by angcyo

Usage: create_placeholder_image [OPTIONS]

Options:
  -c, --color <COLOR>            图片颜色, 16进制RGBA色值 "#f6f6f6ff"
      --text-color <TEXT_COLOR>  文本的颜色,16进制RGBA色值 "#000000ff" [default: #000000ff]
      --width <WIDTH>            生成的图片宽度 [default: 100]
      --height <HEIGHT>          生成的图片高度 [default: 100]
  -i, --input <INPUT>            输入图片路径, 则可以对应输出指定大小的图片数据
  -o, --output <OUTPUT>          输出文件路径, 不指定则输出base64数据到控制台
  -h, --help                     Print help
  -V, --version                  Print version
```
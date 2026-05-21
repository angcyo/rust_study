# 压缩算法测试

| 算法名称               | 算法流派       | G-code 预期压缩后体积 | 下位机解压所需 RAM    | 推荐指数 (针对单片机)         |
|--------------------|------------|----------------|----------------|----------------------|
| PAQ8 / ZPAQ        | 上下文混合 (AI) | 约 15%          | 无法运行 (> 100MB) | ❌                    |
| LZMA (7-Zip)       | 字典 + 马尔可夫  | 约 20%          | 16 KB - 64 KB  | ⭐⭐⭐ (需评估芯片RAM)       |
| Zstd (Level 22)    | FSE 熵编码    | 约 22%          | ~ 100 KB       | ❌                    |
| Deflate (zlib/ZIP) | LZ77 + 哈夫曼 | 约 35%          | ~ 4 KB - 8 KB  | ⭐⭐⭐⭐ (有 uzlib/miniz) |
| LZ4                | LZ77 (极速)  | 约 50%          | 0 KB (纯流式)     | ⭐⭐⭐⭐⭐                |

## LZ4

由 Yann Collet 开发，是目前公认的“速度之王”，在保证极速解压的同时兼顾了优秀的压缩率。

官方 GitHub 仓库：https://github.com/lz4/lz4

开源协议：BSD 2-Clause (解压库), GPLv2 (命令行工具) —— 对于固件和 App 集成，使用其解压库是完全商业友好的。

## FastLZ

由 Ariya Hidayat 开发，主打极致的轻量化和嵌入式友好性。

官方 GitHub 仓库：https://github.com/ariya/FastLZ

开源协议：MIT License —— 极其宽松，适合任意商业项目。

## LZMA (7-Zip 核心) 源码

LZMA 的官方发布渠道通常是以 LZMA SDK (Software Development Kit) 的形式提供，由 7-Zip 的作者 Igor Pavlov 亲自维护。

官方网站与下载地址：https://www.7-zip.org/sdk.html

SourceForge 官方仓库：https://sourceforge.net/projects/sevenzip/

开源协议：公有领域 (Public Domain) / LGPL —— 没有任何商业限制，连版权声明都不强制要求。

## Deflate (zlib / ZIP 核心) 源码

Deflate 最权威、应用最广的 C 语言实现是 zlib，由 Jean-loup Gailly 和 Mark Adler 共同开发。

官方 GitHub 仓库：https://github.com/madler/zlib

官方网站：https://zlib.net/

开源协议：zlib License —— 极其宽松的商业友好协议。
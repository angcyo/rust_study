@echo off
rem 设置当前控制台为UTF-8编码
chcp 65001 >> nul

rem  使用默认平台编译 `rustup show active-toolchain`
rem  stable-x86_64-unknown-linux-gnu (default)
rem  输出目录在 ./release

cargo build --release --package create_placeholder_image
rem cargo build --release --target x86_64-unknown-linux-gnu --package create_placeholder_image
rem cargo build --release --target x86_64-unknown-linux-musl --package create_placeholder_image

rustup show active-toolchain

cmd /C start ..\target\release
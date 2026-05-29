@echo off
rem 设置当前控制台为UTF-8编码
chcp 65001 >> nul

cargo build --release --package resize_image_cli
rustup show active-toolchain
:: explorer ../target/release
@echo off
rem 设置当前控制台为UTF-8编码
chcp 65001 >> nul

git fetch
git rebase origin/main

set "folder=rust_crates"

if exist "%CD%\%folder%\" (
    cd %folder%
    echo 准备拉取仓库：%folder%
    git fetch
    git rebase origin/main
    cd ..
) else (
    echo 准备克隆仓库：%folder%
    git clone "git@github.com:angcyo/%folder%.git"
)
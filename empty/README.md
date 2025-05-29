# 2025-05-29

| 空项目   | 大小     |
|-------|--------|
| 不开启压缩 | ≈418KB |
| 开启压缩  | ≈286KB |
| 压缩后   | ≈286KB |

# 压缩

```toml
[profile.release]
#https://github.com/johnthagen/min-sized-rust
strip = true  # Automatically strip symbols from the binary. 4~mb
opt-level = "z"  # Optimize for size.
lto = true  # Perform link-time optimizations.
codegen-units = 1  # Compile the whole crate at once.
panic = "unwind"  # Abort on panic. 20~kb # unwind
```
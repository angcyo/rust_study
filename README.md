[TOC]

# crates.io: `Rust Package Registry`

https://crates.io/

## 网络相关

### reqwest `v0.12.15`

https://crates.io/crates/reqwest

higher level HTTP client library

https://docs.rs/reqwest/0.12.15/reqwest/

```toml
[package]
name = "untitled1"
version = "0.1.0"
edition = "2024"

[dependencies]
# https://crates.io/crates/reqwest
reqwest = "0.12.15"
# https://crates.io/crates/tokio
tokio = { version = "1.45.0", features = ["full"] }
# https://crates.io/crates/html2md
html2md = "0.2.15"
```

```rust
#[tokio::main]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
    Ok(())
}

//--

async fn get_url_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    //println!("body = {body:?}");
    Ok(body)
}
```

### http `v1.3.1`

https://crates.io/crates/http

A set of types for representing HTTP requests and responses.

### tower `v0.5.2`

https://crates.io/crates/tower

Tower is a library of modular and reusable components for building robust clients and servers.

### axum `v0.8.4`

https://crates.io/crates/axum

Web framework that focuses on ergonomics and modularity

### actix-web `v4.11.0`

https://crates.io/crates/actix-web

Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust

### warp `v0.3.7`

https://crates.io/crates/warp

serve the web at warp speeds.

### rocket `v0.5.1`

https://crates.io/crates/rocket

https://rocket.rs/

Web framework with a focus on usability, security, extensibility, and speed.

### tide `v0.16.0`

https://crates.io/crates/tide

A minimal and pragmatic Rust web application framework built for rapid development

## 工具相关

### tokio `v1.45.0`

https://crates.io/crates/tokio

https://tokio.rs/

An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.

```rust
#[tokio::main]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
    Ok(())
}

//--

async fn get_url_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    //println!("body = {body:?}");
    Ok(body)
}
```

### url `v2.5.4`

https://crates.io/crates/url

URL library for Rust, based on the WHATWG URL Standard

### html2md `v0.2.15`

https://crates.io/crates/html2md

https://docs.rs/html2md/0.2.15/html2md/

Library and binary to convert simple html documents into markdown

```toml
[package]
name = "untitled1"
version = "0.1.0"
edition = "2024"

[dependencies]
# https://crates.io/crates/reqwest
reqwest = "0.12.15"
# https://crates.io/crates/tokio
tokio = { version = "1.45.0", features = ["full"] }
# https://crates.io/crates/html2md
html2md = "0.2.15"
```

```rust
#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //test_macro();

    let url = "https://www.rust-lang.org";
    let output = ".output/rust.md";

    let body = get_url_text(url).await?;

    // Create output directory
    let output_dir = std::path::Path::new(output).parent().unwrap();
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    std::fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
    Ok(())
}
```

## Others


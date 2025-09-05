[TOC]

# Rust

https://www.rust-lang.org/zh-CN/

## 使用 Rustup 安装 Rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- `~/.cargo/bin` : 所有工具安装目录
- `rustc`        : 编译器
- `cargo`        : 包管理工具
- `rustup`       : 升级工具/管理工具链 `rustup target`

## 更新 Rust

`rustup update`

## 卸载 Rust

`rustup self uninstall`

# crates.io: `Rust Package Registry`

https://crates.io/

## 网络相关

### reqwest `v0.12.15` 网络请求

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

### http `v1.3.1` 网络请求

https://crates.io/crates/http

A set of types for representing HTTP requests and responses.

```rust
//Create an HTTP request:

use http::Request;

fn main() {
    let request = Request::builder()
      .uri("https://www.rust-lang.org/")
      .header("User-Agent", "awesome/1.0")
      .body(())
      .unwrap();
}

//--

//Create an HTTP response:

use http::{Response, StatusCode};

fn main() {
    let response = Response::builder()
      .status(StatusCode::MOVED_PERMANENTLY)
      .header("Location", "https://www.rust-lang.org/install.html")
      .body(())
      .unwrap();
}

```

### tower-http `v0.6.4`

https://crates.io/crates/tower-http

https://docs.rs/tower-http/latest/tower_http/

Tower middleware and utilities for HTTP clients and servers

```rust
use tower_http::{
    decompression::DecompressionLayer,
    set_header::SetRequestHeaderLayer,
    trace::TraceLayer,
    classify::StatusInRangeAsFailures,
};
use tower::{ServiceBuilder, Service, ServiceExt};
use hyper_util::{rt::TokioExecutor, client::legacy::Client};
use http_body_util::Full;
use bytes::Bytes;
use http::{Request, HeaderValue, header::USER_AGENT};

#[tokio::main]
async fn main() {
let client = Client::builder(TokioExecutor::new()).build_http();
    let mut client = ServiceBuilder::new()
        // Add tracing and consider server errors and client
        // errors as failures.
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier()
        ))
        // Set a `User-Agent` header on all requests.
        .layer(SetRequestHeaderLayer::overriding(
            USER_AGENT,
            HeaderValue::from_static("tower-http demo")
        ))
        // Decompress response bodies
        .layer(DecompressionLayer::new())
        // Wrap a `Client` in our middleware stack.
        // This is possible because `Client` implements
        // `tower::Service`.
        .service(client);

    // Make a request
    let request = Request::builder()
        .uri("http://example.com")
        .body(Full::<Bytes>::default())
        .unwrap();

    let response = client
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
}
```

## web 服务器

### tower `v0.5.2` clients and servers

https://crates.io/crates/tower

Tower is a library of modular and reusable components for building robust clients and servers.

### axum `v0.8.4` Web framework

https://crates.io/crates/axum

Web framework that focuses on ergonomics and modularity

```rust
#[allow(dead_code)]
pub async fn start_serve() {
    // initialize tracing
    tracing_subscriber::fmt::try_init().ok();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", any(root))
        // `POST /json` goes to `create_user`
        .route("/json", post(post_json));

    // run our app with hyper, listening globally on port 9292
    let addr = "0.0.0.0:9292";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
#[allow(dead_code)]
async fn root() -> String {
    format!("Hello, Axum! {}", now_date_time())
}
#[allow(dead_code)]
async fn post_json(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<HashMap<String, serde_json::Value>>) {
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    let mut result: HashMap<String, serde_json::Value> = HashMap::new();
    result.extend(payload);
    result.insert(
        "reply".to_string(),
        serde_json::Value::String(now_date_time()),
    );
    (StatusCode::CREATED, Json(result))
}

```

#### serde `v1.0.219`

https://crates.io/crates/serde

A generic serialization/deserialization framework

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
```

#### serde_json `v1.0.140`

https://crates.io/crates/serde_json

A JSON serialization file format

```rust
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}

//--

let p: Person = serde_json::from_str(data)?;

// Serialize it to a JSON string.
let j = serde_json::to_string(&address)?;

//--

serde_json::Value::String(now_date_time());

let mut result: HashMap<String, serde_json::Value> = HashMap::new();

```

#### tracing `v0.1.41`

https://crates.io/crates/tracing

Application-level tracing for Rust.

#### tracing-subscriber `v0.3.19`

https://crates.io/crates/tracing-subscriber

Utilities for implementing and composing `tracing` subscribers.

```rust
// initialize tracing
tracing_subscriber::fmt::init();
```

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

### tokio `v1.45.0` 异步

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

### clap `v4.5.38` 命令行解析

https://crates.io/crates/clap

https://docs.rs/clap/latest/clap/

A simple to use, efficient, and full-featured Command Line Argument Parser

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

### colored `v3.0.0` 终端输出颜色

https://crates.io/crates/colored

The most simple way to add colors in your terminal

```rust
extern crate colored; // not needed in Rust 2018+

use colored::Colorize;

// test the example with `cargo run --example most_simple`
fn main() {
    // TADAA!
    println!("{} {} !", "it".green(), "works".blue().bold());
}
```

### jsonxf `v1.1.1` json格式化

https://crates.io/crates/jsonxf

A fast JSON pretty-printer and minimizer.

```rust
extern crate jsonxf;
let ugly_json = "{\"hello\":\"world\"}";
let pretty_json = jsonxf::pretty_print(ugly_json).unwrap();
assert_eq!(pretty_json, "{\n  \"hello\": \"world\"\n}\n");
````

### mime `v0.3.17`

https://crates.io/crates/mime

Strongly Typed Mimes

```rust
extern crate mime;

// common types are constants
let text = mime::TEXT_PLAIN;

// deconstruct Mimes to match on them
match (text.type_(), text.subtype()) {
    (mime::TEXT, mime::PLAIN) => {
        // plain text!
    },
    (mime::TEXT, _) => {
        // structured text!
    },
    _ => {
        // not text!
    }
}
```

### chrono `v0.4.41` 日期和时间

https://crates.io/crates/chrono

https://docs.rs/chrono/latest/chrono/

Date and time library for Rust

```rust
use chrono::prelude::*;

let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`

//--

use chrono::prelude::*;

let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

```

### anyhow `v1.0.98` 错误处理

https://crates.io/crates/anyhow

```rust
use anyhow::Result;

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
```

### log `v0.4.27` logging

https://crates.io/crates/log
https://docs.rs/log/latest/log/

A lightweight logging facade for Rust

#### env_logger `v0.11.8`

https://crates.io/crates/env_logger

A logging implementation for `log` which is configured via an environment variable.

```rust
use log::info;

fn main() {
    env_logger::init();

    info!("starting up");

    // ...
}
```

#### colog `v1.3.0`

https://crates.io/crates/colog

The `colog` library is a simple formatter backend for the standard rust logging system (in the `log` crate).

```rust
// Quick start: use default initialization
colog::init();

log::error!("error message");
log::error!("error with fmt: {}", 42);
log::warn!("warn message");
log::info!("info message");
log::debug!("debug message");
log::trace!("trace message");

//--

[E] error message
[E] error with fmt: 42
[W] warn message
[*] info message

```

#### simple_logger `v5.0.0`

https://crates.io/crates/simple_logger

A logger that prints all messages with a readable output format

```rust
SimpleLogger::new().init().unwrap();

log::error!("error message");
log::error!("error with fmt: {}", 42);
log::warn!("warn message");
log::info!("info message");
log::debug!("debug message");
log::trace!("trace message");

//--

2025-05-23T03:26:21.648Z ERROR [rust_study::utils] error message
2025-05-23T03:26:21.648Z ERROR [rust_study::utils] error with fmt: 42
2025-05-23T03:26:21.648Z WARN  [rust_study::utils] warn message
2025-05-23T03:26:21.648Z INFO  [rust_study::utils] info message
2025-05-23T03:26:21.648Z DEBUG [rust_study::utils] debug message
2025-05-23T03:26:21.648Z TRACE [rust_study::utils] trace message

```

### base64 `v0.22.1`

https://crates.io/crates/base64

encodes and decodes base64 as bytes or utf8

```rust
/// 将字节数组进行base64加密
#[allow(dead_code)]
pub fn base64_encode(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

/// 将base64的字符串进行解密
#[allow(dead_code)]
pub fn base64_decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode(s)
}
```

### md5 `v0.7.0`

https://crates.io/crates/md5

The package provides the MD5 hash function.

```rust
/// 将字节数组进行md5加密
#[allow(dead_code)]
pub fn md5_encode(bytes: &[u8]) -> String {
    format!("{:X}", md5::compute(bytes))
}
```

### uuid `v1.17.0`

https://crates.io/crates/uuid

A library to generate and parse UUIDs.

```rust
/// 生成一个uuid
#[allow(dead_code)]
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string().to_uppercase().replace("-", "")
}
```

### rand `v0.9.1` 随机数

https://crates.io/crates/rand

Random number generators and other randomness functionality.

```rust
/// 随机生成一个浮点数
#[allow(dead_code)]
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

/// 在一个范围内随机
/// `random_range(0..100)`
#[allow(dead_code)]
pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = rand::rng();
    rng.random_range(range)
}
```

### bytes `v1.10.1`

https://crates.io/crates/bytes

Types and traits for working with bytes

```rust
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"hello world"[..]);
buf.put_u16(1234);

let a = buf.split();
assert_eq!(a, b"hello world\x04\xD2"[..]);

buf.put(&b"goodbye world"[..]);

let b = buf.split();
assert_eq!(b, b"goodbye world"[..]);

assert_eq!(buf.capacity(), 998);
```

### pathos `v0.3.0`

https://crates.io/crates/pathos

A natural API for handling OS-specific user or system directories, including iOS and Android.

### cargo_metadata `v0.19.2`

https://crates.io/crates/cargo_metadata

structured access to the output of `cargo metadata`

### is-terminal `v0.4.16`

https://crates.io/crates/is-terminal

Test whether a given stream is a terminal

```rust
use is_terminal::IsTerminal;

fn main() {
    if std::io::stdout().is_terminal() {
        println!("Stdout is a terminal");
    } else {
        println!("Stdout is not a terminal");
    }
}
```

### termcolor `v1.4.1`

https://crates.io/crates/termcolor

A simple cross platform library for writing colored text to a terminal.

```rust
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn write_green() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "green text!")
}
```

### pdfium-render `v0.8.31`

https://crates.io/crates/pdfium-render

A high-level idiomatic Rust wrapper around Pdfium, the C++ PDF library used by the Google Chromium project.

> 需要额外链接`Pdfium`库.

```rust
use pdfium_render::prelude::*;

fn export_pdf_to_jpegs(path: &impl AsRef<Path>, password: Option<&str>) -> Result<(), PdfiumError> {
    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable.
    // See the "Dynamic linking" section below.

    let pdfium = Pdfium::default();

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // ... then render each page to a bitmap image, saving each image to a JPEG file.

    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .into_rgb8() // ... then converts it to an image::Image...
            .save_with_format(
                format!("test-page-{}.jpg", index), 
                image::ImageFormat::Jpeg
            ) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}
```

### pdf2image `v0.1.3`

https://crates.io/crates/pdf2image

A simplified port of Python's pdf2image that wraps pdftoppm and pdftocairo to convert PDFs into images.

> 需要额外安装`Poppler`.
> `brew install poppler`

```rust
use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};

fn main() -> Result<(), PDF2ImageError> {
    let pdf = PDF::from_file("examples/pdfs/ropes.pdf").unwrap();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().pdftocairo(true).build()?,
    )?;

    std::fs::create_dir("examples/out").unwrap();
    for (i, page) in pages.iter().enumerate() {
        page.save_with_format(format!("examples/out/{}.jpg", i + 1), image::ImageFormat::Jpeg)?;
    }

    Ok(())
}
```

### libc `v0.2.172`

https://crates.io/crates/libc

Raw FFI bindings to platform libraries like libc.


## smtp `Simple Mail Transfer Protocol`

邮箱客户端

- `SMTP` : Simple Mail Transfer Protocol; 默认端口:`25`。`starttls`端口:`587`。`SSL`端口 :`465`。
- `LMTP` : Local Mail Transfer Protocol
- `IMAP` :（Internet Message Access Protocol）; 默认端口:`143`。`SSL/TLS`端口 :`993`。
- `POP3` :（Post Office Protocol 3）默认端口:`110`。`SSL/TLS`端口 :`995`。

- [How to send yourself email notifications from a Dart server](https://suragch.medium.com/how-to-send-yourself-email-notifications-from-a-dart-server-a7c16a1900d6)
- [SMTP vs IMAP vs POP3 - Knowing The Difference](https://www.jscape.com/blog/smtp-vs-imap-vs-pop3-difference)

### async-smtp `v0.10.2`

https://crates.io/crates/async-smtp

SMTP client

### lettre `v0.11.16`

https://crates.io/crates/lettre

Email client

```rust
pub async fn send_mail_lettre(
    from: (String, String),
    to: (String, String),
    title: &str,
    html_body: &str,
    text_body: &str,
    //--
    host: (&str, u16),
    credentials: (&str, &str),
) {
    let email = Message::builder()
        .from(format!("{} <{}>", from.0, from.1).parse().unwrap())
        //.reply_to(format!("{} <{}>", from.0, from.1).parse().unwrap())
        .to(format!("{} <{}>", to.0, to.1).parse().unwrap())
        .subject(title)
        .header(ContentType::TEXT_HTML)
        .body(if html_body.is_empty() {
            text_body.to_string()
        } else {
            html_body.to_string()
        })
        .unwrap();

    let creds = Credentials::new(credentials.0.to_owned(), credentials.1.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(host.0)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).unwrap();
}

//--

#[allow(dead_code)]
async fn test_send_mail() {
    mail::send_mail_lettre(
        ("no-reply".to_string(), "no-reply@laserabc.com".to_string()),
        ("angcyo".to_string(), "angcyo@126.com".to_string()),
        format!("title - {}", utils::now_timestamp()).as_str(),
        format!("<h1>Hello, body. <sup>html</sup>! {}</h1>", utils::now_date_time()).as_str(),
        format!("Hello body. <sup>text</sup>! {}", utils::now_date_time()).as_str(),
        ("smtp.feishu.cn", 465),
        ("no-reply@laserabc.com", ""),
    )
    .await;
}
```

### mail-send `v0.5.1`

https://crates.io/crates/mail-send

E-mail delivery library with SMTP and DKIM support

```rust
pub async fn send_mail(
    from: (String, String),
    to: (String, String),
    title: &str,
    html_body: &str,
    text_body: &str,
    //--
    host: (&str, u16),
    credentials: (&str, &str),
) {
    // Build a simple multipart message
    let message = MessageBuilder::new()
        .from(from)
        .to(to)
        .subject(title)
        .html_body(html_body)
        .text_body(text_body);

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new(host.0, host.1)
        .implicit_tls(false) //starttls or ssl
        .credentials(credentials)
        .timeout(std::time::Duration::from_secs(5))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
```

## Others

### lazy_static `v1.5.0`

https://crates.io/crates/lazy_static

A macro for declaring lazily evaluated statics in Rust.

```rust
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

fn main() {
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}
```

### image `v0.25.6`

https://crates.io/crates/image

Imaging library. Provides basic image processing and encoders/decoders for common image formats.

```rust
ImageReader::open(image_file_path)?.decode()
    .unwrap()
    .grayscale() // 灰度处理
    .save(".output/FaceQ_output.png")
    .unwrap();
```

### imageproc `v0.25.0`

https://crates.io/crates/imageproc

Image processing operations

### photon-rs `v0.3.3`

https://crates.io/crates/photon-rs

High-performance image processing library for native use and the web

```rust
extern crate photon_rs;
use photon_rs::native::{open_image, save_image};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("test_image.png");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "raw_image.jpg");    
}
```

### lru `v0.14.0`

https://crates.io/crates/lru

A LRU cache implementation

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    cache.put("apple", 3);
    cache.put("banana", 2);

    assert_eq!(*cache.get(&"apple").unwrap(), 3);
    assert_eq!(*cache.get(&"banana").unwrap(), 2);
    assert!(cache.get(&"pear").is_none());

    assert_eq!(cache.put("banana", 4), Some(2));
    assert_eq!(cache.put("pear", 5), None);

    assert_eq!(*cache.get(&"pear").unwrap(), 5);
    assert_eq!(*cache.get(&"banana").unwrap(), 4);
    assert!(cache.get(&"apple").is_none());

    {
        let v = cache.get_mut(&"banana").unwrap();
        *v = 6;
    }

    assert_eq!(*cache.get(&"banana").unwrap(), 6);
}
```

### prost `v0.13.5` Protocol Buffers

https://crates.io/crates/prost

https://protobuf.dev/

A Protocol Buffers implementation for the Rust Language.

```proto
syntax = "proto3";

package shirt;

// A snazzy new shirt!
message Shirt {
  // Label sizes
  enum Size {
    SMALL = 0;
    MEDIUM = 1;
    LARGE = 2;
  }

  // The base color
  string color = 1;
  // The size as stated on the label
  Size size = 2;
}
```

```rust
#[allow(dead_code)]
pub fn test_protocol() {
    let shirt = shirt::Shirt {
        color: "blue".to_string(),
        size: 2,
    };
    // 将 Shirt 序列化为字节数组
    let mut buf = Vec::new();
    // let mut buf = vec![];
    shirt.encode(&mut buf).unwrap();

    crate::ptl!("Encoded shirt: {:?}", crate::utils::base64_encode(&buf));

    // 从字节数组反序列化为 Shirt 实例
    let decoded_shirt = shirt::Shirt::decode(&buf[..]).unwrap();

    // 输出反序列化出来的内容
    println!("Decoded shirt: {:?}", decoded_shirt);
}
```

#### protoc 安装

https://protobuf.dev/installation/

##### MacOS

```
brew install protobuf
protoc --version  # Ensure compiler version is 3+
```

#### Windows

```
> winget install protobuf
> protoc --version # Ensure compiler version is 3+
```

##### Linux

```
apt install -y protobuf-compiler
protoc --version  # Ensure compiler version is 3+
```

#### prost-build `v0.13.5`

https://crates.io/crates/prost-build

Generate Prost annotated Rust types from Protocol Buffers files.

```rust
// build.rs
fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .out_dir("src/protocol")
        .compile_protos(&["src/protocol/shirt.proto"], &["src/"])?;
    //prost_build::compile_protos(&["src/protocol/items.proto"], &["src/"])?;
    Ok(())
}
```

## 第三方

### jni `v0.21.1`

https://crates.io/crates/jni

Rust bindings to the JNI

### safer-ffi `v0.1.13`

https://crates.io/crates/safer-ffi

Write safer FFI code in Rust without polluting it with unsafe code

### maturin `v1.8.6`

https://crates.io/crates/maturin

Build and publish crates with pyo3, cffi and uniffi bindings as well as rust binaries as python packages

### pyo3 `v0.25.0`

https://crates.io/crates/pyo3

Bindings to Python interpreter


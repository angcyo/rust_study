use crate::utils::uuid;
use clap::Parser;
use colored::Colorize;
use lazy_static::lazy_static;
use std::collections::HashMap;

mod args;
mod https;
mod image;
mod macros;
mod mail;
mod protocol;
mod utils;
mod web;

#[allow(dead_code)]
fn ensure_output_dir_exist() {
    let output = ".output/xxx.out";
    utils::ensure_dir_exist(output);
}

#[allow(dead_code)]
fn test_macro() {
    //println!("Hello, world!");
    println!();
    ptl!();
    ptl!("...println2...");
    //println2!("...println2...{:s}", "...println2...2");
    dbg!();
    //vec!();
    let huge: u32 = u32::MAX;
    dbg!(huge);
    dbg!(huge as u16);
    let small: u16 = 0;
    //dbg!(small-1);
    dbg!(9 / 3, 9 / 2, 9 / 1);
    dbg!(10 / 3);
}

#[allow(dead_code)]
async fn test_html2md() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org";
    let output = ".output/rust.md";

    ptl!("Request {}", url);
    let body = https::get_url_text(url).await?;

    // Create output directory
    utils::ensure_dir_exist(output);
    ptl!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    std::fs::write(output, md.as_bytes()).unwrap();
    ptl!("Converted markdown has been saved in {}.", output);
    Ok(())
}

#[allow(dead_code)]
fn test_args() {
    let args = args::Args::parse();

    for _ in 0..args.count {
        if let Some(name) = args.name.as_ref() {
            ptl!("Hello {}!", name.green().bold()); //>Hello angcyo!
        //ptl!("Hello {:?}!", name);//>Hello "angcyo"!
        } else {
            println!("Please use {} run.", "`--name xxx`".red().bold());
        }
        //println!("Hello {:?}!", args.name.as_ref().unwrap());
    }
}

#[allow(dead_code)]
async fn test_send_mail() {
    mail::send_mail(
        ("no-reply".to_string(), "no-reply@laserabc.com".to_string()),
        ("angcyo".to_string(), "angcyo@126.com".to_string()),
        format!("title - {}", utils::now_timestamp()).as_str(),
        format!(
            "<h1>Hello, body. <sup>html</sup>! {}</h1>",
            utils::now_date_time()
        )
        .as_str(),
        format!("Hello body. <sup>text</sup>! {}", utils::now_date_time()).as_str(),
        ("smtp.feishu.cn", 25),
        ("no-reply@laserabc.com", ""),
    )
    .await;
}

#[allow(dead_code)]
fn test_utf8() {
    let str = "你好, 中国! angcyo";
    let bytes = utils::string_to_bytes(str);
    let base64 = utils::base64_encode(bytes.as_slice());
    let md5 = utils::md5_encode(bytes.as_slice());
    log::warn!("{}", str);
    ptl!(
        "{} base64: {} md5: {}",
        utils::bytes_to_string(bytes.as_slice()),
        base64,
        md5
    );
    ptl!(
        "{}",
        utils::bytes_to_string(utils::base64_decode(base64.as_str()).unwrap().as_slice())
    );
}

#[allow(dead_code)]
fn test_image() {
    ensure_output_dir_exist();
    image::read_image_file("tests/FaceQ.png")
        .unwrap()
        .grayscale() // 灰度处理
        .save(".output/FaceQ_output.png")
        .unwrap();
}

//--

lazy_static! {
    static ref MAP: serde_json::Map<String, serde_json::Value> = {
        let mut m = serde_json::Map::new();
        m.insert(
            "0".to_string(),
            serde_json::Value::String("foo".to_string()),
        );
        m.insert(
            "1".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        m.insert(
            "2".to_string(),
            serde_json::Value::String("baz".to_string()),
        );
        m
    };
}

#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_utils();
    log::info!("{}", utils::get_current_dir());
    log::debug!(
        "{} {} {}",
        uuid(),
        utils::random_f64(),
        utils::random_range(0..100)
    );
    //--
    //serde_json::json!(serde_json::from_value(serde_json::Value::Object(MAP.clone())).unwrap());
    /*log::info!(
        "{:?}",
        serde_json::from_value(serde_json::Value::Object(MAP.clone())).unwrap()
    );*/
    MAP.keys().for_each(|k| {
        log::info!("key:{} value:{}", k, MAP.get(k).unwrap());
    });
    //serde_json::to_string(MAP.);
    //test_macro();
    //test_html2md().await?;
    //test_args();
    //test_send_mail().await;
    //web::start_serve().await;
    test_utf8();
    test_image();
    
    protocol::test_protocol();
    
    Ok(())
}

//--

#[cfg(test)]
mod tests {
    use crate::ensure_output_dir_exist;

    #[test]
    fn test_resize_image() {
        ensure_output_dir_exist();
        let width = 512;
        let height = width;
        let image_path = "tests/logo.png";
        let output_image_path = format!(".output/logo_{}_{}.png", width, height);
        crate::image::resize_image_file(image_path, width, height, output_image_path.as_str())
            .unwrap()
    }
}

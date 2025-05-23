use clap::Parser;
use colored::Colorize;

mod args;
mod https;
mod macros;
mod mail;
mod utils;
mod web;

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
    let output_dir = std::path::Path::new(output).parent().unwrap();
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }
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

#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_utils();
    //test_macro();
    //test_html2md().await?;
    //test_args();
    //test_send_mail().await;
    //web::start_serve().await;
    test_utf8();
    Ok(())
}

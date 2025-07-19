use colored::Colorize;
use lazy_static::lazy_static;
use rc_basis::ptl;
use std::collections::HashMap;
use std::sync::Mutex;

mod android;
mod badges;
mod mail;
mod pdf;
mod protocol;
mod threads;
mod web;
mod files;
mod nalgebra;
mod geo;
mod test;

#[allow(dead_code)]
fn ensure_output_dir_exist() {
    let output = ".output/xxx.out";
    rc_basis::files::ensure_parent_dir_exist(output);
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
    let body = rc_http::http::get_url_text(url).await?;

    // Create output directory
    rc_basis::files::ensure_parent_dir_exist(output);
    ptl!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    std::fs::write(output, md.as_bytes()).unwrap();
    ptl!("Converted markdown has been saved in {output}.");
    Ok(())
}

#[allow(dead_code)]
async fn test_send_mail() {
    mail::send_mail(
        ("no-reply".to_string(), "no-reply@laserabc.com".to_string()),
        ("angcyo".to_string(), "angcyo@126.com".to_string()),
        format!("title - {}", rc_basis::now_timestamp()).as_str(),
        format!(
            "<h1>Hello, body. <sup>html</sup>! {}</h1>",
            rc_basis::now_date_time()
        )
        .as_str(),
        format!("Hello body. <sup>text</sup>! {}", rc_basis::now_date_time()).as_str(),
        ("smtp.feishu.cn", 25),
        ("no-reply@laserabc.com", ""),
    )
    .await;
}

#[allow(dead_code)]
fn test_utf8() {
    let str = "你好, 中国! angcyo";
    let bytes = rc_basis::bytes::string_to_bytes(str);
    let base64 = rc_basis::bytes::base64_encode(bytes.as_slice());
    let md5 = rc_basis::bytes::md5_encode(bytes.as_slice());
    rc_log::log::warn!("{}", str);
    ptl!(
        "{} base64: {base64} md5: {md5}",
        rc_basis::bytes::bytes_to_string(bytes.as_slice()),
    );
    ptl!(
        "{}",
        rc_basis::bytes::bytes_to_string(
            rc_basis::bytes::base64_decode(base64.as_str())
                .unwrap()
                .as_slice()
        )
    );
}

#[allow(dead_code)]
fn test_image() {
    ensure_output_dir_exist();
    rc_image::read::read_image_file("tests/FaceQ.png")
        .unwrap()
        .grayscale() // 灰度处理
        .save(".output/FaceQ_output.png")
        .unwrap();
}

//--

lazy_static! {
    /// 不可变延迟初始化的静态map对象
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

lazy_static! {
    /// 可变的延迟初始化的静态map对象
    static ref MUT_MAP: Mutex<HashMap<String, String>> = {
        let mut map = HashMap::new();
        map.insert("0".to_string(), "0".to_string());
        map.insert("1".to_string(), "1".to_string());
        map.insert("2".to_string(), "true".to_string());
        Mutex::new(map)
    };
}

#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rc_log::init_log();
    rc_log::log::info!("{}", rc_basis::get_current_dir());
    rc_log::log::debug!(
        "{} {} {}",
        rc_basis::bytes::uuid(),
        rc_basis::num::random_f64(),
        rc_basis::num::random_range(0..100)
    );
    //--
    //serde_json::json!(serde_json::from_value(serde_json::Value::Object(MAP.clone())).unwrap());
    /*log::info!(
        "{:?}",
        serde_json::from_value(serde_json::Value::Object(MAP.clone())).unwrap()
    );*/
    MAP.keys().for_each(|k| {
        rc_log::log::info!("key:{k} value:{}", MAP.get(k).unwrap());
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
    use crate::{ensure_output_dir_exist, ptl, MAP, MUT_MAP};
    use rc_basis::get_current_dir;

    /// [study/src/main.rs:183:9]->/Users/angcyo/project/rust/rust_study/study
    #[test]
    fn it_works() {
        ptl!("{}", get_current_dir());
    }

    #[test]
    fn test_resize_image() {
        ensure_output_dir_exist();
        let width = 512;
        let height = width;
        let image_path = "../tests/FaceQ.png";

        let image_name = rc_basis::files::last_path(image_path);
        let output_image_name = format!("{image_name}_{width}_{height}.png");
        let output_image_path = format!("../.output/{output_image_name}");
        rc_image::convert::resize_image_file(image_path, width, height, output_image_path.as_str())
            .unwrap()
    }

    #[test]
    fn test_lazy_static() {
        rc_log::init_log();
        MAP.keys().for_each(|k| {
            rc_log::log::info!("key:{k} value:{}", MAP.get(k).unwrap());
        });

        //--

        let mut mut_map = MUT_MAP.lock().unwrap();
        mut_map.remove("0");
        mut_map.remove("2");
        mut_map.remove("3");
        mut_map.insert("new_key".to_string(), "new_value".to_string());
        mut_map.keys().for_each(|k| {
            rc_log::log::info!("mut_key:{k} mut_value:{}", mut_map.get(k).unwrap());
        });
        mut_map.clear();
        mut_map.keys().for_each(|k| {
            rc_log::log::info!("mut_key:{k} mut_value:{}", mut_map.get(k).unwrap());
        });
    }
}

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/22
///

pub async fn get_url_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    //println!("body = {body:?}");
    Ok(body)
}

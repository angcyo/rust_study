use clap::Parser;
use colored::Colorize;

mod args;
mod https;
mod macros;

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

#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //test_macro();
    //test_html2md().await?;
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
    Ok(())
}

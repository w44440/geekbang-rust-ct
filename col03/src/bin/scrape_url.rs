
use std::fs;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
///scrape config
struct Config {
    #[structopt(long)]
    ///the url to scrape
    url: String,

    #[structopt(long, default_value="thescrape.md")]
    ///write scraped page to
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args();
    println!("{:?}",config);

    let url = &config.url;
    let output = &config.output;
    
    println!("{:?}",std::env::current_dir());

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
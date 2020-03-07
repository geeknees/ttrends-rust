extern crate reqwest;

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://trends24.in/japan/")
        .await?
        .text_with_charset("utf-8")
        .await?;
    //println!("{:#?}", resp);

    let fragment = Html::parse_fragment(&resp);
    let selector = Selector::parse("a").unwrap();
    for element in fragment.select(&selector) {
        println!("{:#?}", element.inner_html());
    }

    Ok(())
}

#[test]
fn test_yes() {
    assert_eq!(true, true);
}

// scrape/mod.rs
//      The core scraper - nabu.

use ureq;
mod types;

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    let http_response: String = ureq::get(url)
                                      .set("User-Agent","Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                                      .call()?
                                      .into_string()?;
    Ok(http_response)
}

#[test]
fn mans_gotta_test() {
    make_request("https://www.amazon.in/s?k=mac+m1");
}

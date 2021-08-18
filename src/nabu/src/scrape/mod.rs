//                      /$$                
//                      | $$                
//   /$$$$$$$   /$$$$$$ | $$$$$$$  /$$   /$$
//  | $$__  $$ |____  $$| $$__  $$| $$  | $$
//  | $$  \ $$  /$$$$$$$| $$  \ $$| $$  | $$
//  | $$  | $$ /$$__  $$| $$  | $$| $$  | $$
//  | $$  | $$|  $$$$$$$| $$$$$$$/|  $$$$$$/
//  |__/  |__/ \_______/|_______/  \______/ 
//                              src/scrape/mod.rs
//                                  - The core scraper nabu.

#[allow(deprecated)]
#[allow(unused_imports)]

use ureq::{get,Error};
use regex::Regex;
mod types;
mod orel;


pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    let http_response: String = ureq::get(url)
                                      .set("User-Agent","Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                                      .call()?
                                      .into_string()?;
    Ok(http_response)
}

pub fn stage_one(html_response: &str, website_profile: types::Profile ) {
    let listing_expression = Regex::new(r#"<a class="(?:.*?)" target="(?:.*?)" rel="(?:.*?)" href="(.*?)"><div class="(?:.*?)"><div class="(?:.*?)"><div><div class="(?:.*?)" style="(?:.*?)"><img class="(?:.*?)" alt="(?:.*?)" src="(.*?[^">])">"#).unwrap();
    let listing_url = listing_expression.captures_iter(html_response)
                                        .map(|something| {
                                            vec![something[1].to_string() ,something[2][0..something[2].to_string().find('"').unwrap()].to_string() ]
                                        }).collect::<Vec<Vec<String>>>();
    println!("Found: {:#?}",listing_url);
	//println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}",product_name_url.text(), site_root, product_name_url.attr("href").unwrap(),pimg, price);
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.amazon.in/s?k=realme+x7").unwrap().bytes().count(),0);
}

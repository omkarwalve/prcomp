//                       /$$                
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

use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};
//use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde_json;
mod types;
mod orel;

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    println!("Making Request!");
    let n_agent: Agent = AgentBuilder::new()
                              .timeout(Duration::from_secs(5))
                              .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                              .build();
    let http_response: String = n_agent.get(url)
                                      .set("ACCEPT","text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                                      .set("ACCEPT-LANGUAGE","en-us")
                                      //.set("ACCEPT-ENCODING","gzip, deflate, br")
                                      .set("CACHE-CONTROL", "no-cache")
                                      .set("REFERER", "https://www.ajio.com/")
                                      .set("SEC-CH-UA", r#""Chromium";v="93", " Not A;Brand";v="99""#)
                                      .set("SEC-CH-UA-MOBILE", "?0")
                                      .set("SEC-FETCH-MODE", "navigate")
                                      .set("SEC-FETCH-DEST", "document")
                                      .set("SEC-FETCH-SITE", "same-origin")
                                      .set("SEC-FETCH-USER", "?1")
                                      .set("UPGRADE-INSECURE-REQUESTS", "1")
                                      .call()?
                                      .into_string()?;
    println!("Got Response!");
    Ok(http_response)
}

pub fn stage_one(html_response: &str) {
    //println!("{:#?}",html_response);
    //panic!("--BREAKPOINT--");
    let html_document = Document::from(html_response);
    for lnode in html_document.find(Class("productbox")) {
        //println!("Inside for loop");
        let purl = lnode.find(Class("product-img")).next().unwrap().attr("href").unwrap();
        let pimg = lnode.find(Class("product-img-default")).next().unwrap().attr("src").unwrap();
        let product_name = lnode.find(Class("product-title")).next().unwrap().text();
        let price = match lnode.find(Class("price-number").descendant(Name("span"))).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
        let site_root = "https://www.urbanladder.com";
        println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}", product_name.trim(), site_root, purl, pimg, price.trim());
    }
    //println!("Past for loop I guess");
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.urbanladder.com/").unwrap().bytes().count(),0);
}


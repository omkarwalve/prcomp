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

use std::time::Duration;
use std::borrow::Cow;
use ureq::{Agent, AgentBuilder, Error};
//use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use serde_json;
mod types;

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    println!("Making Request!");
    let n_agent: Agent = AgentBuilder::new()
                              .timeout(Duration::from_secs(5))
                              .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                              .build();
    let http_response: String = n_agent.get(url)
                                      .set("ACCEPT","text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                                      .set("ACCEPT-LANGUAGE","en-GB")
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
    //println!("{:#?}",html_document);
    //let listing_expression = Regex::new(r#"window.__PRELOADED_STATE__ = (.*?) </script>"#).unwrap();
    //println!("Going to find listings now in ajio DOM using regex");
    //for site_content in listing_expression.captures_iter(html_response) {
        //let pjson: serde_json::Value = serde_json::from_str(&site_content[1]).unwrap();
        //println!("Found:\n{:#?}",pjson);
    //}

    for lnode in html_document.find(Name("body")) {
        println!("Found Body");
        let script_child = lnode.children().next().unwrap().next().unwrap().next().unwrap().next().unwrap().next().unwrap().next().unwrap().next().unwrap().next().unwrap().next().unwrap().next().unwrap();
        //let pjson: serde_json::Value = serde_json::from_str(&).unwrap();
        //println!("Inside for loop");
        let json_part = script_child.text().trim().split('=').collect::<Vec<&str>>()[1];
        let batata : serde_json::Value = serde_json::from_str(json_part).unwrap();
        println!("SCHEMA JSON IS:\n----------\n{:#?}",batata);
        //let purl = lnode.parent().unwrap().attr("href").unwrap();
        //let pimg = lnode.find(Name("img")).next().unwrap().attr("src").unwrap();
        //let product_name = lnode.find(Class("name")).next().unwrap().text();
        //let price = match lnode.find(Class("price")).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
        //let site_root = "https://www.ajio.com";
        //println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}", product_name, site_root, purl, pimg, price);
    }
    //println!("Past for loop I guess");
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.ajio.com/search/?text=black%20sneakers").unwrap().bytes().count(),0);
}


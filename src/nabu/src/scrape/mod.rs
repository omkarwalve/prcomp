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

use crate::orel;
use std::time::Duration;
use fastrand;
use ureq::{Agent, AgentBuilder, Error};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
//use serde_json;
mod types;

const WAIT_FOR_RESPONSE_TIMEOUT : u64 = 10;
const USER_AGENT_POOL : [&'static str; 11] = ["Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0; Trident/7.0; rv:11.0) like Gecko",
                                              "Mozilla/4.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Edg/92.0.902.78",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                              "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_5_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                              "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                            ];

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    println!("Making Request!");
    let n_agent: Agent = AgentBuilder::new()
                              .timeout(Duration::from_secs(WAIT_FOR_RESPONSE_TIMEOUT))
                              .user_agent(USER_AGENT_POOL[fastrand::usize(..USER_AGENT_POOL.len())])
                              .build();
    let http_response: String = n_agent.get(url)
                                       .set("ACCEPT","text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                                       .set("ACCEPT-LANGUAGE","en-us")
                                       .set("CACHE-CONTROL", "no-cache")
                                       .set("REFERER", "https://www.google.com/search?q=shopping")
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

pub fn stage_one(html_response: &str, website_profile: orel::Orel<String>) -> types::Listing<String> {
    //println!("{:#?}",html_response); // VERBOSE
    //panic!("--BREAKPOINT--");
    println!("Parsing..");
    let html_document = Document::from(html_response);
    let mut plisting: types::Listing<String> = Default::default();
    if website_profile.listing_find_by == "Class"
    {   for lnode in html_document.find(Class(&website_profile.listing_identifier[..])) {
            //println!("Inside for loop");
            if website_profile.product_url_find_by == "Class" {
                plisting.url = lnode.find(Class(&website_profile.product_url_identifier[..])).next().unwrap().attr("href").unwrap().to_string();
            } else {
                plisting.url = lnode.find(Attr(&website_profile.product_url_identifier[..],&website_profile.product_url_ivalue[..])).next().unwrap().attr("href").unwrap().to_string();
            }

            plisting.img = lnode.find(Class(&website_profile.image_identifier[..])).next().unwrap().attr("src").unwrap().to_string();

            if website_profile.product_name_find_by == "Class" {
                plisting.name = lnode.find(Class(&website_profile.product_url_identifier[..])).next().unwrap().text();
            } else {
                plisting.name = lnode.find(Attr(&website_profile.product_name_identifier[..],&website_profile.product_name_ivalue[..])).next().unwrap().text();
            }
            if website_profile.product_price_find_by == "Class" {
                plisting.price = match lnode.find(Class(&website_profile.product_price_identifier[..]).descendant(Name("span"))).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
            } else {
                plisting.price = match lnode.find(Attr(&website_profile.product_price_identifier[..],&website_profile.product_price_ivalue[..]).descendant(Name("span"))).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
            }
            println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}", plisting.name, &website_profile.root_uri, plisting.url, plisting.img, plisting.price);
        }
            plisting
    }
    else 
    {   for lnode in html_document.find(Attr(&website_profile.listing_identifier[..],&website_profile.listing_ivalue[..])) {
            if website_profile.product_url_find_by == "Class" {
                plisting.url = lnode.find(Class(&website_profile.product_url_identifier[..])).next().unwrap().attr("href").unwrap().to_string();
            } else {
                plisting.url = lnode.find(Attr(&website_profile.product_url_identifier[..],&website_profile.product_url_ivalue[..])).next().unwrap().attr("href").unwrap().to_string();
            }

            plisting.img = lnode.find(Class(&website_profile.image_identifier[..])).next().unwrap().attr("src").unwrap().to_string();

            if website_profile.product_name_find_by == "Class" {
                plisting.name = lnode.find(Class(&website_profile.product_url_identifier[..])).next().unwrap().text();
            } else {
                plisting.name = lnode.find(Attr(&website_profile.product_name_identifier[..],&website_profile.product_name_ivalue[..])).next().unwrap().text();
            }
            if website_profile.product_price_find_by == "Class" {
                plisting.price = match lnode.find(Class(&website_profile.product_price_identifier[..]).descendant(Name("span"))).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
            } else {
                plisting.price = match lnode.find(Attr(&website_profile.product_price_identifier[..],&website_profile.product_price_ivalue[..]).descendant(Name("span"))).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
            }
            println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}", plisting.name, &website_profile.root_uri, plisting.url, plisting.img, plisting.price);
        }
            plisting
    }
}

pub fn stage_two(part_listing: types::Listing<String>) {
    //do lv2 stuff here
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.urbanladder.com/").unwrap().bytes().count(),0);
}


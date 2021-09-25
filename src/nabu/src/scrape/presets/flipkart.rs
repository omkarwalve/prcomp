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
//use html5ever;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
mod types;

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    let http_response: String = ureq::get(url)
                                      .set("User-Agent","Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                                      .call()?
                                      .into_string()?;
    println!("Got Response!");
    Ok(http_response)
}

pub fn stage_one(html_response: &str) {
    let html_document = Document::from(html_response);
    //println!("{:#?}",html_document);
    // Flipkart Study
    // https://www.flipkart.com/search?q=m1%20macbook
    // Listing Tag
    // <a class="_1fQZEK" target="_blank" rel="noopener noreferrer" href="/apple-macbook-air-m1-8-gb-512-gb-ssd-mac-os-big-sur-mgn73hn-a/p/itm37da92e833fa3?pid=COMFXEKM8GQJW5DE&amp;lid=LSTCOMFXEKM8GQJW5DEVOHG87&amp;marketplace=FLIPKART&amp;q=m1+macbook&amp;store=6bo%2Fb5g&amp;srno=s_1_1&amp;otracker=search&amp;otracker1=search&amp;fm=organic&amp;iid=c3562f5a-55d0-44c3-aebd-f12856c63bfe.COMFXEKM8GQJW5DE.SEARCH&amp;ppt=None&amp;ppn=None&amp;ssid=0gg5mgtaj40000001629175294814&amp;qH=dcdaca4e855184a5">
    for lnode in html_document.find(Class("_1fQZEK")) {
        let purl = lnode.attr("href").unwrap();
        let pimg = lnode.find(Class("_396cs4")).next().unwrap().attr("src").unwrap();
        let product_name = lnode.find(Class("_4rR01T")).next().unwrap().text();
        let price = match lnode.find(Class("_30jeq3")).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
        let site_root = "https://www.flipkart.com";
        //{ Some(node) => node, None => panic!("NO LINKS FOUND"), };
        println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}", product_name, site_root, purl, pimg, price);
    }
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.amazon.in/s?k=realme+x7").unwrap().bytes().count(),0);
}

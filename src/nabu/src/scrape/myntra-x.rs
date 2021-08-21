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
use ureq::{Agent, AgentBuilder, Error};
//use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
mod types;

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    println!("Making Request!");
    let nAgent: Agent = AgentBuilder::new()
                              .timeout(Duration::from_secs(5))
                              .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                              .build();
    let http_response: String = nAgent.get(url)
                                      .set("ACCEPT","text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                                      .set("ACCEPT-LANGUAGE","en-us")
                                      .set("CACHE-CONTROL", "no-cache")
                                      .set("PRAGMA", "no-cache")
                                      .set("REFERER", "https://www.myntra.com/")
                                      .set("SEC-CH-UA", r#""Chromium";v="93", " Not A;Brand";v="99""#)
                                      .set("SEC-CH-UA-MOBILE", "?0")
                                      .set("SEC-FETCH-MODE", "navigate")
                                      .set("SEC-FETCH-DEST", "document")
                                      .set("SEC-FETCH-SITE", "same-origin")
                                      .set("SEC-FETCH-USER", "?1")
                                      .set("UPGRADE-INSECURE-REQUESTS", "1")
                                      //.set("COOKIE", "AKA_A2=A; bm_sz=49502F8F468A92F386BCD96BBA8E13EF~YAAQvv3UF7s60p96AQAATK8hYwyAG774Wxi2QA29lNDD7rI8qiMA4tUWzor4ZS4+IY1ElGl/10lF/YJCR2BT6qRA1KdX8V4vddKZXK3FCPxwf8l6OQsBVy7VWgGELQUjqyor40aawSPdjMVTdtZtnBAxLQsvJ/u5EDEQAG8jKz4/eS+AhhBTlnZjT78b9o91TyGGONS2FRng5Bw+3d6jzzpahZLbVYsrXeUOYdHPwg8oaxNUiIYyL37YDeQskwKiJPN6bBb5BKvs+TLWhL9meIaOMVJmAOKTLkaviY4dERVZQdo=~4539184~3290945; _d_id=37691d3a-5fbb-4dfe-ab20-492a464d6398; mynt-eupv=1; bc=true; _xsrf=uB30znz48idw1numkEh4LLYmnl3DkZIX; ak_bmsc=B8B6EB503FF82849519126C65136CAC0~000000000000000000000000000000~YAAQvv3UFw870p96AQAAsLQhYwxTBNn7wkYw4d4XcOqIeIdRqt4+3K9BVzZkeKNb6X2+KZby6HYskh5+qBvqqJOwdA6Islh6F5RUgaExtihXPrpkYWE05XHfZBTFgF5Xg490a9pQif2XUM4eHBBPpY15qhKKyDSw2vXdtyaveTKNoimegHLNOCdGfMd9OtxgVbda+qK76rGx8T9Lv+Naoz93rKqF5VChhCbcEy5jQod3F3yk38s+opH04KZ5/O5cbWfAaaJtTaSxt0InwM1qcz5NX1dyILseaUGW+7JBgbNrJxT0R36AkctvHxD7nu0VBICCIQ1fo7xhhM8Y3nErUz8eW1WANUF4YLbp2kLcZXl5Dp73Wp17l8btdlCD9lFf8x2UWwTILF0g7JQFUcJHptvwoX+BApumJznsqFQRBHi0YLi5/ZmEFM7/hSAdXNtkt8gRxHPvjwKLG6CVTOADD9kdiLadkPqfGad+YGzGLDpX4I9xuZm1EtbdJkNr; at=ZXlKaGJHY2lPaUpJVXpJMU5pSXNJbXRwWkNJNklqRWlMQ0owZVhBaU9pSktWMVFpZlEuZXlKdWFXUjRJam9pTmpnME5ERmlNMkl0TURGaE1pMHhNV1ZqTFdGaE1qY3RNREF3WkROaFpqSTRaVFF4SWl3aVkybGtlQ0k2SW0xNWJuUnlZUzB3TW1RM1pHVmpOUzA0WVRBd0xUUmpOelF0T1dObU55MDVaRFl5WkdKbFlUVmxOakVpTENKaGNIQk9ZVzFsSWpvaWJYbHVkSEpoSWl3aWMzUnZjbVZKWkNJNklqSXlPVGNpTENKbGVIQWlPakUyTkRVd01EYzNOVGdzSW1semN5STZJa2xFUlVFaWZRLjJEN19SNTFhTk4wZHY0Q2g4VmhIU1RvblpSS2o3bUw0T2R2aGxnc1BCYjQ=; _abck=A69BA0F199B0E54C3E2535BB64C43FD1~0~YAAQvv3UFyo70p96AQAANLYhYwaRGglKYOrA5L7ybjv29eJTRqTcnkezEu4b0UgjuG/yCWQLCSUG4mDEwUkZqus1wYg6ov5N5LDJYU++NuNpwXuhILxdJ+pnD/lg1HhYD+x8jxdUmFqA3byzeO8EdgXexrr84Dpn0QU8KuDwVvTIwvuS/1rzC1Mn/hnnfzbYkvIAwW0+tENXhPM5e1ul8Os5bzUFxDWYqkn8TV+QfKugOdJ7x6dSAY2Jgu2D5bzgaJbMGgK8x7MHfqEqjcfgIabKZLxqfxePWFTwJHBbrR/JrGPAImwVNe2Tb477j3xe2KbUk62h3Sw/Iq1KEA1x1zwFrk4ZSQoyNYU6u5Plh2eJyZ0ISPlfstEOWdG6N7ZshyPK6z6J9LZ+h9MCYlFUzoRlWFtCoJ2u~-1~-1~-1; mynt-ulc-api=pincode%3A380006; mynt-loc-src=expiry%3A1629457198995%7Csource%3AIP; utrid=SlRVSkNkBAtDEFgUCxpAMCM2NTk2Mzk5MTgkMg%3D%3D.7542be9b4bfe11a1e3e21432a72ac8d3; _ma_session=%7B%22id%22%3A%220c5b90b6-7bf9-42ef-b6fc-1141c6b9b504-37691d3a-5fbb-4dfe-ab20-492a464d6398%22%2C%22referrer_url%22%3A%22%22%2C%22utm_medium%22%3A%22%22%2C%22utm_source%22%3A%22%22%2C%22utm_channel%22%3A%22direct%22%7D; akaas_myntra_SegmentationLabel=1632049841~rv=75~id=5fc8f3885fb9d2cdd2c94bcee6b66aa1~rn=PWA; bm_sv=FB347D0CB76F205412A3251FF6EDF9A3~hHc9Il+gdmrC6jYidfRmgriFEi8HVMwrhcehKvuCYCJZi7r3pTAWVsRusAAIyUtjHOQeIK6exy39yK6quisla4V2SlplAWxSsjwsATvKRWazX9ZX481f7KrqeJw+NRwCTvuWqZvGGL6j4DecuLDvSTPpt/rn4gSrYMNqp7X5P5o=")
                                      .call()?
                                      .into_string()?;
    println!("Got Response!");
    Ok(http_response)
}

pub fn stage_one(html_response: &str) {
    println!("{:#?}",html_response);
    panic!("--BREAKPOINT--");
    let html_document = Document::from(html_response);
    //println!("{:#?}",html_document);
    for lnode in html_document.find(Class("product-base")) {
        let purl = lnode.find(Name("a")).next().unwrap().attr("href").unwrap();
        let pimg = lnode.find(Name("img")).next().unwrap().attr("src").unwrap();
        let product_name = lnode.find(Class("product-product")).next().unwrap().text();
        let price = match lnode.find(Class("product-discountedPrice")).next() { Some(node) => node.text(), None => "Not Available".to_string(), };
        let site_root = "https://www.myntra.com";
        println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}", product_name, site_root, purl, pimg, price);
    }
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.myntra.com/mens-denims").unwrap().bytes().count(),0);
}


//                       /$$                
//                      | $$                
//   /$$$$$$$   /$$$$$$ | $$$$$$$  /$$   /$$
//  | $$__  $$ |____  $$| $$__  $$| $$  | $$
//  | $$  \ $$  /$$$$$$$| $$  \ $$| $$  | $$
//  | $$  | $$ /$$__  $$| $$  | $$| $$  | $$
//  | $$  | $$|  $$$$$$$| $$$$$$$/|  $$$$$$/
//  |__/  |__/ \_______/|_______/  \______/ 
//                              src/scrape/mod.rs
//                                  - The core scraper/crawler nabu.

#[allow(deprecated)]

use crate::orel;
use crate::types::{ self, JSONize};
use std::time::Duration;
use std::sync::{ Arc, Mutex};
use fastrand;
use ureq::{Agent, AgentBuilder, Error};
use select::document::Document;
use reqwest;
use futures::{ stream, StreamExt};
use tokio;
use select::predicate::{Attr, Class, Name, Predicate};
use ansi_term::Color;

//use serde_json;

const ALT_CHAR: char = '|';
const WAIT_FOR_RESPONSE_TIMEOUT : u64 = 5;
const USER_AGENT_POOL : [&'static str; 11] = [
                                              "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0; Trident/7.0; rv:11.0) like Gecko",
                                              "Mozilla/4.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Edg/92.0.902.78",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                              "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                              "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                            ];
const PROXY_POOL : [&'static str; 10 ] = [
                                           "103.92.114.6:8080",
                                           "202.62.67.209:53281",
                                           "117.242.36.205:42252",
                                           "182.72.150.242:8080",
                                           "103.251.214.167:6666",
                                           "103.216.82.20:6666",
                                           "14.139.57.195:23500",
                                           "43.241.141.27:35101",
                                           "117.241.98.189:44643",
                                           "103.216.82.18:6666",
                                         ];
const NOT_FOUND_MESSAGE : &'static str = "❔";
const CONFIG_ERROR_MESSAGE : &'static str = "❗CONFIGURATION-ERROR❗";
const FAKE_RESPONSE : &'static str = "<h1>REQUEST FAILED</h1>";

//const FIND_BY : [&'static String; 4 ] = [ &"Attr.d".to_string(), &"Class.d".to_string() , &"Attr".to_string() , &"Class".to_string() ];

macro_rules! mkvec {
    ($node: expr) => { $node.into_iter()
                             .map(|node| node.text())
                             .collect::<Vec<String>>()
    }
}

macro_rules! clean {
    ($text: expr, "wty") => { $text.replace("Know More","") };
    ($text: expr, "rep") => { $text.replace("?","") }
}

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    println!("Making Request to {}",url);
    let n_agent: Agent = AgentBuilder::new()
                              .timeout(Duration::from_secs(WAIT_FOR_RESPONSE_TIMEOUT))
                              .user_agent(USER_AGENT_POOL[fastrand::usize(..USER_AGENT_POOL.len())])
                              //.proxy(ureq::Proxy::new(PROXY_POOL[fastrand::usize(..PROXY_POOL.len())]).unwrap())
                              .build();
    let http_response: String = n_agent.get(url)
                                       .set("Accept","text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                                       .set("Accept-Language","en-us")
                                       .set("Cache-Control", "no-cache")
                                       .set("Connection", "keep-alive")
                                       .set("Referer", "https://www.google.com/search?q=shopping")
                                       .set("Sec-CH-UA", r#""Chromium";v="93", " Not A;Brand";v="99""#)
                                       .set("Sec-CH-UA-Mobile", "?0")
                                       .set("Sec-Fetch-Mode", "navigate")
                                       .set("Sec-Fetch-Dest", "document")
                                       .set("Sec-Fetch-Site", "same-origin")
                                       .set("Sec-Fetch-User", "?1")
                                       .set("Upgrade-Insecure-Requests", "1")
                                       .call()?
                                       .into_string()?;
    println!("{}", Color::Green.paint(format!("-- Got Response from {}", url)));
    Ok(http_response)
}

pub fn stage_one<'t>(html_response: &str, website_profile: &'t orel::Orel<String>) -> Option<(Vec<types::Listing<String>>,&'t orel::Orel<String>)> {
    //println!("{:#?}",html_response); // VERBOSE
    //panic!("--BREAKPOINT--");
    println!("Parsing..");
    let html_document = Document::from(html_response);
    let mut plistings : Vec<types::Listing<String>> = Vec::new();
    println!("making iterator..");
    let listing_iterator: Option<Box<dyn Iterator<Item = select::node::Node>>>
        = match website_profile.listing_find_by.as_str() {
            "Class" =>  Some(Box::new(html_document.find(Class(&website_profile.listing_identifier[..])))),
            "Class.alt" =>  Some(Box::new( 
                             { let classes = &website_profile.listing_identifier.split_once(ALT_CHAR).expect("Inseperabale Class.alt.identifier");
                               let mut find1 = html_document.find(Class(classes.0));
                               if find1.next().is_some() {
                                   find1
                               }
                               else {
                                   let find2 = html_document.find(Class(classes.1));
                                   find2
                               }
                             } )),
            "Attr"  =>  Some(Box::new(html_document.find(Attr(&website_profile.listing_identifier[..],
                                                         &website_profile.listing_ivalue[..])))),
            _ => { println!("Failed to build iterator"); None },
        };
    println!("successfully made iterator..");
    if listing_iterator.is_some() {
    for lnode in listing_iterator.unwrap() {
        println!("Inside iterator for loop..");
            let mut plisting: types::Listing<String> = Default::default();
            //-- URL
            //println!("url..");
            plisting.url
                = match website_profile.product_url_find_by.as_str() {
                    "Class" => format!("{}{}",&website_profile.root_uri
                                             ,lnode.find(Class(&website_profile.product_url_identifier[..]))
                                                   .next()
                                                   .expect(&format!("Failed to get URL from {}", Color::Red.paint(&website_profile.product_url_identifier)))
                                                   .attr("href")
                                                   .expect("href attribute not found")
                                                   .to_string()),
                    "Class.alt" => 
                        format!("{}{}",&website_profile.root_uri
                                      ,{  let classes = &website_profile.product_url_identifier.split_once(ALT_CHAR).expect("Inseperabale Class.alt.URL.identifier");
                                          let find1 = lnode.find(Class(classes.0)).next();
                                          if find1.is_some() {
                                           find1.expect(&format!("Failed to get URL from {}. Raw HTML", Color::Red.paint(classes.0)))
                                                .attr("href")
                                                .expect("href attribute not found")
                                                .to_string()
                                        } else {
                                             lnode.find(Class(classes.1))
                                                  .next()
                                                  .expect(&format!("Failed to get URL from {}", Color::Red.paint(classes.1)))
                                                  .attr("href")
                                                  .expect("href attribute not found")
                                                  .to_string()
                                         }
                                     }),
                    "Attr" => format!("{}{}",&website_profile.root_uri
                                            ,lnode.find(Attr(&website_profile.product_url_identifier[..]
                                                            ,&website_profile.product_url_ivalue[..]))
                                                  .next()
                                                  .expect("Failed to get URL")
                                                  .attr("href")
                                                  .expect("href attribute not found")
                                                  .to_string()),
                        "self" => format!("{}{}",&website_profile.root_uri
                                                ,lnode.attr("href").unwrap().to_string()),
                    _ => CONFIG_ERROR_MESSAGE.to_string()
                };
            //-- IMAGE
            //println!("imageurl..");
            plisting.img = lnode.find(Class(&website_profile.image_identifier[..]))
                                .next().unwrap().attr("src").unwrap().to_string();
            //-- PRODUCT NAME
            //println!("pname..");
            plisting.name
                = match website_profile.product_name_find_by.as_str() {
                    "Class" => lnode.find(Class(&website_profile.product_name_identifier[..]))
                                     .next().unwrap().text().replace("\"","\\\""),
                    "Class.alt" => { 
                        let classes = &website_profile.product_name_identifier.split_once(ALT_CHAR).expect("Inseperable Class.alt.pnameident");
                        let mut find1 = lnode.find(Class(classes.0)).next();
                        if find1.is_some() {
                             find1.unwrap().text().replace("\"","\\\"")
                        }
                        else {
                            lnode.find(Class(classes.1)).next().unwrap().text().replace("\"","\\\"")
                        }
                    },
                    "Attr" => lnode.find(Attr(&website_profile.product_name_identifier[..],
                                                &website_profile.product_name_ivalue[..]))
                                     .next().unwrap().text().replace("\"","\\\""),
                    _ => CONFIG_ERROR_MESSAGE.to_string(),
            };
            //-- PRICE
            //println!("price..");
            plisting.price
                = match website_profile.product_price_find_by.as_str() {
                    "Class.d" => match lnode.find(Class(&website_profile.product_price_identifier[..])
                                                   .descendant(Name(&website_profile.product_price_ivalue[..])))
                                            .next() { Some(node) => node.text(), 
                                                      None => "Not Available".to_string(), },
                    "Attr"  => match lnode.find(Attr(&website_profile.product_price_identifier[..],
                                                       &website_profile.product_price_ivalue[..]))
                                                   .next() { Some(node) => node.text(),
                                                             None => "Not Available".to_string(), },
                    _ => CONFIG_ERROR_MESSAGE.to_string(),
            };
            //-- STORE
            plisting.store = website_profile.name.clone();
            println!("==== Found ====\n=> PRODUCT: {}\n=> URL: {}\n=> IMG.SRC:- {}\n=> PRICE: {}",
                     plisting.name,
                     plisting.url,
                     plisting.img,
                     plisting.price);
            plistings.push(plisting);
        }
        Some((plistings,website_profile))
    }
    else { println!("No listings element found in {}", &website_profile.name); 
           None
    }
}

//#[tokio::main]
async fn concurrent_requests(urls: Vec<String>) -> Result<Vec<select::document::Document>, Box<dyn std::error::Error>> {
    let concurrent_requests: usize = urls.len();
    let mut headers = reqwest::header::HeaderMap::new();
          headers.insert("Accept", reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
          headers.insert("Accept-Language", reqwest::header::HeaderValue::from_static("en-us"));
          headers.insert("Cache-Control", reqwest::header::HeaderValue::from_static("no-cache"));
          headers.insert("Connection", reqwest::header::HeaderValue::from_static("keep-alive"));
          headers.insert("Referer", reqwest::header::HeaderValue::from_static("https://www.google.com/search?q=shopping"));
          headers.insert("Sec-CH-UA", reqwest::header::HeaderValue::from_static(r#""Chromium";v="93", " Not A;Brand";v="99""#));
          headers.insert("Sec-CH-UA-Mobile", reqwest::header::HeaderValue::from_static("?0"));
          headers.insert("Sec-Fetch-Mode", reqwest::header::HeaderValue::from_static("navigate"));
          headers.insert("Sec-Fetch-Dest", reqwest::header::HeaderValue::from_static("document"));
          headers.insert("Sec-Fetch-Site", reqwest::header::HeaderValue::from_static("same-origin"));
          headers.insert("Sec-Fetch-User", reqwest::header::HeaderValue::from_static("?1"));
          headers.insert("Upgrade-Insecure-Requests", reqwest::header::HeaderValue::from_static("1"));

    let nclient = reqwest::Client::builder()
                  .user_agent(USER_AGENT_POOL[fastrand::usize(..USER_AGENT_POOL.len())])
                  .default_headers(headers)
                  .timeout(Duration::from_secs(WAIT_FOR_RESPONSE_TIMEOUT))
                  .build().unwrap();

    let all_the_responses = stream::iter(urls)
                            .map(|url| {
                                         let lc_nclient = &nclient;
                                         async move {
                                             lc_nclient.get(url).send().await?.text().await
                                         }
                                       }
                                ).buffer_unordered(concurrent_requests);

    let response_vec: Arc<Mutex<Vec<select::document::Document>>> = Arc::new(Mutex::new(Vec::new()));

    all_the_responses
        .for_each(|resp| async {
            match resp {
                Ok(res) => response_vec.lock().unwrap().push(Document::from(res.as_str())),
                Err(e) => { println!("{}",Color::Red.bold().paint(format!("Async Request failed\tReason: {}",e)));
                            response_vec.lock().unwrap().push(Document::from(FAKE_RESPONSE))
                },
            }
        }).await;
    let output = response_vec.lock().unwrap();

    Ok(output.to_vec())
}

//pub fn stage_two(Some((mut listings,profile)) : Option<(Vec<types::Listing<String>>, &orel::Orel<String>)>) -> Vec<types::Listing<String>> {
pub fn stage_two(tupie : Option<(Vec<types::Listing<String>>, &orel::Orel<String>)>) -> Option<Vec<types::Listing<String>>> {
  if tupie.is_some() {
    let (mut listings, profile ) = tupie.unwrap();
    let urls: Vec<String> = listings.iter().map(|listing| listing.url.clone()).collect();
    //let mut urls: Vec<String> = Vec::new();
    //for listing in listings.iter() {
        //urls.push(listing.url.clone());
    //}
    let product_pages = tokio::runtime::Runtime::new().unwrap().block_on(concurrent_requests(urls)).unwrap();
    //println!("Found the following::-----------------\n{:#?}",product_pages);
    //panic!("------BREAKPOINT-------");
    let mut count: usize = 0;
    for listing in listings.iter_mut() {
        //let product_page = Document::from(make_request(&listing.url).unwrap().as_str());
        //let product_page = &product_pages[count];
        //println!("The following page is for the first product::\n-----------------\n{:#?}",product_page);
        //panic!("------BREAKPOINT-------");
        println!("Parsing product info for {} from {}", &listing.name, &listing.store);
        // RETURN POLICY
        listing.return_replace 
            = match profile.product_return_policy_find_by.as_str() {
                "Attr.d" => match product_pages[count].find(Attr(&profile.product_return_policy_identifier[..],
                                                        &profile.product_return_policy_ivalue[..])
                                                    .descendant(Name(&profile.product_return_policy_idescendant[..])))
                                              .next() { Some(node) => clean!(node.text(),"rep"),
                                                        None => format!("{}", NOT_FOUND_MESSAGE) },
                "Class.d" => match product_pages[count].find(Class(&profile.product_return_policy_identifier[..])
                                                     .descendant(Name(&profile.product_return_policy_idescendant[..])))
                                               .next() { Some(node) => clean!(node.text(),"rep"),
                                                        None => format!("{}", NOT_FOUND_MESSAGE) },
                "Class" => match product_pages[count].find(Class(&profile.product_return_policy_identifier[..]))
                                             .next() { Some(node) => clean!(node.text(),"rep"),
                                                       None => format!("{}", NOT_FOUND_MESSAGE) },
                "Attr" => match product_pages[count].find(Attr(&profile.product_return_policy_identifier[..],
                                                       &profile.product_return_policy_ivalue[..]))
                                            .next() { Some(node) => clean!(node.text(),"rep"),
                                                      None => format!("{}", NOT_FOUND_MESSAGE) },
                _ => CONFIG_ERROR_MESSAGE.to_string()
        };
        println!("Done with replace");
        // WARRANTY
        listing.warranty
            = match profile.product_warranty_find_by.as_str() {
                "Attr.d" => match product_pages[count].find(Attr(&profile.product_warranty_identifier[..],
                                                         &profile.product_warranty_ivalue[..])
                                                    .descendant(Name(&profile.product_warranty_idescendant[..])))
                                              .next() { Some(node) => clean!(node.text(),"wty"),
                                                        None => format!("{}", NOT_FOUND_MESSAGE) },
                "Class.d" => match product_pages[count].find(Class(&profile.product_warranty_identifier[..])
                                                     .descendant(Name(&profile.product_warranty_idescendant[..])))
                                               .next() { Some(node) => clean!(node.text(),"wty"),
                                                        None => format!("{}", NOT_FOUND_MESSAGE) },
                "Class" => match product_pages[count].find(Class(&profile.product_warranty_identifier[..]))
                                             .next() { Some(node) => clean!(node.text(),"wty"),
                                                       None => format!("{}", NOT_FOUND_MESSAGE) },
                "Attr" => match product_pages[count].find(Attr(&profile.product_warranty_identifier[..],
                                                       &profile.product_warranty_ivalue[..]))
                                            .next() { Some(node) => clean!(node.text(),"wty"),
                                                      None => format!("{}", NOT_FOUND_MESSAGE) },
                _ => CONFIG_ERROR_MESSAGE.to_string()
        };
        println!("Done with warranty");
        // SPECS
        listing.specs
            = match profile.product_specs_find_by.as_str() {
                "Attr.d" => match product_pages[count].find(Attr(&profile.product_specs_identifier[..],
                                                                 &profile.product_specs_ivalue[..])
                                                    .descendant(Name(&profile.product_specs_idescendant[..])))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str()
                                                                            ,profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }.to_json()
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                              }.to_json()
                                                                                             },
                                                                        _ => CONFIG_ERROR_MESSAGE.to_string(),
                                                                      },
                                                        None => format!("{}", NOT_FOUND_MESSAGE) },
                "Class.d" => match product_pages[count].find(Class(&profile.product_specs_identifier[..])
                                                     .descendant(Name(&profile.product_specs_idescendant[..])))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str(),
                                                                             profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }.to_json()
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                              }.to_json()
                                                                                             },
                                                                        _ => CONFIG_ERROR_MESSAGE.to_string(),
                                                                      },
                                                         None => format!("{}", NOT_FOUND_MESSAGE) },
                "Class" => match product_pages[count].find(Class(&profile.product_specs_identifier[..]))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str(),
                                                                             profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }.to_json()
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                               }.to_json()
                                                                                             },
                                                                        _ => CONFIG_ERROR_MESSAGE.to_string(),
                                                                      },
                                                       None => format!("{}", NOT_FOUND_MESSAGE) },
                "Attr" => match product_pages[count].find(Attr(&profile.product_specs_identifier[..],
                                                       &profile.product_specs_ivalue[..]))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str(),
                                                                             profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }.to_json()
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))
                                                                                                                            ),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                               }.to_json()
                                                                                             },
                                                                        _ => CONFIG_ERROR_MESSAGE.to_string(),
                                                                      },
                                                      None => format!("{}", NOT_FOUND_MESSAGE) },
                _ => CONFIG_ERROR_MESSAGE.to_string()
        };
        println!("Done with specs");
        count = count+1;
    }
  Some(listings)
  }
  else { None }
}

#[test]
fn is_http_request_working() {
    assert_ne!(make_request("https://www.urbanladder.com/").unwrap().bytes().count(),0);
    assert_ne!(make_request("https://www.amazon.in/").unwrap().bytes().count(),0);
    assert_ne!(make_request("https://www.flipkart.com/").unwrap().bytes().count(),0);
}

#[test]
fn spectable_test() {
    println!("{}", types::Spectable {
                                    key: vec!["Display".to_string(), "Os".to_string(), "Ram".to_string()],
                                    value: vec!["14inch".to_string(), "Linux".to_string() , "2Gb".to_string()]
                   }.to_json());
}

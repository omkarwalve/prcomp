                            
//  __  __  __  __  __  ______  
// |  |/ / |  \/  \|  ||   ___| 
// |     \ |     /\   | `-.`-.  
// |__|\__\|____/  \__||______| 
//                   kws/mod.rs
//                   - The kilowog core scraper/crawler(formerly nabu).

#[allow(deprecated)]

use crate::orel;
use crate::types;
use crate::log;
use std::{
    time::Duration,
    sync::mpsc::{Sender,Receiver,channel},
    sync::{ Arc, Mutex}
};
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
const WAIT_FOR_RESPONSE_TIMEOUT : Duration = Duration::from_secs(5);
const USER_AGENT_POOL : [&'static str; 7] = [
                                              "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                              "Mozilla/5.0 (Windows NT 10.0; Trident/7.0; rv:11.0) like Gecko",
                                              //"Mozilla/4.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
                                              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Edg/92.0.902.78",
                                              //"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                              //"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                              //"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Vivaldi/4.1",
                                            ];
//const PROXY_POOL : [&'static str; 10 ] = [
                                           //"103.92.114.6:8080",
                                           //"202.62.67.209:53281",
                                           //"117.242.36.205:42252",
                                           //"182.72.150.242:8080",
                                           //"103.251.214.167:6666",
                                           //"103.216.82.20:6666",
                                           //"14.139.57.195:23500",
                                           //"43.241.141.27:35101",
                                           //"117.241.98.189:44643",
                                           //"103.216.82.18:6666",
                                         //];
const NOT_FOUND_MESSAGE : &'static str = "❔";
const CONFIG_ERROR_MESSAGE : &'static str = "❗CONFIGURATION-ERROR❗";
const FAKE_RESPONSE : &'static str = "<h1>REQUEST FAILED</h1>";
/// Number of concurrent requests to be made. Always `< 1`. 
/// No point going above since cannot make more requests than the `size` of `URL` pool.
const REQUEST_FACTOR : usize = 1;

macro_rules! mkvec {
    ($node: expr) => { $node.into_iter()
                             .map(|node| node.text().trim().to_string())
                             .collect::<Vec<String>>()
    }
}

macro_rules! clean {
    ($text: expr, "wty") => { $text.replace("Know More","") };
    ($text: expr, "rep") => { $text.replace("?","") }
}

pub fn make_request(url: &str) -> Result<String,Error>{
    log!("c",format!("Making Request to {}",url));
    let n_agent: Agent = AgentBuilder::new()
                              .timeout(WAIT_FOR_RESPONSE_TIMEOUT)
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
    log!("gl",format!("Acquired Response ↣ {}",url));
    Ok(http_response)
}

/// ## Stage One :: `Synchronous`
/// Level 1 depth data collection.
pub fn s1<'t>(html_response: &str, website_profile: &'t orel::Orel<String>) -> Option<(Vec<types::Listing<String>>,&'t orel::Orel<String>)> {
    log!("c",r#"
 ____   __  
/ ___) /  \ 
\___ \(_/ / 
(____/ (__) "#);
    let html_document = Document::from(html_response);
    let mut plistings : Vec<types::Listing<String>> = Vec::new();
    log!("c","Finding nodelist in DOM..");
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
            _ => { print!("Failed to build iterator |"); None },
        };
    let mut id_counter: u8 = 0;
    if listing_iterator.is_some() {
        log!("c","Iterating through matched nodelist..");
        for lnode in listing_iterator.unwrap() {
            let mut plisting: types::Listing<String> = Default::default();
            //-- URL
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
            plisting.img = lnode.find(Class(&website_profile.image_identifier[..]))
                                .next().unwrap().attr("src").unwrap().to_string();
            //-- PRODUCT NAME
            plisting.name
                = match website_profile.product_name_find_by.as_str() {
                    "Class" => lnode.find(Class(&website_profile.product_name_identifier[..]))
                                     .next().unwrap().text().replace("\"","\\\""),
                    "Class.alt" => { 
                        let classes = &website_profile.product_name_identifier.split_once(ALT_CHAR).expect("Inseperable Class.alt.pnameident");
                        let find1 = lnode.find(Class(classes.0)).next();
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
            plisting.id = format!("{}#{:#02X}",website_profile.name.clone()[0..2].to_string(),id_counter);
            id_counter +=1;
            println!("━━━━[ {} ]━━━━\n┆ [{}...] :: [{}...] :: [{}...] :- {} ┆",
                     plisting.store,
                     &plisting.name[..20],
                     &plisting.url[..25],
                     &plisting.img[..25],
                     plisting.price);
            plistings.push(plisting);
        }
        println!("━━━━━━━━━━");
        Some((plistings,website_profile))
    }
    else { 
        // println!("No listings element found in {}", &website_profile.name); 
        log!("r",format!("No root node ↣ {}", &website_profile.name)); 
           None
    }
}

/// ## Request :: `Async`
/// Asynchronous requests using `reqwest`.
/// Takes a list of url's and returns a list of `Document`[`select`] type.
async fn make_requests(urls: Vec<String>) -> Result<Vec<select::document::Document>, Box<dyn std::error::Error>> {
    let concurrency: usize = urls.len() * REQUEST_FACTOR;
    let mut headers = reqwest::header::HeaderMap::new();
          headers.insert("Accept", reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
          headers.insert("Accept-Language", reqwest::header::HeaderValue::from_static("en-us"));
          headers.insert("Cache-Control", reqwest::header::HeaderValue::from_static("no-cache"));
        //   headers.insert("Connection", reqwest::header::HeaderValue::from_static("keep-alive"));
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
                  .timeout(WAIT_FOR_RESPONSE_TIMEOUT)
                  .build().unwrap();

    let (a_tx, a_rx) = channel();
    // let (a_tx, a_rx): (Sender<String>,Receiver<String>) = channel();
    let mut responses: Vec<Document> = Vec::new();
    const MPSC_SEND_FAILURE: &str = "MPSC-SEND-FAILURE:MKRQS:: Could not send response from async block";
    // const MPSC_RECV_FAILURE: &str = "MPSC-RECV-FAILURE:MKRQS:: Could not recieve response from async block";

    let _all_the_responses = stream::iter(urls)
                            .map(|url| {
                                         let lc_nclient = &nclient;
                                         async move {
                                             lc_nclient.get(url).send().await?.text().await
                                         }
                                       }
                                )
                            .buffered(concurrency)
                            .for_each(|resp| async {
                                let a_tx_local = a_tx.clone();
                                match resp {
                                    // Ok(res) => response_vec.lock().unwrap().push(Document::from(res.as_str())),
                                    Ok(res) => a_tx_local.send(Some(res)).expect(MPSC_SEND_FAILURE),
                                    Err(e) => { 
                                        log!("r",format!("ASYNC-REQUEST-FAILURE::{}",e));
                                        a_tx_local.send(None).expect(MPSC_SEND_FAILURE);
                                        // println!("{}",Color::Red.bold().paint(format!("Async Request failed\tReason: {}",e)));
                                        // response_vec.lock().unwrap().push(Document::from(FAKE_RESPONSE))
                                    },
                                }
                            }).await;
    // let output = a_rx.recv()
    //                  .into_iter()
    //                  .filter_map(|st| match st { 
    //                      Some(stg) => Some(Document::from(stg.as_str())),
    //                      None => None 
    //                  }).collect();
    // .filter(|st| st.is_some()).map(|st| Document::from(st.some())).collect();
    while let Ok(string) = a_rx.recv_timeout(WAIT_FOR_RESPONSE_TIMEOUT) {
        if let Some(stg) = string { responses.push(Document::from(stg.as_str())) }
        else { responses.push(Document::from(FAKE_RESPONSE)) }
    }

    // let response_vec: Arc<Mutex<Vec<select::document::Document>>> = Arc::new(Mutex::new(Vec::new()));

    // let output = response_vec.lock().expect("MAKE-REQUESTS::Failed to acquire lock on mutex");

    Ok(responses)
}

/// ## Stage Two :: `Internally Async`
/// Level 2 depth data collection.
pub fn s2(tupie : Option<(Vec<types::Listing<String>>, &orel::Orel<String>)>) -> Option<Vec<types::Listing<String>>> {
    log!("c",r#"
  ____  ____ 
 / ___)(___ \
 \___ \ / __/
 (____/(____)"#);
  if tupie.is_some() {
    let (mut listings, profile ) = tupie?;
    let urls: Vec<String> = listings.iter().map(|listing| listing.url.clone()).collect();
    let product_pages = tokio::runtime::Runtime::new().ok()?.block_on(make_requests(urls)).ok()?;
    let mut count: usize = 0;
    for listing in listings.iter_mut() {
        println!("S2 ↣ {} ↣ {}...", &listing.store, &listing.name[..25]);
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
        log!("g","REPL ");
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
        log!("g","WRT ");
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
                                                                                                                 }
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                              }
                                                                                             },
                                                                        _ => types::Spectable::default(),
                                                                      },
                                                        // None => format!("{}", NOT_FOUND_MESSAGE) },
                                                        None =>  types::Spectable::default() },
                "Class.d" => match product_pages[count].find(Class(&profile.product_specs_identifier[..])
                                                     .descendant(Name(&profile.product_specs_idescendant[..])))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str(),
                                                                             profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                              }
                                                                                             },
                                                                        _ => types::Spectable::default(),
                                                                      },
                                                        None =>  types::Spectable::default() },
                "Class" => match product_pages[count].find(Class(&profile.product_specs_identifier[..]))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str(),
                                                                             profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                               }
                                                                                             },
                                                                        _ => types::Spectable::default(),
                                                                      },
                                                        None =>  types::Spectable::default() },
                "Attr" => match product_pages[count].find(Attr(&profile.product_specs_identifier[..],
                                                       &profile.product_specs_ivalue[..]))
                                              .next() { Some(node) => match (profile.product_specs_key_find_by.as_str(),
                                                                             profile.product_specs_val_find_by.as_str()) {
                                                                        ("Class","Class") => { types::Spectable { key : mkvec!(node.find(Class(&profile.product_specs_keyi[..]))),
                                                                                                                  value : mkvec!(node.find(Class(&profile.product_specs_vali[..])))
                                                                                                                 }
                                                                                             },
                                                                        ("Attr","Attr") => { types::Spectable { key : mkvec!(node.find(Attr(&profile.product_specs_keyi[..]
                                                                                                                                    ,&profile.product_specs_keyv[..]))
                                                                                                                            ),
                                                                                                                value : mkvec!(node.find(Attr(&profile.product_specs_vali[..],
                                                                                                                                         &profile.product_specs_valv[..])))
                                                                                                               }
                                                                                             },
                                                                        _ => types::Spectable::default(),
                                                                      },
                                                    None =>  types::Spectable::default() },
                // _ => CONFIG_ERROR_MESSAGE.to_string()
                _ => types::Spectable::default()
        };
        log!("g","SPEC\n");
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
    println!("{:#?}", types::Spectable {
                                    key: vec!["Display".to_string(), "Os".to_string(), "Ram".to_string()],
                                    value: vec!["14inch".to_string(), "Linux".to_string() , "2Gb".to_string()]
                   });
}
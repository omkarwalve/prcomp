// scrape/mod.rs
//      The core scraper - nabu.

#[allow(deprecated)]
#[allow(unused_imports)]
use ureq::{get,Error};
//use regex::Regex;
//use html5ever;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
mod types;

pub fn make_request(url: &str) -> Result<String,ureq::Error>{
    let http_response: String = ureq::get(url)
                                      .set("User-Agent","Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15")
                                      .call()?
                                      .into_string()?;
    Ok(http_response)
}

pub fn stage_one(html_response: &str) {
    let html_document = Document::from(html_response);
    //println!("{:#?}",html_document);
    // -- Amazon Study
    // Image tag
    //   <img class="s-image" src="https://m.media-amazon.com/images/I/81lCHTRxa4S._AC_UY218_.jpg" srcset="https://m.media-amazon.com/images/I/81lCHTRxa4S._AC_UY218_.jpg 1x, https://m.media-amazon.com/images/I/81lCHTRxa4S._AC_UY327_FMwebp_QL65_.jpg 1.5x, https://m.media-amazon.com/images/I/81lCHTRxa4S._AC_UY436_FMwebp_QL65_.jpg 2x, https://m.media-amazon.com/images/I/81lCHTRxa4S._AC_UY545_FMwebp_QL65_.jpg 2.5x, https://m.media-amazon.com/images/I/81lCHTRxa4S._AC_UY654_FMwebp_QL65_.jpg 3x" alt="realme X7 (Space Silver, 6GB RAM, 128GB Storage) with No Cost EMI/Additional Exchange Offers" data-image-index="2" data-image-load="" data-image-latency="s-product-image" data-image-source-density="1">
    // Product Name and URL
    //   <a class="a-link-normal a-text-normal" target="_blank" href="/Renewed-Realme-Nebula-Storage-Without/dp/B097Q2C6B3/ref=sr_1_4?crid=13WS9OJ4SFKDS&amp;dchild=1&amp;keywords=realme+x7&amp;qid=1629094995&amp;refinements=p_89%3ANokia%7Crealme&amp;rnid=3837712031&amp;s=electronics&amp;sprefix=realme+x%2Caps%2C331&amp;sr=1-4"><span class="a-size-medium a-color-base a-text-normal">(Renewed) Realme X7 (Nebula, 8GB RAM, 128GB Storage) Without Offer</span> </a>
    // Product Price
    //   <span class="a-price" data-a-size="l" data-a-color="price"><span class="a-offscreen">₹20,999</span><span aria-hidden="true"><span class="a-price-symbol">₹</span><span class="a-price-whole">20,999</span></span></span>
    for lnode in html_document.find(Attr("data-component-type", "s-search-result")) {
	let pimg = lnode.find(Class("s-image")).next().unwrap().attr("src").unwrap();
	let product_name_url = lnode.find(Class("a-text-normal")).next().unwrap();
	let price = match lnode.find(Attr("data-a-color", "price")).next() { Some(node) => node.first_child().unwrap().text(), None => "Not Available".to_string(), };
	let site_root = "https://www.amazon.in";
	//{ Some(node) => node, None => panic!("NO LINKS FOUND"), };
	println!("==== Found ====\n-PRODUCT: {}\n-URL: {}{}\n-IMG.SRC :-{}\nPRICE: {}",product_name_url.text(), site_root, product_name_url.attr("href").unwrap(),pimg, price);
    }
}

#[test]
fn mans_gotta_test() {
    assert_ne!(make_request("https://www.amazon.in/s?k=realme+x7").unwrap().bytes().count(),0);
}

// scrape/types.rs
//         - Data structures for nabu.
use std::collections::HashMap;

// HTML Attribute-Value pairs
struct Attribute<'t> {
    attribute: &'t str,
    value: &'t str
}

// HTML Tag
struct Tag<'t>{
    element: &'t str,
    attributes: Vec<Attribute<'t>>,
}

// HTML Node
struct Node<'t> {
    open_tag: Tag<'t>,
    content: &'t str,
    close_tag: Tag<'t>,
}

// Document type?
enum Document<'t> {
    Nodes(Node<'t>),
    NonHTML
}

// ===== OREL ======
struct Orel<'t> {
    name: &'t str,
    root_uri: &'t str,
    query_space: &'t str,
    uri_seperator: &'t str,

    listing_find_by: &'t str,
    listing_attr: &'t str,
    listing_value: &'t str,

    image_find_by: &'t str,

    product_url_find_by: &'t str,
    product_url_attr: &'t str,
    product_url_value: &'t str,

    product_name_find_by: &'t str,
    product_name_attr: &'t str,
    product_name_value: &'t str,

    product_price_find_by: &'t str,
    product_price_attr: &'t str,
    product_price_value: &'t str
}

// -- Stage One --
struct Listing<'t> {
    pname: &'t str,
    price: &'t f32,
    img_src: &'t str,
    prl: &'t str
}

// -- Stage Two --
struct Product<'t> {
    return_replace: &'t str,
    warranty: &'t str,
    specs: &'t str,
}

pub type Profile = HashMap<String,String>;

pub trait JSONize {
    fn to_json(&self) -> String;
}

//<table id="productDetails_techSpec_section_1" class="a-keyvalue prodDetTable" role="presentation">

impl JSONize for Listing<'_> {
    fn to_json(&self) -> String {
        format!("{{ NAME : {}
                    PRICE: {}
                    IMG: {}
                    URL: {}
                 }}", self.pname,self.price,self.img_src,self.prl)
    }
}

impl JSONize for Product<'_> {
    fn to_json(&self) -> String {
        format!("{{ RETURN_REPLACE : {}
                    WARRANTY : {}
                    SPECS : {}
                 }}", self.return_replace,self.warranty,self.specs)
    }
}

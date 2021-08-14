// scrape/types.rs
//         - Data structures for nabu.

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

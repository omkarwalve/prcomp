// scrape/types.rs
//         - Data structures for nabu.

// HTML Attribute-Value pairs
//struct Attribute<'t> {
    //attribute: &'t str,
    //value: &'t str
//}

//// HTML Tag
//struct Tag<'t>{
    //element: &'t str,
    //attributes: Vec<Attribute<'t>>,
//}

//// HTML Node
//struct Node<'t> {
    //open_tag: Tag<'t>,
    //content: &'t str,
    //close_tag: Tag<'t>,
//}

//// Document type?
//enum Document<'t> {
    //Nodes(Node<'t>),
    //NonHTML
//}

#[derive(Default)]
pub struct Listing<T> { 
    pub name: T,
    pub store: T,
    pub return_replace: T,
    pub warranty: T,
    pub specs: T,
    pub price: T,
    pub img: T,
    pub url: T,
}

pub struct Listings<T> {
    query: T,
    category: T,
    date_time: T,
    listings: Vec<Vec<Listing<T>>>,
}

pub trait JSONize {
    fn to_json(&self) -> String;
}

impl JSONize for Listing<String> {
    fn to_json(&self) -> String {
        format!("{{\n
                    \"NAME\" : \"{}\",\n
                    \"STORE\" : \"{}\",\n
                    \"RET_POLICY\" : \"{}\",\n
                    \"WARRANTY\" : \"{}\",\n
                    \"SPECS\" : \"{}\",\n
                    \"PRICE\" : \"{}\",\n
                    \"IMG\" : \"{}\",\n
                    \"URL\" : \"{}\",\n
                }}\n",self.name,
                      self.store,
                      self.return_replace,
                      self.warranty,
                      self.specs,
                      self.price,
                      self.img,
                      self.url)
    }
}

impl JSONize for Listings<String> {
    fn to_json(&self) -> String {
        format!("{{\n
                    \"DATE\" : \"{}\",\n
                    \"CATEGORY\" : \"{}\",\n
                    \"QUERY\" : \"{}\",\n
                    \"RESULTS\" : {{ {}\n\t}}
                }}\n",self.date_time,
                      self.category,
                      self.query,
                      self.listings.iter().map(|listing| for product in listing.iter() {
                                                                return product.to_json();
                                                         }
                                              ).collect::<Vec<String>>().join(",")
    }
}

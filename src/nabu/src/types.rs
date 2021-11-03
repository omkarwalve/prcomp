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
use rocket::serde::{ Serialize, Deserialize };

#[derive(Serialize,Deserialize)]
#[derive(Default,Debug,Clone)]
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

#[derive(Serialize)]
pub struct Listings<T> {
    pub query: T,
    pub category: T,
    pub date_time: T,
    pub listings: Vec<Listing<T>>,
}

#[derive(Default)]
pub struct Spectable<T> {
   pub key: Vec<T>,
   pub value: Vec<T>
}

pub trait JSONize {
    fn to_json(&self) -> String;
}

impl JSONize for Listing<String> {
    fn to_json(&self) -> String {
        format!("{{
                    \"NAME\" : \"{}\",
                    \"STORE\" : \"{}\",
                    \"RET_POLICY\" : \"{}\",
                    \"WARRANTY\" : \"{}\",
                    \"SPECS\" :  {} ,
                    \"PRICE\" : \"{}\",
                    \"IMG\" : \"{}\",
                    \"URL\" : \"{}\"
                }}",self.name,
                      self.store,
                      self.return_replace.trim(),
                      self.warranty.trim(),
                      { let spec = self.specs.trim().replace("\n",""); if spec.chars().count() < 2 { "\"❔\"".to_string() } else { spec } },
                      self.price,
                      self.img,
                      self.url)
    }
}

impl JSONize for Listings<String> {
    fn to_json(&self) -> String {
        format!("{{
                    \"DATE\" : \"{}\",
                    \"CATEGORY\" : \"{}\",
                    \"QUERY\" : \"{}\",
                    \"RESULTS\" : [ {} ]
                }}",self.date_time,
                      self.category,
                      self.query,
                      self.listings_to_json().join(",")
                      )
    }
}

impl Listings<String>{ 
    pub fn listings_to_json(&self) -> Vec<String> { 
        let mut product_json : Vec<String> = Vec::new();
        for listing in self.listings.iter() {
            //for product in listing.iter() { 
                product_json.push(listing.to_json())
            
        }
        product_json
    }
}

impl JSONize for Spectable<String> {
    fn to_json(&self) -> String {
        let mut jsonized_kv_pairs: Vec<String> = Vec::new();
        for i in 0..self.len() {
            jsonized_kv_pairs.push(format!("\"{}\" : \"{}\"", { let x = self.key[i].replace("\"","\\\""); x.replace("\n"," ") }, { let x = self.value[i].replace("\"","\\\""); x.replace("\n"," ") } ));
        }
        format!("{{ {} }}", jsonized_kv_pairs.join(","))
    }
}

impl Spectable<String> {
    //fn from_vec(&self,key_vec: Vec<String>, value_vec: Vec<String>) -> Spectable<String> {
    //}
    fn len(&self) -> usize {
        self.key.len()
    }
}

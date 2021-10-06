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

pub struct Listings<'t, T> {
    pub query: T,
    pub category: T,
    pub date_time: T,
    pub listings: &'t Vec<Vec<Listing<T>>>,
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
        format!("{{\n
                    \"NAME\" : \"{}\",\n
                    \"STORE\" : \"{}\",\n
                    \"RET_POLICY\" : \"{}\",\n
                    \"WARRANTY\" : \"{}\",\n
                    \"SPECS\" :  {} ,\n
                    \"PRICE\" : \"{}\",\n
                    \"IMG\" : \"{}\",\n
                    \"URL\" : \"{}\"\n
                }}\n",self.name,
                      self.store,
                      self.return_replace.trim(),
                      self.warranty.trim(),
                      self.specs.trim().replace("\n",""),
                      self.price,
                      self.img,
                      self.url)
    }
}

impl JSONize for Listings<'_, String> {
    fn to_json(&self) -> String {
        format!("{{\n
                    \"DATE\" : \"{}\",\n
                    \"CATEGORY\" : \"{}\",\n
                    \"QUERY\" : \"{}\",\n
                    \"RESULTS\" : [ {}\n\t]
                }}\n",self.date_time,
                      self.category,
                      self.query,
                      self.listings_to_json().join(",\n")
                      )
    }
}

impl Listings<'_, String>{ 
    pub fn listings_to_json(&self) -> Vec<String> { 
        let mut product_json : Vec<String> = Vec::new();
        for listing in self.listings.iter() {
            for product in listing.iter() { 
                product_json.push(product.to_json())
            }
        }
        product_json
    }
}

impl JSONize for Spectable<String> {
    fn to_json(&self) -> String {
        let mut jsonized_kv_pairs: Vec<String> = Vec::new();
        for i in 0..self.len() {
            jsonized_kv_pairs.push(format!("\"{}\" : \"{}\"", self.key[i],self.value[i]));
        }
        format!("{{ {} }}",jsonized_kv_pairs.join(",\n"))
    }
}
impl Spectable<String> {
    //fn from_vec(&self,key_vec: Vec<String>, value_vec: Vec<String>) -> Spectable<String> {
    //}
    fn len(&self) -> usize {
        self.key.len()
    }
}

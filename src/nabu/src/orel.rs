//    ╔═╗╦═╗╔═╗╦  
//    ║ ║╠╦╝║╣ ║  
//    ╚═╝╩╚═╚═╝╩═╝
//         OBJECT-RELATIONS(OREL) FILE PARSER
//           - A simplistic json,yaml,toml,etc like configuration syntax

use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Orel<T> {
    name: T,
    root_uri: T,
    query_space: T,
    uri_seperator: T,

    listing_find_by: T,
    listing_attr: T,
    listing_value: T,

    image_find_by: T,

    product_url_find_by: T,
    product_url_attr: T,
    product_url_value: T,

    product_name_find_by: T,
    product_name_attr: T,
    product_name_value: T,

    product_price_find_by: T,
    product_price_attr: T,
    product_price_value: T
}

impl Orel<String> {
    fn catch<'t>(&mut self, var: String, val: String) -> Option<String> { 
        if var == "NAME" { self.name = val; None }
        else if var == "ROOT_URI" { self.root_uri = val; None}
        else if var == "QUERY_SPACE" { self.query_space = val; None}
        else if var == "URI_SEPERATOR" { self.uri_seperator = val; None}

        else if var == "LISTING_FIND_BY" { self.listing_find_by = val; None }
        else if var == "LISTING_ATTR" { self.listing_attr = val; None}
        else if var == "LISTING_ATTR_VALUE" { self.listing_value = val; None }

        else if var == "IMG_FIND_BY" { self.image_find_by = val; None }

        else if var == "PNAME_FIND_BY" { self.product_name_find_by = val; None }
        else if var == "PNAME_ATTR" { self.product_name_attr = val; None }
        else if var == "PNAME_VALUE" { self.product_name_value = val; None }

        else if var == "PRICE_FIND_BY" { self.product_price_find_by = val; None }
        else if var == "PRICE_ATTR" { self.product_price_attr = val; None }
        else if var == "PRICE_ATTR_VALUE" { self.product_price_value = val; None }

        else if var == "PURL_FIND_BY" { self.product_url_find_by = val; None }
        else if var == "PURL_ATTR" { self.product_url_attr = val; None }
        else if var == "PURL_ATTR_VALUE" { self.product_url_value = val; None }
        else { Some(var) }
    }

    fn pretty_print(&self) { 
        println!("name: {}\n
                  root_uri: {}\n
                  query_space: {}\n
                  uri_sep: {}\n
                  listing_find_by: {}\n
                  listing_attr: {}\n
                  listing_at_val: {}\n
                  image_find_by: {}\n
                  pname_fby: {}\n
                  pname_attr: {}\n
                  pname_val: {}\n
                  pprice_fby: {}\n
                  pprice_attr: {}\n
                  pprice_val: {}\n
                  purl_fby: {}\n
                  purl_attr: {}\n
                  purl_val: {}",
                  self.name,
                  self.root_uri,
                  self.query_space,
                  self.uri_seperator,
                  self.listing_find_by,
                  self.listing_attr,
                  self.listing_value,
                  self.image_find_by,
                  self.product_name_find_by,
                  self.product_name_attr,
                  self.product_name_value,
                  self.product_price_find_by,
                  self.product_price_attr,
                  self.product_price_value,
                  self.product_url_find_by,
                  self.product_url_attr,
                  self.product_url_value);
    }
}

pub fn parse_orel(orel_file_path: &String) -> Orel<String> {
    let orelfile = match File::open(orel_file_path) {
        Err(why) => panic!("Couldn't open {} ,\nbecause {}", orel_file_path,why),
        Ok(file) => file,
    };
    let mut orelcfg : Orel<String> = Default::default();
    for line in BufReader::new(orelfile).lines().map(|loine| loine.unwrap()) {
        let var_bracket: (_,_) = (match line.find('[') { Some(index)=> index,None => 0 }, match line.find(']'){ Some(index)=> index, None => 0 });
        let val_bracket: (_,_) = (match line.find('{') { Some(index)=> index,None => 0 }, match line.find('}'){ Some(index)=> index, None => 0 });
        orelcfg.catch(line[var_bracket.0+1..var_bracket.1].to_string(),line[val_bracket.0+1..val_bracket.1].to_string());
    }
    orelcfg
}

// == Main Function - Only for testing it out as a binary ==
//fn main() {
    //let arguments: Vec<String> = std::env::args().collect();
    ////println!("{:#?}",parse_orel("./orel_format"));
    //parse_orel(&arguments[1]).pretty_print();
//}

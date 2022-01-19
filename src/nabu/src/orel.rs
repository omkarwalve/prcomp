//        ╔═╗╦═╗╔═╗╦  
//        ║ ║╠╦╝║╣ ║  
//        ╚═╝╩╚═╚═╝╩═╝
//             Object-Relations(OREL) File
//               - A simplistic json,yaml,toml,etc like configuration syntax

use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Orel<T> {
    pub name: T,
    pub root_uri: T,
    pub query_cmd: T,
    pub uri_seperator: T,

    pub listing_find_by: T,
    pub listing_identifier: T,
    pub listing_ivalue: T,

    pub image_find_by: T,
    pub image_identifier: T,

    pub product_url_find_by: T,
    pub product_url_identifier: T,
    pub product_url_ivalue: T,

    pub product_name_find_by: T,
    pub product_name_identifier: T,
    pub product_name_ivalue: T,

    pub product_price_find_by: T,
    pub product_price_identifier: T,
    pub product_price_ivalue: T,

    pub product_return_policy_find_by: T,
    pub product_return_policy_identifier: T,
    pub product_return_policy_ivalue: T,
    pub product_return_policy_idescendant: T,

    pub product_warranty_find_by: T,
    pub product_warranty_identifier: T,
    pub product_warranty_ivalue: T,
    pub product_warranty_idescendant: T,

    pub product_specs_find_by: T,
    pub product_specs_identifier: T,
    pub product_specs_ivalue: T,
    pub product_specs_idescendant: T,
    pub product_specs_key_find_by: T,
    pub product_specs_keyi: T,
    pub product_specs_keyv: T,
    pub product_specs_val_find_by: T,
    pub product_specs_vali: T,
    pub product_specs_valv: T,
}

impl Orel<String> {
    fn catch<'t>(&mut self, var: String, val: String) -> Option<String> { 
        if var == "NAME" { self.name = val; None }
        else if var == "ROOT_URI" { self.root_uri = val; None}
        else if var == "QUERY_CMD" { self.query_cmd = val; None}
        else if var == "URI_SEPERATOR" { self.uri_seperator = val; None}

        else if var == "LISTING_FIND_BY" { self.listing_find_by = val; None }
        else if var == "LISTING_IDENTIFIER" { self.listing_identifier = val; None}
        else if var == "LISTING_IDENTIFIER_VALUE" { self.listing_ivalue = val; None }

        else if var == "IMG_FIND_BY" { self.image_find_by = val; None }
        else if var == "IMG_IDENTIFIER" { self.image_identifier = val; None }

        else if var == "PNAME_FIND_BY" { self.product_name_find_by = val; None }
        else if var == "PNAME_IDENTIFIER" { self.product_name_identifier = val; None }
        else if var == "PNAME_IDENTIFIER_VALUE" { self.product_name_ivalue = val; None }

        else if var == "PRICE_FIND_BY" { self.product_price_find_by = val; None }
        else if var == "PRICE_IDENTIFIER" { self.product_price_identifier = val; None }
        else if var == "PRICE_IDENTIFIER_VALUE" { self.product_price_ivalue = val; None }

        else if var == "PURL_FIND_BY" { self.product_url_find_by = val; None }
        else if var == "PURL_IDENTIFIER" { self.product_url_identifier = val; None }
        else if var == "PURL_IDENTIFIER_VALUE" { self.product_url_ivalue = val; None }

        else if var == "RETPOL_FIND_BY" { self.product_return_policy_find_by = val; None }
        else if var == "RETPOL_IDENTIFIER" { self.product_return_policy_identifier = val; None }
        else if var == "RETPOL_IDENTIFIER_VALUE" { self.product_return_policy_ivalue = val; None }
        else if var == "RETPOL_DESCENDANT" { self.product_return_policy_idescendant = val; None }

        else if var == "WARRANTY_FIND_BY" { self.product_warranty_find_by = val; None }
        else if var == "WARRANTY_IDENTIFIER" { self.product_warranty_identifier = val; None }
        else if var == "WARRANTY_IDENTIFIER_VALUE" { self.product_warranty_ivalue = val; None }
        else if var == "WARRANTY_DESCENDANT" { self.product_warranty_idescendant = val; None }

        else if var == "SPECS_FIND_BY" { self.product_specs_find_by = val; None }
        else if var == "SPECS_IDENTIFIER" { self.product_specs_identifier = val; None }
        else if var == "SPECS_IDENTIFIER_VALUE" { self.product_specs_ivalue = val; None }
        else if var == "SPECS_DESCENDANT" { self.product_specs_idescendant = val; None }
        else if var == "SPECS_KEY_FIND_BY" { self.product_specs_key_find_by = val; None }
        else if var == "SPECS_KEY_IDENTIFIER" { self.product_specs_keyi = val; None }
        else if var == "SPECS_KEY_IDENTIFIER_VALUE" { self.product_specs_keyv = val; None }
        else if var == "SPECS_VALUE_FIND_BY" { self.product_specs_val_find_by = val; None }
        else if var == "SPECS_VALUE_IDENTIFIER" { self.product_specs_vali = val; None }
        else if var == "SPECS_VALUE_IDENTIFIER_VALUE" { self.product_specs_valv = val; None }

        else { Some(var) }
    }

    pub fn pretty_print(&self) { 
        println!("\tname: {}\n\troot_uri: {}\n\tquery_cmd: {}\n\turi_sep: {}\n\tlisting_find_by: {}\n\tlisting_identifier: {}\n\tlisting_at_val: {}\n\timage_find_by: {}\n\timage_identifier: {}\n\tpname_fby: {}\n\tpname_identifier: {}\n\tpname_val: {}\n\tpprice_fby: {}\n\tpprice_identifier: {}\n\tpprice_val: {}\n\tpurl_fby: {}\n\tpurl_identifier: {}\n\tpurl_val: {}\n\tret_pol_fby: {}\n\tret_pol_iden: {}\n\tret_pol_ival: {}\n\tret_pol_idesc: {}\n\twarranty_fby: {}\n\twarranty_iden: {}\n\twarranty_ival: {}\n\twarranty_idesc: {}\n\tspecs_fby: {}\n\tspecs_iden: {}\n\tspecs_ival: {}\n\tspecs_idesc: {}",
                  self.name,
                  self.root_uri,
                  self.query_cmd,
                  self.uri_seperator,
                  self.listing_find_by,
                  self.listing_identifier,
                  self.listing_ivalue,
                  self.image_find_by,
                  self.image_identifier,
                  self.product_name_find_by,
                  self.product_name_identifier,
                  self.product_name_ivalue,
                  self.product_price_find_by,
                  self.product_price_identifier,
                  self.product_price_ivalue,
                  self.product_url_find_by,
                  self.product_url_identifier,
                  self.product_url_ivalue,
                  self.product_return_policy_find_by,
                  self.product_return_policy_identifier,
                  self.product_return_policy_ivalue,
                  self.product_return_policy_idescendant,
                  self.product_warranty_find_by,
                  self.product_warranty_identifier,
                  self.product_warranty_ivalue,
                  self.product_warranty_idescendant,
                  self.product_specs_find_by,
                  self.product_specs_identifier,
                  self.product_specs_ivalue,
                  self.product_specs_idescendant,
                  );
    }
}

pub fn parse_orel(orel_file_path: &String) -> Orel<String> {
    println!("Config: {}",orel_file_path);
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
    //println!("{:#?}",parse_orel("./orel_format"));
    //parse_orel(&arguments[1]).pretty_print();
//}

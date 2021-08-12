// OBJECT-RELATIONS(OREL) FILE PARSER
//                 - A simplistic json,yaml,toml,etc like configuration syntax

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

pub fn parse_orel(orel_file_path: &String) -> HashMap<String,String> {
    let orelfile = match File::open(orel_file_path) {
        Err(why) => panic!("Couldn't open {} ,\nbecause {}", orel_file_path,why),
        Ok(file) => file,
    };
    let mut orelmap = HashMap::new();
    for line in BufReader::new(orelfile).lines().map(|loine| loine.unwrap()) {
        let var_bracket: (_,_) = (match line.find('[') { Some(index)=> index,None => 0 }, match line.find(']'){ Some(index)=> index, None => 0 });
        let val_bracket: (_,_) = (match line.find('{') { Some(index)=> index,None => 0 }, match line.find('}'){ Some(index)=> index, None => 0 });
        orelmap.insert(line[var_bracket.0+1..var_bracket.1].to_string(),line[val_bracket.0+1..val_bracket.1].to_string());
    }
    orelmap
}

// == Main Function - Only for testing it out as a binary ==
//use std::env;
//fn main() {
    //let arguments: Vec<String> = env::args().collect();
    //println!("{:?}",parse_orel(&arguments[1]));
//}

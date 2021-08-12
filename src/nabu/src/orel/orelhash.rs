// Object Relations(orel) File Parser

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::env;

fn parse_orel(orel_file_path: &str) -> HashMap<String,String> {
    let orelfile = match File::open(orel_file_path) {
        Err(why) => panic!("Couldn't open {} , because {}", orel_file_path,why),
        Ok(file) => file,
    };
    //match output_type { 
        //"hash" => Some(orel_hash(BufReader::new(orelfile))),
        //"list" => Some(orel_list(BufReader::new(orelfile))),
        //_ => panic!("Unknown Output Format!"),
    //}
    orel_hash(BufReader::new(orelfile))
    //orel_list(BufReader::new(orelfile))
}

fn orel_hash(ofile: BufReader<File>) -> HashMap<String,String> {
    let mut orelmap = HashMap::new();
    for line in ofile.lines().map(|loine| loine.unwrap()) {
        let var_bracket: (_,_) = (match line.find('[') { Some(index)=> index,None => 0 }, match line.find(']'){ Some(index)=> index, None => 0 });
        let val_bracket: (_,_) = (match line.find('{') { Some(index)=> index,None => 0 }, match line.find('}'){ Some(index)=> index, None => 0 });
        //let val_bracket: (_,_) = (line.find('{'),line.find('}'));
        //println!("Var brackets at: {},{}\nVal Brackets at: {},{}",var_bracket.0,var_bracket.1,val_bracket.0,val_bracket.1);
        //let val_bracket: (Vec<_>,Vec<_>) = (line.match_indices('{').collect::<Vec<(usize,_)>>(),line.match_indices('}').collect::<Vec<(usize,_)>>());
        orelmap.insert(line[var_bracket.0+1..var_bracket.1].to_string(),line[val_bracket.0+1..val_bracket.1].to_string());
    }
    orelmap
}
//fn orel_list(ofile: BufReader<File>) -> Vec<String> {
    //let mut orelvec = Vec::new();
    //for line in ofile.lines().map(|loine| loine.unwrap()) {
        //let var_bracket: (_,_) = (match line.find('[') { Some(index)=> index,None => 0 }, match line.find(']'){ Some(index)=> index, None => 0 });
        //let val_bracket: (_,_) = (match line.find('{') { Some(index)=> index,None => 0 }, match line.find('}'){ Some(index)=> index, None => 0 });
        //orelvec.push(line[var_bracket.0+1..var_bracket.1].to_string());
        //orelvec.push(line[val_bracket.0+1..val_bracket.1].to_string());
    //}
    //orelvec
//}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("{:?}",parse_orel(&arguments[1]));
}

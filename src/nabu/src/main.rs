// Main.rs
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::env;
mod orel;

// == Global Variables
const CATEGORY_DIR: &str = "./cnp/categories";
const PROFILE_DIR: &str = "./cnp/profiles";
const CONFIG_EXTENSION: &str = "orel";

// Common function for opening files
fn fopen(fname: &String) -> File {
    match File::open(fname) {
        Err(why) => panic!("Couldn't open {},\nbecause {}",fname,why),
        Ok(file) => file,
    }
}

// Function that fetches the list of website by category.
fn read_category(file_name: &str) -> Vec<String>{
    let category_file = fopen(&format!("{}/{}",CATEGORY_DIR,file_name));
    let mut website_list = Vec::new();
    for website in BufReader::new(category_file).lines().map(|line| line.unwrap()) {
        website_list.push(format!("{}.{}",website,CONFIG_EXTENSION))
    }
    website_list
}

// Call the program as `nabu <CATEGORY> <SEARCH_QUERY>`
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let category: &String = &arguments[1];
    let search_query: &[String] = &arguments[2..];
    let site_list = read_category(category);
    println!("Category: {}\nQuery: {:?}\nCategory File Says: {:?}", category,search_query,site_list); // Verbose Output
    for site_file in site_list.iter() {
        println!("Website Configuration is:\n{:?}",orel::parse_orel(&format!("{}/{}",PROFILE_DIR,site_file)));
    }
}

// Main.rs
mod orel;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::env;

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

// Function to read the websites profiles
fn read_profiles(website_list: Vec<String>)  -> Vec<HashMap<String,String>> {
    let mut nabu_index: Vec<HashMap<String,String>> = Vec::new();
    for website_file in website_list.iter() {
        //println!("Website Configuration is:\n{:?}",orel::parse_orel(&format!("{}/{}",PROFILE_DIR,website_file)));
        nabu_index.push(orel::parse_orel(&format!("{}/{}",PROFILE_DIR,website_file)));
    }
    nabu_index
}

// Call the program as `nabu <CATEGORY> <SEARCH_QUERY>`
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let category: &String = &arguments[1];
    let search_query: &[String] = &arguments[2..];
    //let site_list = read_category(category);
    println!("Category: {}\nQuery: {:?}\nCategory File Says: {:#?}", category,search_query,read_category(category)); // Verbose Output
    println!("Website Configuration is:\n{:#?}",read_profiles(read_category(category)));
}

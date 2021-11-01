//         __      __  __  ___ __  
//    |  ||__) /\ |__)|__)|__ |__) 
//    |/\||  \/~~\|   |   |___|  \ 
//                      src/wrapper.rs
//                       - A wrapper for the underlying scraper.

use std::{ fs::File , 
           io::{ BufRead, BufReader, Write },
           thread,
           sync::{ Arc, Mutex },
           env,
           rc::Rc,
         };
use crate::orel;
use crate::nabu;
use crate::types::{ Listing , Listings, JSONize };
use ansi_term::Color;
use chrono::offset::Local as time;

#[macro_export]
macro_rules! debug {
    ($format:expr, $($strs:expr),*) => {
        println!($format, $($strs,)*);
    }
}

// == Global Variables
const CONFIG_EXTENSION: &str = "orel";

// Common function for opening files
fn fopen(fname: &String) -> File {
    match File::open(fname) {
        Err(why) => panic!("Couldn't open {},\nbecause {}",fname,why),
        Ok(file) => file,
    }
}

// Function that fetches the list of website by category.
fn read_category(cat_dir: &str, file_name: &str) -> Vec<String>{
    let category_file = fopen(&format!("{}/{}",cat_dir,file_name));
    let mut website_list = Vec::new();
    for website in BufReader::new(category_file).lines().map(|line| line.unwrap()) {
        website_list.push(format!("{}.{}",website,CONFIG_EXTENSION))
    }
    website_list
}

// Function to read website profile
fn read_profile(profile_folder: &str, website_profile: &String) -> orel::Orel<String> {
        //println!("Inside read_profile() function");
        orel::parse_orel(&format!("{}/{}",profile_folder,website_profile))
}

fn make_url(root_url: &String, query_cmd: &String, uri_seperator: &str, query: &String) -> String {
    format!("{}/{}={}",root_url,query_cmd,query.replace(" ", uri_seperator))
}

macro_rules! dir_mode {
    ("raw",$cdir:ident,$pdir:ident) => {
        let $cdir: &str = &format!("{}/src/cnp/categories",env!("CARGO_MANIFEST_DIR"));
        let $pdir: Arc<String> = Arc::new(format!("{}/src/cnp/profiles",env!("CARGO_MANIFEST_DIR")));
    };
    ("bin",$cdir:ident,$pdir:ident) => {
        let current_dir = env::current_dir().unwrap().display().to_string();
        let $cdir: &str = &format!("{}/cnp/categories",current_dir);
        let $pdir: Arc<String> = Arc::new(format!("{}/cnp/profiles",current_dir));
    };
    ("arg",$cdir:ident,$pdir:ident) => {
        let arguments: Vec<String> = std::env::args().collect();
        let category: &String = &arguments[1];
        let search_query = Arc::new(arguments[2..].join(" "));
    };
}

// Call the function as `nabu(CATEGORY,SEARCH_QUERY)`
pub fn nabu_fetch(category: String, query: String) -> Option<crate::types::Listings<String>> {
    dir_mode!("raw",category_dir,profile_dir);
    let search_query = Arc::new(query);
    let site_list = Arc::new(read_category(category_dir,category.as_str()));
    let sites_count = site_list.len();
    let _n_output = Arc::new(Mutex::new(String::new()));
    let listings: Arc<Mutex<Vec<Vec<Listing<String>>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut fetch_handle: Vec<thread::JoinHandle<()>> = Vec::new();
    //println!("Category: {}\nQuery: {}\nWebsites: {:#?}\nSite Count: {}", category,&search_query,&site_list,sites_count); // Verbose Output
    // Spawn a thread for each concurrent website
    for i in 0..sites_count { 
        let squery = Arc::clone(&search_query);
        let slist = Arc::clone(&site_list);
        let listng = Arc::clone(&listings);
        let pdir = Arc::clone(&profile_dir);
        let t_name = Rc::new(slist[i].clone().replace(".orel",""));
        let w_thread = thread::Builder::new().name(t_name.to_string());
        fetch_handle.push(
            w_thread.spawn(move || {
                println!("⌛ Spawned Thread: {}", thread::current().name().expect("Failed to get name"));
                let site_profile = read_profile(&pdir,&slist[i]);
                let results 
                    = nabu::stage_two(nabu::stage_one(match &nabu::make_request(&make_url(&site_profile.root_uri
                                                                                         ,&site_profile.query_cmd
                                                                                         ,&site_profile.uri_seperator
                                                                                         ,&squery)) { 
                                                        Err(why) => stringify!("ERROR::NO_RESPONSE:: Failed to get response from the server.\n
                                                                                Reason: {}\n
                                                                                Kind: {}",why,why.kind()),
                                                        Ok(response) => response }
                                                     ,&site_profile));
                listng.lock().expect("Error acquiring mutex lock").push(results);
                println!("{}",Color::Green.paint(format!("Done scraping from {}", thread::current().name().expect("Failed to get current thread name"))));
            }).expect(&format!("Failed to create thread {}",t_name)));
    }
    //println!("{}",Color::Yellow.paint("Ready to join threads"));
    // Joining all threads to the main thread -- BLOCKING!
    fetch_handle.into_iter()
                .for_each(|thread| 
                          thread.join()
                                .expect(&format!("{}"
                                                , Color::Fixed(202)
                                                        .bold()
                                                        .paint("---- ERROR::THREAD_JOIN_FAILURE ----"))));

    //write_json(category.to_string(),(&search_query).to_string(),listings.lock().unwrap().to_vec());
    thread::sleep(std::time::Duration::from_secs(5));
 
    println!("Ready to pass json");
    let all_listings =  &listings.lock().expect("Failed to acquire lock on listings mutex");
    Some(Listings{ date_time: format!("{}", time::now()),
                          category: category.to_string(),
                          query: (&search_query).to_string(),
                          listings: all_listings.to_vec()
                   })
}

fn write_json(cat: String, query: String, lst: Vec<Vec<Listing<String>>> ) -> () {
    std::fs::File::create("output.json").unwrap().write_all(Listings{ date_time: format!("{}", time::now()),
                                                                             category: cat,
                                                                             query: query,
                                                                             listings: lst
                                                                           }.to_json().as_bytes()).unwrap();
}

#[test]
fn is_url_generated_correctly(){
    assert_eq!(make_url(&"https://urbanladder.com".to_string()
                       ,&"products/search?keywords".to_string()
                       ,"+",&"Queen Size Bed".to_string())
              ,"https://www.urbanladder.com/products/search?keywords=Queen+Size+Bed");
}

//         __      __  __  ___ __  
//    |  ||__) /\ |__)|__)|__ |__) 
//    |/\||  \/~~\|   |   |___|  \ 
//                      src/wrapper.rs
//                       - A wrapper for the underlying scraper.

mod orel;
mod nabu;
mod types;
use types::JSONize;
#[macro_use] extern crate rocket;
//use rocket;
use std::{ fs::File , 
           io::{ BufRead, BufReader },
           thread,
           sync::{ Arc, Mutex },
           //path::{ Path, PathBuf},
           //io::Write
         };
use chrono;

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

#[test]
fn is_url_generated_correctly(){
    assert_eq!(make_url(&"https://urbanladder.com".to_string(),&"products/search?keywords".to_string(),"+",&"Queen Size Bed".to_string()),"https://www.urbanladder.com/products/search?keywords=Queen+Size+Bed");
}

// Call the function as `nabu(CATEGORY,SEARCH_QUERY)`
fn nabu_fetch(category: String, query: String) -> String {
//fn main() {
    //let arguments: Vec<String> = std::env::args().collect();
    let category_dir: &str = &format!("{}/src/cnp/categories",env!("CARGO_MANIFEST_DIR"));
    let profile_dir: Arc<String> = Arc::new(format!("{}/src/cnp/profiles",env!("CARGO_MANIFEST_DIR")));
    //let category: &String = &arguments[1];
    //let search_query = Arc::new(arguments[2..].join(" "));
    let search_query = Arc::new(query);
    let site_list = Arc::new(read_category(category_dir,category.as_str()));
    let sites_count = site_list.len();
    let _n_output = Arc::new(Mutex::new(String::new()));
    let listings: Arc<Mutex<Vec<Vec<types::Listing<String>>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut fetch_handle: Vec<thread::JoinHandle<()>> = Vec::new();
    //println!("Category: {}\nQuery: {}\nCategory File Says: {:#?}\nSite Count: {}", category,&search_query,&site_list,sites_count); // Verbose Output
    // Spawn a thread for each concurrent website
    for i in 0..sites_count { 
        //println!("Inside thread for loop!");
        let squery = Arc::clone(&search_query);
        let slist = Arc::clone(&site_list);
        let listng = Arc::clone(&listings);
        let pdir = Arc::clone(&profile_dir);
        fetch_handle.push(
            //thread::Builder::new().name(site_list[i][..site_list[i].find('.').unwrap()].to_string()).spawn(move || {
            thread::spawn(move || {
                println!("⌛ Spawned Thread: {}",i);
                let site_profile = read_profile(&pdir,&slist[i]);
                println!("☀ Configuration:");
                site_profile.pretty_print();
                //let mut raw_listings = listng.lock().unwrap();
                //println!("{}",make_url(&site_profile.root_uri,&site_profile.query_cmd,&site_profile.uri_seperator,&squery));
                let results 
                    = nabu::stage_two(nabu::stage_one(match &nabu::make_request(&make_url(&site_profile.root_uri,
                                                                                                                  &site_profile.query_cmd,
                                                                                                                  &site_profile.uri_seperator,
                                                                                                                  &squery)) { 
                                                                                 Err(why) => panic!("ERROR::NO_RESPONSE:: Failed to get response from the server.\nReason: {}\nKind: {}",why,why.kind()),
                                                                                 Ok(response) => response },
                                                                              &site_profile));
                listng.lock().unwrap().push(results);
                //raw_listings.push(results);//drop(listings);
            //}).join().unwrap();
            }));
        println!("----------------X-------------------");
    }
    fetch_handle.into_iter().for_each(|thread| thread.join().unwrap());
 
    //std::fs::File::create("output.json").unwrap().write_all(types::Listings{ date_time: format!("{}", chrono::offset::Local::now()),
                                                                             //category: category.to_string(),
                                                                             //query: (&search_query).to_string(),
                                                                             //listings: &listings.lock().unwrap()
                                                                           //}.to_json().as_bytes()).unwrap();
    let all_listings =  &listings.lock().unwrap();
    types::Listings{ date_time: format!("{}", chrono::offset::Local::now()),
                                                                                 category: category.to_string(),
                                                                                 query: (&search_query).to_string(),
                                                                                 listings: all_listings
                                                                               }.to_json()
}

#[get("/")]
fn root() -> &'static str {
    "This is the root of the website. You shouldn't be here :)'"
}

#[get("/elx/<ex_query>")]
fn elx(ex_query: &str) -> String  {
    nabu_fetch("electronics".to_string(),ex_query.replace("+"," "))
}

#[get("/fur/<furn_query>")]
fn fur(furn_query: &str) -> String  {
    nabu_fetch("furniture".to_string(),furn_query.replace("+"," "))
}

#[get("/clo/<cloth_query>")]
fn clo(cloth_query: &str) -> String  {
    nabu_fetch("clothing".to_string(),cloth_query.replace("+"," "))
}
#[rocket::main]
async fn main() {
    //tokio::runtime::Runtime::new()
                            //.unwrap()
                            //.block_on(
                rocket::build()
                       .mount("/",routes![root,elx,fur,clo])
                       .launch()
                       .await;
                                    //);
}

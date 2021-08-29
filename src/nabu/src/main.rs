//         __      __  __  ___ __  
//    |  ||__) /\ |__)|__)|__ |__) 
//    |/\||  \/~~\|   |   |___|  \ 
//                      src/main.rs
//                       - A frontend wrapper for the underlying scraper.

mod orel;
mod nabu;
mod types;
use crate::types::JSONize;
use std::{ fs::File , 
           io::{ BufRead, BufReader },
           thread,
           sync::{ Arc, Mutex }
         };
use chrono;

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
    let category_file = fopen(&format!("{}/{}",&CATEGORY_DIR,file_name));
    let mut website_list = Vec::new();
    for website in BufReader::new(category_file).lines().map(|line| line.unwrap()) {
        website_list.push(format!("{}.{}",website,CONFIG_EXTENSION))
    }
    website_list
}

// Function to read the websites profiles
fn read_profiles(website_list: Vec<String>)  -> Vec<orel::Orel<String>> {
    let mut nabu_index: Vec<orel::Orel<String>> = Vec::new();
    for website_file in website_list.iter() {
        //println!("Website Configuration is:\n{:?}",orel::parse_orel(&format!("{}/{}",PROFILE_DIR,website_file))); // VERBOSE
        nabu_index.push(orel::parse_orel(&format!("{}/{}",&PROFILE_DIR,website_file)));
    }
    nabu_index
}
// Alt Function to parallely read website profiles
fn read_profile(website_profile: &String) -> orel::Orel<String> {
        //println!("Inside read_profile() function");
        orel::parse_orel(&format!("{}/{}",&PROFILE_DIR,website_profile))
}

fn make_url(root_url: &String, query_cmd: &String, uri_seperator: &str, query: &String) -> String {
    format!("{}/{}={}",root_url,query_cmd,query.replace(" ", uri_seperator))
}

#[test]
fn is_url_generated_correctly(){
    assert_eq!(make_url(&"https://urbanladder.com".to_string(),&"products/search?keywords".to_string(),"+",&"Queen Size Bed".to_string()),"https://www.urbanladder.com/products/search?keywords=Queen+Size+Bed");
}

// Call the program as `nabu <CATEGORY> <SEARCH_QUERY>`
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let category: &String = &arguments[1];
    let search_query = Arc::new(arguments[2..].join(" "));
    let site_list = Arc::new(read_category(category));
    let sites_count = site_list.len();
    let _n_output = Arc::new(Mutex::new(String::new()));
    let listings: Arc<Mutex<Vec<Vec<types::Listing<String>>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut fetch_handle: Vec<thread::JoinHandle<()>> = Vec::new();
    //let mut fetch_handle = Vec::new();
    //let (trx: mpsc::Sender<Arc<Mutex<Vec<Vec<types::Listing<String>>>>>>,
        //rcv: mpsc::Receiver<Arc<Mutex<Vec<Vec<types::Listing<String>>>>>>) = mpsc::channel();
    println!("Category: {}\nQuery: {}\nCategory File Says: {:#?}\nSite Count: {}", category,&search_query,&site_list,sites_count); // Verbose Output
    // Spawn a thread for each concurrent website
    for i in 0..sites_count { 
        //println!("Inside thread for loop!");
        let squery = Arc::clone(&search_query);
        let slist = Arc::clone(&site_list);
        let listng = Arc::clone(&listings);
        //println!("Done Cloning arc variables");
        fetch_handle.push(
            //thread::Builder::new().name(site_list[i][..site_list[i].find('.').unwrap()].to_string()).spawn(move || {
            thread::spawn(move || {
                println!("⌛ Spawned Thread: {}",i);
                let site_profile = read_profile(&slist[i]);
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
                                                                              site_profile));
                listng.lock().unwrap().push(results);
                //raw_listings.push(results);//drop(listings);
            //}).join().unwrap();
            }));
        println!("----------------X-------------------");
    }
    //fetch_handle.into_iter().map(|thread| thread.join().unwrap());
 
    for thread in fetch_handle.into_iter() { 
        thread.join().unwrap();
    }
    //let all_listings = *listings.lock().unwrap();

    println!("{}",types::Listings{ date_time: format!("{}", chrono::offset::Local::now()),
                                   category: category.to_string(),
                                   query: (&search_query).to_string(),
                                   listings: &listings.lock().unwrap()
                                 }.to_json());
    //println!("Website Configuration is:\n{:#?}",read_profiles(read_category(category)));
    //println!("{}",scrape::make_request("https://www.amazon.in/s?k=mac+m1").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.flipkart.com/search?q=m1%20macbook").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.myntra.com/kurta-for-men?plaEnabled=false&rf=Price%3A319.0_2240.0_319.0%20TO%202240.0").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.myntra.com").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.ajio.com/search/?text=black+sneakers").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.urbanladder.com/products/search?utf8=%E2%9C%93&keywords=queen+size+bed").unwrap());
}

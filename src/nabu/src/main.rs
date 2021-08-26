//         __      __  __  ___ __  
//    |  ||__) /\ |__)|__)|__ |__) 
//    |/\||  \/~~\|   |   |___|  \ 
//                      src/main.rs
//                       - A frontend wrapper for the underlying scraper.


mod orel;
mod scrape;
use std::{ fs::File , 
           io::{ BufRead, BufReader },
           thread,
           //borrow::Borrow,
           //path::Path,
           sync::{ Arc, Mutex }
         };
//use std::future::Future;

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
    println!("Category: {}\nQuery: {}\nCategory File Says: {:#?}\nSite Count: {}", category,&search_query,&site_list,sites_count); // Verbose Output
    for i in 0..sites_count { // Spawn a thread for each concurrent website
        //println!("Inside thread for loop!");
        let squery = search_query.clone();
        let slist = site_list.clone();
        //println!("Done Cloning arc variables");
        let FetchHandle: thread::JoinHandle<()> 
            = thread::spawn(move || {
                println!("Inside spawned thread!");
                let site_profile = read_profile(&slist[i]);
                println!("pre pretty print now");
                site_profile.pretty_print();
                //println!("{}",make_url(&site_profile.root_uri,&site_profile.query_cmd,&site_profile.uri_seperator,&squery));
                scrape::stage_one(match &scrape::make_request(&make_url(&site_profile.root_uri,&site_profile.query_cmd,&site_profile.uri_seperator,&squery)) { 
                                         Err(why) => panic!("ERROR::NO_RESPONSE:: Failed to get response from the server.\nReason: {}\nKind: {}",why,why.kind()),
                                         Ok(response) => response,
                                  },
                                  site_profile);
            });
        FetchHandle.join().unwrap();
        println!("----------------X-------------------");
    }
    //println!("Website Configuration is:\n{:#?}",read_profiles(read_category(category)));
    //println!("{}",scrape::make_request("https://www.amazon.in/s?k=mac+m1").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.flipkart.com/search?q=m1%20macbook").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.myntra.com/kurta-for-men?plaEnabled=false&rf=Price%3A319.0_2240.0_319.0%20TO%202240.0").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.myntra.com").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.ajio.com/search/?text=black+sneakers").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.urbanladder.com/products/search?utf8=%E2%9C%93&keywords=queen+size+bed").unwrap());
}

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
           path::Path,
           sync::{ Arc, Mutex }
         };
use std::future::Future;

// == Global Variables
const CATEGORY_DIR: &Path = Path::new("./cnp/categories");
const PROFILE_DIR: &Path = Path::new("./cnp/profiles");
const CONFIG_EXTENSION: &str = "orel";

// Common function for opening files
fn fopen(fname: &Path) -> File {
    match File::open(fname) {
        Err(why) => panic!("Couldn't open {},\nbecause {}",fname.display(),why),
        Ok(file) => file,
    }
}

// Function that fetches the list of website by category.
fn read_category(file_name: &str) -> Vec<String>{
    let category_file = fopen(&CATEGORY_DIR.join(file_name));
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
        nabu_index.push(orel::parse_orel(&PROFILE_DIR.join(website_file)));
    }
    nabu_index
}
// Alt Function to parallely read website profiles
fn read_profile(website_profile: &String) -> orel::Orel<String> {
        orel::parse_orel(&PROFILE_DIR.join(website_profile))
}

fn make_url(root_url: &String, query_cmd: &String, uri_seperator: &str, query: &String) -> String {
    format!("{}/{}={}",root_url,query_cmd,query.replace(" ", uri_seperator))
}

// Call the program as `nabu <CATEGORY> <SEARCH_QUERY>`
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let category: &String = &arguments[1];
    let search_query = &arguments[2..].concat();
    let site_list = read_category(category);
    let n_output = Arc::new(Mutex::new(String::new()));
    for i in 0..site_list.len() { // Spawn a thread for each concurrent website
        thread::spawn(move || {
            let site_profile = read_profile(&site_list[i]);
            scrape::stage_one(&scrape::make_request(&make_url(&site_profile.root_uri,&site_profile.query_space,&site_profile.uri_seperator,search_query)).unwrap(),site_profile);
        });
    }
    //println!("Category: {}\nQuery: {:?}\nCategory File Says: {:#?}", category,search_query,read_category(category)); // Verbose Output
    //println!("Website Configuration is:\n{:#?}",read_profiles(read_category(category)));
    //println!("{}",scrape::make_request("https://www.amazon.in/s?k=mac+m1").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.flipkart.com/search?q=m1%20macbook").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.myntra.com/kurta-for-men?plaEnabled=false&rf=Price%3A319.0_2240.0_319.0%20TO%202240.0").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.myntra.com").unwrap());
    //scrape::stage_one(&scrape::make_request("https://www.ajio.com/search/?text=black+sneakers").unwrap());
    scrape::stage_one(&scrape::make_request("https://www.urbanladder.com/products/search?utf8=%E2%9C%93&keywords=queen+size+bed").unwrap());
}

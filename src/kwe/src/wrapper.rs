//         __      __  __  ___ __  
//    |  ||__) /\ |__)|__)|__ |__) 
//    |/\||  \/~~\|   |   |___|  \ 
//                      src/wrapper.rs
//                       - A wrapper for the underlying scraper.
// #![allow(dead_code)]

use std::{ fs::File , 
           io::{ BufRead, BufReader, Write },
           time::Duration,
           thread,
           sync::{ Arc, Mutex, mpsc::channel },
           env,
           rc::Rc,
         };
use crate::orel;
use crate::kws;
use crate::log;
use crate::types::{ Listing , Listings};
use ansi_term::Color;
use chrono::offset::Local as time;
use actix_web::web::Json;
// use rocket_contrib::json::Json;

#[macro_export]
macro_rules! debug {
    ($format:expr, $($strs:expr),*) => {
        println!($format, $($strs,)*);
    }
}

// == Global Variables
const CONFIG_EXTENSION: &str = "orel";
const ENGINE: &str = r#"
 __  __  __  __  __  ______  
|  |/ / |  \/  \|  ||   ___| 
|     \ |     /\   ||   ___| 
|__|\__\|____/  \__||______|"#;

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

// Configuration directory mode
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

/// # KWE Fetch
/// The main interface to the KWE(kilowog-engine).
/// Used to fetch data from targets decided using `category` regarding `query`.
pub fn kwe_fetch(category: &str, query: String) -> Option<crate::types::Listings<String>> {
    println!("{}", Color::Fixed(13).paint(ENGINE));
    dir_mode!("raw",category_dir,profile_dir);
    let search_query = Arc::new(query);
    let site_list = Arc::new(read_category(category_dir,category));
    let sites_count = site_list.len();
    let _n_output = Arc::new(Mutex::new(String::new()));
    let mut fetch_handle: Vec<thread::JoinHandle<()>> = Vec::new();
    // Spawn a thread for each concurrent website
    let (tx, rx) = channel();
    for i in 0..sites_count { 
        let tx_t = tx.clone();
        let squery = Arc::clone(&search_query);
        let slist = Arc::clone(&site_list);
        //let listng = Arc::clone(&listings);
        let pdir = Arc::clone(&profile_dir);
        let t_name = Rc::new(slist[i].clone().replace(".orel","").to_uppercase());
        let w_thread = thread::Builder::new().name(t_name.to_string());
        fetch_handle.push(
            w_thread.spawn(move || {
                let thread_name = thread::current().name().expect("Failed to get current thread name").to_string();
                log!("c",format!("⌛ Spawned Thread: {}", thread_name ));
                let site_profile = read_profile(&pdir,&slist[i]);
                let results 
                    = kws::s2(kws::s1(match &kws::make_request(&make_url(&site_profile.root_uri
                                                                                         ,&site_profile.query_cmd
                                                                                         ,&site_profile.uri_seperator
                                                                                         ,&squery)) { 
                                                        Err(_why) => "[x] ERROR::NO_RESPONSE:: Failed to get response from the server.",
                                                        Ok(response) => response }
                                                     ,&site_profile));
                tx_t.send(results)
                    .expect(&format!("{}" ,Color::Red.paint( format!("[x] ERROR::MPSC_SEND_FALIURE:T->{}:- Couldn't Send Acquired results across threads!",thread_name))));
                log!("gl",format!("Sent to MPSC channel ↣ {}", thread_name ));
            }).expect(&format!("[x] Failed to create thread ↣ {}",t_name)));
    }
    // Joining all threads to the main thread -- BLOCKING!
    fetch_handle
    .into_iter()
    .for_each(|thread| thread.join()
                             .expect(&format!("{}" , Color::Fixed(202).bold().paint("---- ERROR::THREAD_JOIN_FAILURE ----"))));

    let mut temp: Vec<Listing<String>> = Default::default();
    for i in 0..sites_count {
        let temp_raw = rx.recv_timeout(Duration::from_secs(5))
                        .expect(&format!("{}",Color::Red.paint("[x] ERROR::MPSC_RECIEVE_FALIURE:- Couldn't Receive acquired results on main thread!")));
        if temp_raw.is_some() {
            if i == 0 {
                temp = temp_raw.unwrap();
            }
            else {
                temp.extend(temp_raw.unwrap());
            }
        }
    }

    log!("m","Passing JSON from KWE.Fetch");
    // # Write to file
    //write_json(category.to_string(),(&search_query).to_string(),listings.lock().unwrap().to_vec());

    //write_json(category.to_string(),(&search_query).to_string(),temp.clone());
 
    //let all_listings =  &listings.lock().expect("Failed to acquire lock on listings mutex");
    Some(Listings{ date_time: format!("{}", time::now()),
                   category: category.to_string(),
                   query: (&search_query).to_string(),
                   listings: temp
                 })
}

fn write_json(cat: String, query: String, lst: Vec<Listing<String>> ) -> () {
    std::fs::File::create(format!("{}.json", query.replace(" ","_")))
                  .unwrap()
                  .write_all(
                      format!("{:#?}",
                          Json(Listings{ date_time: format!("{}", time::now()),
                                         category: cat,
                                         query,
                                         listings: lst
                                       })).as_bytes()).unwrap();
}

#[test]
fn is_url_generated_correctly(){
    assert_eq!(make_url(&"https://www.urbanladder.com".to_string()
                       ,&"products/search?keywords".to_string()
                       ,"+",&"Queen Size Bed".to_string())
              ,"https://www.urbanladder.com/products/search?keywords=Queen+Size+Bed");
}

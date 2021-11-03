//  oooooooo8 oooooooooo ooooo  oooo 
// 888         888    888 888    88  
//  888oooooo  888oooo88   888  88   
//         888 888  88o     88888    
// o88oooo888 o888o  88o8    888     
//                       src/main.rs
//                       The scraping server for prcomp

mod orel;
mod nabu;
mod types;
mod wrapper;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
//use rocket::response::content::Json;
use rocket::serde::json::Json;
use wrapper::nabu_fetch;

#[macro_use] extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn fake_listings()  -> crate::types::Listings<String> {
    let x = "X".to_string();
    crate::types::Listings{ date_time: x.clone()
                          , query: x.clone()
                          , category: x.clone()
                          , listings: vec![crate::types::Listing{ name: x.clone()
                                                                     , store: x.clone()
                                                                     , return_replace: x.clone()
                                                                     , warranty: x.clone()
                                                                     , specs: x.clone()
                                                                     , price: x.clone()
                                                                     , img: x.clone()
                                                                     , url: x.clone()}]  }
}

// Routes
#[get("/")]
fn root() -> &'static str {
    "This is the root of the website. You shouldn't be here :)"
}

//#[get("/elx/<ex_query>", format = "application/json")]
#[get("/elx/<ex_query>")]
fn elx(ex_query: &str) -> Json<crate::types::Listings<String>>  {
    println!("Started elx code");
    Json(match nabu_fetch("electronics".to_string(),ex_query.replace("+"," ")) {
        Some(listings) => listings,
        None => fake_listings()
    })
}

#[get("/fur/<furn_query>")]
fn fur(furn_query: &str) -> Json<crate::types::Listings<String>>  {
    Json(match nabu_fetch("furniture".to_string(),furn_query.replace("+"," ")) {
        Some(listings) => listings,
        None => fake_listings()
    })
}

#[get("/clo/<cloth_query>")]
fn clo(cloth_query: &str) -> Json<crate::types::Listings<String>>  {
    Json(match nabu_fetch("clothing".to_string(),cloth_query.replace("+"," ")) {
        Some(listings) => listings,
        None => fake_listings()
    })
}
#[rocket::main]
async fn main() {
                rocket::build()
                       .mount("/",routes![root,elx,fur,clo])
                       .attach(CORS)
                       .launch()
                       .await.unwrap();
}

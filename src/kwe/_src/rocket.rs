//  oooooooo8 oooooooooo ooooo  oooo 
// 888         888    888 888    88  
//  888oooooo  888oooo88   888  88   
//         888 888  88o     88888    
// o88oooo888 o888o  88o8    888     
//                       src/main.rs
//                       The scraping server for prcomp

#![feature(proc_macro_hygiene, decl_macro)]

mod orel;
mod nabu;
mod log;
mod types;
mod wrapper;
use std::io::Cursor;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType,Header,Method,Status},
    Request,
    Response,
};
// use serde::{Serialize,Deserialize};
use rocket_cors::{CorsOptions, AllowedHeaders, AllowedOrigins};
use std::str::FromStr;
//use rocket::response::content::Json;
// use rocket::serde::json::Json;
use rocket_contrib::json::Json;
use wrapper::nabu_fetch;
use types::{Listing,Listings};
use ansi_term::Color;

#[macro_use] extern crate rocket;

pub struct CORS();

// #[rocket::async_trait]
// impl Fairing for CORS {
//     fn info(&self) -> Info {
//         Info {
//             name: "Attaching CORS headers to responses",
//             kind: Kind::Response
//         }
//     }

//     async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
//         if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
//         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
//         response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
//         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
//         }
//         if request.method() == Method::Options {
//             response.set_header(ContentType::Plain);
//             response.set_sized_body(0,Cursor::new(""));
//         }
//     }
// }

fn fake_listings()  -> Listings<String> {
    let x = "X".to_string();
    crate::types::Listings{ date_time: x.clone()
                          , query: x.clone()
                          , category: x.clone()
                          , listings: vec![Listing{ name: x.clone()
                                                  , store: x.clone()
                                                  , return_replace: x.clone()
                                                  , warranty: x.clone()
                                                  , specs: x.clone()
                                                  , price: x.clone()
                                                  , img: x.clone()
                                                  , id: x.clone()
                                                  , url: x.clone()}]  }
}

// Routes
#[get("/")]
fn root() -> &'static str {
    "This is the root of the website. You shouldn't be here :)"
}

// #[get("/elx/<ex_query>", format = "application/json")]
#[get("/elx/<ex_query>")]
fn elx(ex_query: String) -> Json<Listings<String>>  {
// fn elx<'t>(ex_query: &str) -> Response<'t>  {
// fn elx(ex_query: &str) -> Json<String>  {
    log!("c","Started elx code");
    Json( match nabu_fetch("electronics".to_string(),ex_query.replace("+"," ")) {
        Some(listings) => {log!("c","Sending JSON..."); listings},
        None => fake_listings()
        // Some(listings) => format!("{{ \"status \" : \"200 OK\", \"body\" : {}}}",listings.to_json()),
        // None => String::from(r#"{ "status" : "404 Not There"}"#)
    })
}
// #[options("/elx/<ex_query>")]
// fn elx_preflight(ex_query: &str) -> rocket::response::status::NoContent {
//     rocket::response::status::NoContent
// }
#[options("/elx/<ex_query>")]
fn elx_preflight<'a>(ex_query: String) -> Response<'a> {
    Response::build()
            .status(Status::Ok)
            .raw_header("Access-Control-Allow-Origin", "http://localhost:3000")
            .raw_header("Access-Control-Allow-Methods", "OPTIONS, GET")
            .raw_header("Access-Control-Allow-Headers", "*")
            .finalize()
}

#[get("/fur/<furn_query>")]
fn fur(furn_query: String) -> Json<Listings<String>>  {
    Json(match nabu_fetch("furniture".to_string(),furn_query.replace("+"," ")) {
        Some(listings) => listings,
        None => fake_listings()
    })
}

#[get("/clo/<cloth_query>")]
fn clo(cloth_query: String) -> Json<Listings<String>>  {
    Json(match nabu_fetch("clothing".to_string(),cloth_query.replace("+"," ")) {
        Some(listings) => listings,
        None => fake_listings()
    })
}
// #[rocket::main]
fn main() {
    // let corsOpts = CorsOptions::default().to_cors().expect("Failed to convert CorsOptions to Cors");
    // let allowed_method = vec![Method::Get].into_iter().map(From::from).collect();
    let allowed_method = vec!["Get","Options"].into_iter().map(|s| FromStr::from_str(s).unwrap()).collect();
    // You can also deserialize this
    let cors: rocket_cors::Cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: allowed_method,
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default() }
    .to_cors().expect("Failed to convert CorsOptions to Cors");

    rocket::ignite()
        //    .mount("/",routes![root,elx,elx_preflight,fur,clo])
           .mount("/",routes![root,elx,elx_preflight,fur,clo])
        //    .attach(CORS())
           .attach(cors)
           .launch();
        //    .await.unwrap();
}
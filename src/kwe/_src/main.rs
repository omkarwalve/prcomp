//   __  __  __  __  __  ______  
//  |  |/ / |  \/  \|  ||   ___| 
//  |     \ |     /\   ||   ___| 
//  |__|\__\|____/  \__||______| src/main.rs                              
//       Kilowog's engine server.
#![allow(dead_code)]
#![feature(async_closure)]
mod orel;
mod kws;
mod log;
mod types;
mod wrapper;
mod db;

use ansi_term::Color;
use actix_web::{App,Error,get,post,HttpResponse,HttpServer,middleware::Logger,Responder,web};
use env_logger::Env;
use actix_cors::Cors;
use serde::Deserialize;
use serde_json::json;
use std::{env,fs::File, io::Read};
// use types::{Listing,Listings};

#[derive(Deserialize)]
struct MSearch {
    queries: Vec<String>,
}

const ROUTES: [&'static str; 3] = ["elx","clo","fur"];
const ENGINE_WIKI: &'static str = "root.html";

/// ## KWE `:: Root`
/// Triggers when '/' is called. Returns a small wiki on how to use the `api` endpoint.
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(
        match File::open(format!("{}/{}",env!("CARGO_MANIFEST_DIR"),ENGINE_WIKI)) {
            Ok(mut file) => {
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)
                    .expect("Failed to read `root.html` to `String` .");
                file_contents
            },
            Err(_) => String::from("gg"),
        }
    )
}

/// ## KWE `:: API`
/// The `api` for users who'd rather need `json`.
#[get("/api/{category}/{query}")]
// async fn api(web::Path((category,query)): web::Path<(String,String)>) -> Result<impl Responder,Error> {
async fn api(path: web::Path<(String,String)>) -> Result<impl Responder,Error> {
    Ok(format!("Finding {} in {}",path.0,path.1))
}

/// ## KWE `:: Search`
/// The main route that handles search requests on `kilowog`.
#[get("/{category}/{query}")]
// async fn search(web::Path((category,query)): web::Path<(String,String)>) -> Option<impl Responder> {
async fn search(path: web::Path<(String,String)>) -> Option<impl Responder> {
    match ROUTES.iter().any(|&s| s == &path.0) {
        true => Some(match wrapper::kwe_fetch(&path.0, path.1.replace('+'," ")) {
                    Some(results) => {log!("c","Sending JSON..."); web::Json(results)},
                    None => web::Json(Default::default())
                }),
        false => None
    }
}

/// ## KWE `:: Database`
/// The main endpoint to get user's click data
#[get("/clk/{uid}")]
async fn get_clk(uid: web::Path<String>) -> impl Responder {
    match uid.parse::<i32>() {
        Ok(id) => 
        match db::init().await {
            Ok(client) => match db::get_clicks(id, client).await {
                Ok(list) => web::Json( json!({ "code": 200, "success": true , "data": list }) ),
                Err(y) => web::Json(json!({ "code": 1, "success": false, "reason": y.to_string() })), 
            },
            Err(y) => web::Json( json!({ "code": 2, "success": false, "reason": y.to_string() }) )
        },
        Err(y) => web::Json( json!({ "code": 0, "success": false, "reason": format!("{} :: {}", y.to_string(), uid) }) )
    }
}

#[post("/{category}")]
async fn msearch(category: web::Path<String>, body: web::Json<MSearch>) -> impl Responder {
    // web::Json( json!({ "qrs" : body.queries.join("|") }) )
    let cat = category.into_inner();
    match ROUTES.iter().any(|&s| s == cat) {
        true => Some(match wrapper::kwe_multi_fetch(&cat, body.queries.clone()) {
                    Some(results) => {log!("c","Sending JSON..."); web::Json(results)},
                    None => web::Json(Default::default())
                }),
        false => None
    }
}

// async fn preflight(web::Path((category,query)): web::Path<(String,String)>) -> impl Responder {
//     HttpResponse::Ok().body("")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service( 
                web::scope("/q")
                    .service(search).wrap(Cors::permissive()) 
                    .service(msearch)
                    // .wrap(Cors::permissive()) 
            )
            .service(api)
            .service(get_clk)
            .wrap(Logger::default())
    })
    .shutdown_timeout(10)
    .bind("0.0.0.0:8000")?
    // .expect("Failed to bind to port 8000")
    .run()
    .await
}

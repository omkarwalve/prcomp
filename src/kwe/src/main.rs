//   __  __  __  __  __  ______  
//  |  |/ / |  \/  \|  ||   ___| 
//  |     \ |     /\   ||   ___| 
//  |__|\__\|____/  \__||______| 
//                   src/main.rs                              
//       Kilowog's engine server.
mod orel;
mod kws;
mod log;
mod types;
mod wrapper;

use ansi_term::Color;
use actix_web::{App,Error,get,HttpResponse,HttpServer,middleware::Logger,Responder,web};
use env_logger::Env;
use actix_cors::Cors;
use std::{env,fs::File, io::{Read,Cursor}};
// use types::{Listing,Listings};

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
                file_contents},
            Err(_) => String::from("gg"),
        }
    )
}
/// ## KWE `:: API`
/// The `api` for users who'd rather need `json`.
#[get("/api/{category}/{query}")]
async fn api(web::Path((category,query)): web::Path<(String,String)>) -> Result<impl Responder,Error> {
    Ok(format!("Finding {} in {}",query,category))
}

/// ## KWE `:: Search`
/// The main route that handles search requests on `kilowog`.
#[get("/{category}/{query}")]
async fn search(web::Path((category,query)): web::Path<(String,String)>) -> Option<impl Responder> {
    match ROUTES.iter().any(|&s| s == &category) {
        true => Some(match wrapper::kwe_fetch(&category, query.replace('+'," ")) {
                    Some(results) => {log!("c","Sending JSON..."); web::Json(results)},
                    None => web::Json(Default::default())
                }),
        false => None
    }
}

async fn preflight(web::Path((category,query)): web::Path<(String,String)>) -> impl Responder {
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service( 
                web::scope("/q")
                    .service(search).wrap(Cors::permissive()) 
            )
            .service(api)
            .wrap(Logger::default())
    })
    .shutdown_timeout(10)
    .bind("0.0.0.0:8000")?
    // .expect("Failed to bind to port 8000")
    .run()
    .await
}
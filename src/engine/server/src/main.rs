
use actix_web::{
    App,
    Error,
    get,
    HttpResponse,
    HttpServer,
    middleware::Logger,
    Responder,
    web
};
use std::{
    env,
    fs::File,
    io::{Read,Cursor,Result}
};
use env_logger::Env;
use actix_cors::Cors;
use kws;
#[macro_use] extern crate lazy_static;

mod dir;

const BIND_ADDRESS: &str = "0.0.0.0:8000";
const ROUTES: [&'static str; 3 ] = [ "elx", "clo", "fur" ];
const ENGINE_WIKI: &'static str = "root.html";
const WIKI_FAIL_TEXT: &'static str = r#"This is the root of the website.<br> Access the API using <code>/api/{category}/{query}</code>"#;
lazy_static! {
    static ref SCHEMA: kws::Schema = kws::Schema::from((dir::current().expect("CRNT-DIR").join("schema"),
                                            vec![("elx","elx"),
                                                ("fur","fur"),
                                                ("clo","clo")]));
}
/// ## KWE `:: Root`
/// Triggers when '/' is called. Returns a small wiki on how to use the `api` endpoint.
#[get("/")]
async fn wiki() -> impl Responder {
    HttpResponse::Ok().body(
        match File::open(format!("{}/{}", dir::current_as_string().expect("CRNT-DIR!"), ENGINE_WIKI)) {
            Ok(mut file) => {
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)
                    .expect("Failed to read `root.html` to `String` .");
                file_contents },
            Err(_) => String::from(WIKI_FAIL_TEXT),
        }
    )
}

/// ## KWE `:: Search`
/// The main route that handles search requests on `kilowog`.
#[get("/{category}/{query}")]
async fn search(web::Path((category,query)): web::Path<(String,String)>) -> impl Responder {
    let bs = kws::Request::default(category,query,SCHEMA.clone());
    println!("{}",bs);
    "hehe"
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("{}",SCHEMA.clone());
    HttpServer::new(|| {
        App::new()
            .service(wiki)
            .service(
                web::scope("/q")
                    .service(search)
                    .wrap(Cors::permissive()))
            .wrap(Logger::default())
    })
    .shutdown_timeout(10)
    .bind(BIND_ADDRESS)?
    .run()
    .await
}
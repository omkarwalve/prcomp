// scrape/types.rs
//         - Data structures for nabu.

// use rocket::serde::{ Serialize, Deserialize };
use serde::{ Serialize, ser::{Serializer,SerializeMap}, Deserialize };
use actix_web::{Error,Responder,HttpResponse,HttpRequest};
use futures::future::{ready,Ready};
/// ## Listing
/// Container struct to store each product data
#[derive(Serialize)]
#[derive(Default,Debug,Clone)]
pub struct Listing<T> { 
    pub name: T,
    pub store: T,
    pub return_replace: T,
    pub warranty: T,
    pub specs: Spectable<String>,
    pub price: T,
    pub img: T,
    pub url: T,
    pub id: T,
}

#[derive(Serialize,Debug)]
pub struct Listings<T> {
    pub query: T,
    pub category: T,
    pub date_time: T,
    pub listings: Vec<Listing<T>>,
}

#[derive(Clone,Default,Debug)]
pub struct Spectable<T> {
   pub key: Vec<T>,
   pub value: Vec<T>
}

impl Spectable<String> {
    fn len(&self) -> usize {
        self.key.len()
    }
}
impl Serialize for Spectable<String> {
    fn serialize<S>(&self,serializer: S) -> Result<S::Ok,S::Error>
    where S: Serializer 
    {
        if self.len() > 0 {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (key,value) in self.key.iter().zip(self.value.iter()) {
                map.serialize_entry(key,value)?
            }
            map.end()
        } else {
            serializer.serialize_char('❔')
        }
    }
}

// Actix-web 3.*.* code
// impl Responder for Listings<String> {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse,Error>>;
//     fn respond_to(self,_req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).expect("Failed to stringify `Listings<String>`");
//         ready(Ok(HttpResponse::Ok()
//                 .content_type("application/json")
//                 .body(body)
//             ))
//     }
// }

// Actix-web 4.*.* code
// impl Responder for Listings<String> {
//     // type Error = Error;
//     // type Future = Ready<Result<HttpResponse,Error>>;
//     type Body = 
//     fn respond_to(self,_req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).expect("Failed to stringify `Listings<String>`");
//         ready(Ok(HttpResponse::Ok()
//                 .content_type("application/json")
//                 .body(body)
//             ))
//     }
// }
impl Default for Listings<String> {
    fn default() -> Self {
        let def: String = String::from("-");
        Listings {
            query: def.clone(),
            category: def.clone(),
            date_time: def.clone(),
            listings: vec![]
        }
    }

}

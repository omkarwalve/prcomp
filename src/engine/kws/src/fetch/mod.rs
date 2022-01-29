
mod types;
pub use types::{Request,Schema};


/// ## Fetch `:: async`
pub async fn fetch<T>(options: Request<T>) {
}

/// ## `say_hi()`
/// The name speaks for itself.
pub fn say_hi() { println!("Hi from kws::fetch!"); }
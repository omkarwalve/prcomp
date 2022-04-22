use mongodb::{options::ClientOptions, Client, Collection, bson::{Document,doc} };
use std::error::Error;

pub async fn init() -> Result<Client, Box<dyn Error>> {
    let opts = ClientOptions::parse("mongodb+srv://sohamwakade7:Soham%4027@cluster0.wnbre.mongodb.net/Kilowog").await?;
    Ok(mongodb::Client::with_options(opts)?)
}

pub async fn get_clicks(uid: i32, client: Client) -> Result<Vec<String>, Box<dyn Error>> {
    let users: Collection<Document> = client.database("Kilowog").collection("Users");
    let target_user = users
                      .find_one(doc! {"userid": uid }, None,)
                      .await?
                      .expect(&format!("No user with id: {} found",uid));
    let mut res: Vec<String> = Vec::new(); 
    for bsn in target_user.get_array("clicks")?.clone().into_iter() {
        let item = bsn.to_string();
        res.push(item[1..(item.len()-1)].into());
    }
    Ok(res)
}

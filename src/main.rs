use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ISSLocation {
    iss_position: Position,
    message: String,
    timestamp: i64,
}

#[derive(Deserialize, Debug)]
struct Position {
    latitude: String,
    longitude: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("http://api.open-notify.org/iss-now.json")
        .await?
        .json::<ISSLocation>()
        .await?;
    println!("{:#?}", response);
    Ok(())
}

use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct Messages {
    id: String,
    created_at: i64,
    content: String,
    attachments: Vec<String>,
    channel_id: String,
    author_id: String
}

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
}


pub async fn get_message() -> Result<Vec<Messages>, Box<dyn std::error::Error>> {

    let url = format!(
        "https://{hostname}/{version}/message/{channel}/{offset}/{count}",
        hostname = "taku.n1ko23.moe",
        version = "v1",
        channel = "85cd60c4-ab15-4766-a4b9-bb32e2cbbca3",
        offset = 0,
        count = 25
    );

    let response = reqwest::get(&url).await?.text().await?;
    let messages: Vec<Messages> = serde_json::from_str(&response)?;

    Ok(messages)
    
}

pub async fn get_user(_id: String) -> Result<User, Box<dyn std::error::Error>> {

    let url = format!(
        "backend.taku.moe/{version}/user/{id}",
        version = "v1",
        id = _id
    );

    let response = reqwest::get(&url).await?.text().await?;
    let user: User = serde_json::from_str(&response)?;

    Ok(user)

}
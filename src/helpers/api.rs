use crate::helpers::json::Listen;
use reqwest::blocking::{Client, Response};
use reqwest::header::AUTHORIZATION;
use serde_json::{Map, Number, Value};

pub fn delete_listen(api_url: &str, api_key: &str, listen: &Listen) -> Result<Response, reqwest::Error> {
    let url = format!("{}/1/delete-listen", api_url);
    let mut key: String = "Token ".to_owned();
    key.push_str(&*api_key);

    let mut map = Map::new();
    map.insert(
        "listened_at".to_string(),
        Value::Number(Number::from(listen.listened_at)),
    );
    map.insert(
        "recording_msid".to_string(),
        Value::String((&listen.recording_msid.to_string()).to_string()),
    );

    let client = Client::new();
    let res = client.post(url).header(AUTHORIZATION, key).json(&map).send()?;

    Ok(res)
}

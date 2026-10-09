use crate::helpers::json::Listen;
use reqwest::blocking::{Client, Response};
use reqwest::header::AUTHORIZATION;
use serde_json::{Map, Number, Value};
use std::sync::OnceLock;
use std::time::Duration;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client")
    })
}

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

    let res = client().post(url).header(AUTHORIZATION, key).json(&map).send()?;

    Ok(res)
}

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct Listen {
    pub track_metadata: TrackMetadata,
    pub listened_at: i64,
    pub recording_msid: String,
}

#[derive(Serialize, Deserialize)]
pub struct TrackMetadata {
    pub artist_name: Option<String>,
    pub release_name: Option<String>,
    pub track_name: Option<String>,
    pub additional_info: AdditionalInfo,
    pub mbid_mapping: Option<MBIDMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalInfo {
    pub artist_msid: Option<String>,
    pub release_msid: Option<String>,
    pub recording_msid: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MBIDMapping {
    pub artist_mbids: Vec<String>,
    pub release_mbid: Option<String>,
    pub recording_mbid: Option<String>,
}

pub fn read_listens(filename: &str) -> Result<Vec<Listen>> {
    let file = File::open(filename).expect("File not found.");
    let reader = BufReader::new(file);

    println!("filename: {}", filename);
    let listens: Vec<Listen> = serde_json::from_reader(reader).expect("Error parsing JSON");

    Ok(listens)
}

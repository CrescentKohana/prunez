extern crate dotenv;

use crate::helpers::json::Listen;
use clap::{load_yaml, App};
use dotenv::dotenv;
use std::borrow::Borrow;
use std::io::{stdout, Write};
use std::path::Path;
use std::{env, thread, time};

extern crate question;
use question::{Answer, Question};

pub mod helpers {
    pub mod api;
    pub mod json;
}

fn main() {
    dotenv().ok();

    #[allow(unused_mut)]
    let mut api_key = env::var("LISTENBRAINZ_API_KEY").expect("$LISTENBRAINZ_API_KEY is not set");
    let mut api_url = "https://api.listenbrainz.org";
    let mut source_file = "listens.json";

    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(i) = matches.value_of("url") {
        api_url = i;
    }

    if let Some(i) = matches.value_of("filename") {
        source_file = i;
    }

    if !Path::new(source_file).exists() {
        println!("{} not found.", source_file);
        std::process::exit(0);
    }

    let mut artists: Vec<String> = vec![];
    if let Some(i) = matches.value_of("artists") {
        let temp: Vec<String> = i.split(";;").map(str::to_string).collect();
        artists.extend(temp);
    }

    let mut songs: Vec<String> = vec![];
    if let Some(i) = matches.value_of("songs") {
        let temp: Vec<String> = i.split(";;").map(str::to_string).collect();
        songs.extend(temp);
    }

    let mut rec_ids: Vec<String> = vec![];
    if let Some(i) = matches.value_of("recids") {
        let temp: Vec<String> = i.split(";").map(str::to_string).collect();
        rec_ids.extend(temp);
    }

    if artists.is_empty() && songs.is_empty() && rec_ids.is_empty() {
        println!("No artists, songs or recording msids defined.");
        std::process::exit(0);
    }

    println!("Artists: {:?}", artists);
    println!("Songs: {:?}", songs);
    println!("MSIDs: {:?}", rec_ids);

    let result = helpers::json::read_listens(source_file);
    let listens = result.unwrap();
    let listens_count = listens.len();

    let mut to_prune: Vec<Listen> = vec![];
    for listen in listens {
        let mut listen_match = songs.contains(listen.track_metadata.track_name.as_ref().unwrap());
        listen_match = listen_match || artists.contains(listen.track_metadata.artist_name.as_ref().unwrap());
        listen_match = listen_match || rec_ids.contains(&listen.recording_msid);

        if listen_match {
            to_prune.push(listen)
        }
    }

    println!("Found total of {} listens", listens_count);

    if to_prune.is_empty() {
        println!("Nothing to prune.");
        std::process::exit(0);
    }

    let total_prunes = to_prune.len();

    println!("Found {:?} listens to be pruned.", total_prunes);
    let answer = Question::new("Do you want to continue?")
        .default(Answer::NO)
        .show_defaults()
        .confirm();

    if answer == Answer::YES {
        println!("Pruning...");
    } else {
        println!("Aborting...");
        std::process::exit(0);
    }

    // 400ms sleep duration to prevent hitting ratelimits (30 req/10s) of ListenBrainz
    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(400);

    let mut stdout = stdout();
    let mut prunes = 0;
    for listen in to_prune {
        print!("\r{} / {} pruned", prunes, total_prunes);

        let result = helpers::api::delete_listen(api_url, api_key.borrow(), &listen);
        if result.unwrap().status().is_success() {
            prunes += 1;
        }

        stdout.flush().unwrap();
        thread::sleep(SLEEP_DURATION);
    }

    if prunes > 0 {
        print!("\rSuccessfully pruned {} listens.", prunes);
    } else {
        print!("\rNo listens pruned.");
    }
}

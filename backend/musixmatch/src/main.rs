use rayon::prelude::*;
use webscraping::{get_artist_name, single_artist_scrap};

fn init() -> Result<reqwest::blocking::Client, reqwest::Error> {
    let client = reqwest::blocking::Client::builder().build()?;
    Ok(client)
}

fn main() -> Result<(), reqwest::Error> {
    let client = init()?;
    let _album_url = get_artist_name();

    _album_url
        .into_par_iter()
        .for_each(|x| single_artist_scrap(&x.to_string(), &client));

    Ok(())
}

// add rayon now

// slam into database?

// Hyphenify consumer input i.e artist name

// Extract exact song lyrics

//Build RAG on top? in Rust or in python? --use ahnlich?

//tests

//frontend (leptos or react)

//connect API to spotify lyrics, search button. to sift music

// Breadth first search with playlists perhaps to find similar songs? or use theme?

// Data quality issue perhaps, find a way to classify song themes

// Insert Analytics into app builds

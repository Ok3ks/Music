use clap::Parser;
use std::collections::*;
use std::fs;
use rand:: {Rng, thread_rng};
use std::path::Path;
use std::thread;
use std::time::Duration;
use webscraping::{Cli, Lyrics};
use rayon::prelude::*;

pub fn get_lyrics(url: String) -> Lyrics {
    _get_lyrics_internal(url)
}
fn _get_lyrics_internal(url: String) -> Lyrics {
    let parts: Vec<String> = _parse_url_path(&url).unwrap();
    let artist: String = parts.get(3).unwrap().to_string();
    let title: String = parts.get(4).unwrap().to_string();

    // let mut lyrics = Vec::<String>::new();
    // let mut meaning= String::new();
    // let mut moods= String::new();
    // let mut rating =Vec::<String>::new();

    let response = reqwest::blocking::get(&url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let lyrics_selector = scraper::Selector::parse("div").unwrap();
    let selections = document
        .select(&lyrics_selector)
        .next()
        .unwrap()
        .text()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let lyric_section = selections.split("Writer").nth(0).unwrap().to_string();
    let other_section = match selections.split("Writer").nth(1) {
        Some(after_writer) => {
            match after_writer.split("Identify the song's sections").nth(1) {
                Some(section) => section.to_string(),
                None => {
                    println!("Error: Could not find 'Identify the song's sections' text");
                    String::new() // or return/handle error however you want
                }
            }
        }
        None => {
            println!("Error: Could not find 'Writer' section");
            String::new() // or return/handle error however you want
        }
    };

    let song_lyrics = Lyrics {
        artist: artist,
        title: title,
        lyrics_section: lyric_section,
        other_section: other_section,
    };

    song_lyrics
}

fn _parse_url_path(url: &str) -> Result<Vec<String>, String> {
    if url.is_empty() {
        return Err(String::from("URL cannot be empty"));
    }

    let parts: Vec<String> = url
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_get_albums() {
        let artist_url = String::from("https://www.musixmatch.com/artist/Kendrick-Lamar/albums");
        let albums = get_albums(artist_url);

        for i in albums {
            assert_eq!(i.starts_with("/album/"), true);
        }
    }

    #[test]
    fn test_func_get_songs() {
        let album_url = String::from("https://www.musixmatch.com/artist/Taylor-Swift/album/Taylor-Swift/Taylor-Swift-Big-Machine-Radio-Release-Special");
        let songs = get_songs(album_url);

        for i in songs {
            assert_eq!(i.starts_with("/lyrics/"), true);
        }
    }

    #[test]
    fn test_func_get_lyrics() {
        let song_url =
            String::from("https://www.musixmatch.com/lyrics/Taylor-Swift/champagne-problems");
        let songs = get_lyrics(song_url);

        ////Get Lyrics
        assert_eq!(songs.lyrics_section.contains("Lyrics"), true);
        assert_eq!(songs.other_section.contains("Mood"), true);
        assert_eq!(songs.other_section.contains("Rating"), true);
        assert_eq!(songs.other_section.contains("Meaning"), true);
    }
}

pub fn _get_songs(album_url: String) -> HashSet<String> {
    get_songs(&album_url)
}

fn get_songs(album_url: &String) -> HashSet<String> {
    let response = reqwest::blocking::get(album_url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let mut links = Vec::<String>::new();
    let lyrics_selector = scraper::Selector::parse("a").unwrap();

    for element in document.select(&lyrics_selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(String::from(link));
        }
    }

    let _song_links = links
        .into_iter()
        .filter(|x| x.starts_with("/lyrics/"))
        .collect::<HashSet<_>>();

    //Logic to save list of songs in an album
    let album_name = album_url.clone();
    let mut album_name = album_name.split("/").collect::<Vec<_>>();
    let album_path = format!("lyrics/{1}/album-{0}",  album_name.pop().unwrap(), album_name.pop().unwrap());
    let album_path = Path::new(&album_path);

    if let Some(parent) = album_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let _ = fs::write(album_path, _song_links.clone().into_iter().collect::<Vec<_>>().join(",")).unwrap();

    _song_links
}

pub fn _get_albums(url: String) -> HashSet<String> {
    get_albums(&url)
}

fn get_albums(url: &String) -> HashSet<String> {
    let response = reqwest::blocking::get(url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let mut links = Vec::<String>::new();
    let lyrics_selector = scraper::Selector::parse("a").unwrap();

    for element in document.select(&lyrics_selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(String::from(link));
        }
    }

    let _album_links = links
        .into_iter()
        .filter(|x| x.starts_with("/album/"))
        .collect::<HashSet<_>>();


    _album_links
}

fn get_artist_name() -> Vec<String> {
    let args = Cli::parse();
    let mut _albums = Vec::new();

    for i in args.artist {
        let _album_base_url: String =
            format!("https://www.musixmatch.com/artist/{0}/albums", { i }).to_string();
        _albums.push(_album_base_url);
    }

    _albums
}

pub fn single_artist_scrap(artist: String) {
    _single_artist_scrap(&artist);
}

fn _single_artist_scrap(artist: &String) {
    let _base_url = String::from("https://www.musixmatch.com/");
    let mut rng = thread_rng();


    let albums = get_albums(&artist);
    let mut count = 0;

    for element in albums {

        let path = _base_url.clone() + &element.clone().trim_start_matches('/');
        let songs = get_songs(&path);
        let random_seconds = rng.random_range(0..60);


        for song in songs {
            let lyric = get_lyrics(_base_url.clone() + &song.clone().trim_start_matches('/'));
            count += 1;
            lyric.save();
            thread::sleep(Duration::from_millis(random_seconds));
        }
        thread::sleep(Duration::from_millis(10000));
    }
}

// pub fn spotify_search(url: &'static str, bearerToken: &'static str) -> String {

//     let params = HashMap::from(
//         [
//             ("type", "playlist"), 
//             ("next", "5"),
//             ("limit", "15")
//         ]);

//     let response = reqwest::blocking::Client::new().get(url).header(AUTHORIZATION, bearerToken).send();
//     let response = response.unwrap().text().unwrap();
//     response

// }

fn main() {

    let _album_url = get_artist_name();
    _album_url.into_par_iter().for_each(|x| _single_artist_scrap(&x.to_string()));

}

// add rayon now

// slam into database?

// Hyphenify consumer input i.e artist name

// Extract exact song lyrics

//Build RAG on top? in Rust or in python? --use ahnlich?

//tests

//frontend (leptos or react)

//understand owned and borrowed references

//unwrap_or_else, match , if let

//connect API to spotify lyrics, search button. to sift music

// Breadth first search with playlists perhaps to find similar songs? or use theme? 

// Data quality issue perhaps, find a way to classify song themes

// Insert Analytics into app builds
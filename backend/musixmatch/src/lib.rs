use clap::{ArgAction, Parser};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::*;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lyrics {
    pub artist: String,
    pub title: String,
    pub lyrics_section: String,
    pub other_section: String,
}

impl Lyrics {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn save(&self) {
        let path = format!("lyrics/{0}/{1}", { &self.artist }, { &self.title });
        let file_path = Path::new(&path);

        if let Some(parent) = file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let _ = fs::write(file_path, self.to_json());
    }
}

impl Display for Lyrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.lyrics_section, self.other_section)
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "artist", long_about = None, action = "append")]
pub struct Cli {
    #[arg(short, long, action = ArgAction::Append)]
    pub artist: Vec<String>,
}

//TODO: replace with better search algorithm
pub fn search<'a>(query: String, contents: String) -> Vec<String> {
    let query = query.to_lowercase();

    let results: Vec<String> = contents
        .lines()
        .map(|x| x.to_lowercase())
        .filter(|x| x.contains(&query.to_lowercase()))
        .collect();

    results
}

pub fn get_lyrics(url: String, client: &reqwest::blocking::Client) -> Result<Lyrics, String> {
    _get_lyrics_internal(url, &client)
}

fn _get_lyrics_internal(url: String, client: &reqwest::blocking::Client) -> Result<Lyrics, String> {
    let parts = _parse_url_path(&url);

    match parts {
        Ok(_) => {
            match parts?.as_slice() {
                sections => {
                    let Some(artist) = sections.get(3) else {
                        return Err(String::from("No content here"));
                    };
                    let Some(title) = sections.get(4) else {
                        return Err(String::from("Empty"));
                    };

                    let response = client.get(&url).send().unwrap().text().unwrap();

                    let document = scraper::Html::parse_document(&response);

                    let lyrics_selector = scraper::Selector::parse("div").unwrap();

                    let selections = document
                        .select(&lyrics_selector)
                        .flat_map(|element| element.text())  // Flatten all text from all elements
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("\n");

                    let out = match selections.split("Writer").into_iter() {
                        mut sections => {
                            let Some(lyric_section) = sections.nth(0) else {
                                return Err(String::from(format!(
                                    "Error obtaining lyric section, for {0}",
                                    url
                                )));
                            };

                            let Some(other_section) = sections.nth(0) else {
                                return Err(String::from(format!(
                                    "Error obtaining other_section, for {0}",
                                    url
                                )));
                            };

                            (lyric_section, other_section)
                        }
                    };

                    let song_lyrics = Lyrics {
                        artist: artist.to_string(),
                        title: title.to_string(),
                        lyrics_section: out.0.to_string(),
                        other_section: out.1.to_string(),
                    };

                    Ok(song_lyrics)
                }
            }
        }
        _ => Err(String::from("No Url to parse")),
    }
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

        ////Possibly mock this
        assert_eq!(songs.lyrics_section.contains("Lyrics"), true);
        assert_eq!(songs.other_section.contains("Mood"), true);
        assert_eq!(songs.other_section.contains("Rating"), true);
        assert_eq!(songs.other_section.contains("Meaning"), true);
    }
}

fn _link_extractor(response: String) -> Vec<String> {
    let document = scraper::Html::parse_document(&response);
    let lyrics_selector = scraper::Selector::parse("a").unwrap(); // change unwrap

    let links: Vec<String> = document
        .select(&lyrics_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(String::from)
        .collect();

    links
}

pub fn get_songs(album_url: String, client: &reqwest::blocking::Client) -> HashSet<String> {
    _get_songs(&album_url, &client)
}

fn _get_songs(album_url: &String, client: &reqwest::blocking::Client) -> HashSet<String> {
    let response = client.get(album_url).send().unwrap().text().unwrap();
    let _song_links = _link_extractor(response)
        .into_iter()
        .filter(|x| x.starts_with("/lyrics/"))
        .collect::<HashSet<_>>();

    //Logic to save list of songs in an album
    let album_name = album_url.clone();
    let mut album_name = album_name.split("/").collect::<Vec<_>>();
    let album_path = format!(
        "lyrics/{1}/album-{0}",
        album_name.pop().unwrap(),
        album_name.pop().unwrap()
    );

    println!("{}", album_path);
    let album_path = Path::new(&album_path);

    if let Some(parent) = album_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _ = fs::write(
        album_path,
        _song_links
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(","),
    );

    _song_links
}

pub fn get_albums(url: String, client: &reqwest::blocking::Client) -> HashSet<String> {
    _get_albums(&url, &client).expect("Error configuring client")
}

fn _get_albums(
    url: &String,
    client: &reqwest::blocking::Client,
) -> Result<HashSet<String>, reqwest::Error> {
    let response = client.get(url).send()?.text()?;
    let _album_links = _link_extractor(response)
        .into_iter()
        .filter(|x| x.starts_with("/album/"))
        .collect::<HashSet<_>>();

    Ok(_album_links)
}

pub fn get_artist_name() -> Vec<String> {
    _get_artist_name()
}

fn _get_artist_name() -> Vec<String> {
    let args = Cli::parse();
    let mut _albums = Vec::new();

    for i in args.artist {
        let _album_base_url: String =
            format!("https://www.musixmatch.com/artist/{0}/albums", { i }).to_string();
        _albums.push(_album_base_url);
    }

    _albums
}

pub fn single_song_scrap(song: String, client: &reqwest::blocking::Client) {
    let _ = _single_song_scrap(song, &client);
}

pub fn _single_song_scrap(
    song: String,
    client: &reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let _base_url = String::from("https://www.musixmatch.com/");
    let mut rng = thread_rng();

    let random_seconds = rng.random_range(0..1000);
    let lyric = get_lyrics(
        _base_url.clone() + &song.clone().trim_start_matches('/'),
        client,
    );
    lyric?.save();
    thread::sleep(Duration::from_millis(random_seconds));
    Ok(())
}

pub fn single_album_scrap(album: String, client: &reqwest::blocking::Client) {
    let _ = _single_album_scrap(&album, &client);
}

fn _single_album_scrap(album: &String, client: &reqwest::blocking::Client) {
    let _base_url = String::from("https://www.musixmatch.com/");
    let path = _base_url.clone() + &album.clone().trim_start_matches('/');
    let songs = get_songs(path, client);

    for song in songs {
        single_song_scrap(song, client);
    }
    thread::sleep(Duration::from_millis(10000));
}

pub fn single_artist_scrap(artist: String, client: &reqwest::blocking::Client) {
    _single_artist_scrap(artist, &client);
}

fn _single_artist_scrap(artist: String, client: &reqwest::blocking::Client) {
    let albums = get_albums(artist, &client);
    for element in albums {
        println!("{}", element);
        single_album_scrap(element, &client);
    }
}
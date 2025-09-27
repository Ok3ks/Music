use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs;
use std::path::Path;

const SPOTIFY_SEARCH: &'static str = "https://api.spotify.com/v1/search";

enum SSearchType {

    Artist(String),
    Album(String), 
    Playlist(String), 
    Track(String), 
    Show(String), 
    Episode(String), 
    Audiobook(String)
}

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
        let path = format!("lyrics/{0}/{1}", {&self.artist}, {&self.title });
        let file_path = Path::new(&path);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(file_path, self.to_json()).unwrap();
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

pub fn search<'a>(query: String, contents: String) -> Vec<String> {
    let mut results = Vec::<String>::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.to_string());
        }
    }
    results
}
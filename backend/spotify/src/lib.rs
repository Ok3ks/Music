use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials, OAuth, AuthCodeSpotify, scopes};
use rspotify_model::{Market, Country, PlaylistId, FullTrack};
use futures_util::TryStreamExt;
use futures_util::pin_mut;
use futures_util::StreamExt;
use rspotify_model::PlayableItem;
use webscraping::{get_songs, get_lyrics, single_song_scrap};
use std::time::Duration;
use rand:: {Rng, thread_rng};
use std::thread;

#[tokio::main]
pub async fn ClientCredExample() {
    // You can use any logger for debugging.
    // env_logger::init();

    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token().await.unwrap();

    // Running the requests
    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(birdy_uri, None).await;

    println!("Response: {albums:#?}");
}

#[tokio::main]
pub async fn AuthTokenExample() {
    // You can use any logger for debugging.

    let creds = Credentials::from_env().unwrap();

    // Using every possible scope
    let scopes = scopes!(
        "user-read-email",
        "user-read-private",
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read",
        "user-library-read",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "playlist-read-collaborative",
        "playlist-read-private",
        "user-follow-modify",
        "user-library-modify",
        "user-modify-playback-state",
        "playlist-modify-public",
        "playlist-modify-private",
        "ugc-image-upload"
    );
    let oauth = OAuth::from_env(scopes).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();

    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();

    let playlists = spotify.current_user_playlists();

    println!("\nItems (concurrent):");
    playlists
        .try_for_each_concurrent(10, |item| async move {
            println!("{} {} {}", item.name, item.id, item.public.unwrap());
            // item.href item.name item.id
            Ok(())
        })
        .await
        .unwrap();

    // let token = spotify.token.lock().await.unwrap();
    // println!("Access token: {}", &token.as_ref().unwrap().access_token);
    // println!(
    //     "Refresh token: {}",
    //     token.as_ref().unwrap().refresh_token.as_ref().unwrap()
    // );

    // let sub_playlist = value.playlist(item.id, Some("name"), Some(Market::Country(Country::UnitedKingdom)));
    // println!("{}", sub_playlist);
}

#[tokio::main]
pub async fn get_playlist_info(id: &str) {

    let creds = Credentials::from_env().unwrap();

    // Using every possible scope
    let scopes = scopes!(
        "user-read-email",
        "user-read-private",
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read",
        "user-library-read",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "playlist-read-collaborative",
        "playlist-read-private",
        "user-follow-modify",
        "user-library-modify",
        "user-modify-playback-state",
        "playlist-modify-public",
        "playlist-modify-private",
        "ugc-image-upload"
    );
    let oauth = OAuth::from_env(scopes).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();


    spotify.prompt_for_token(&url).await.unwrap();

    let tracks = spotify.playlist(
        PlaylistId::from_id(id).expect("Id Error"), 
        Some("collaborative,external_urls,href,id,images,name,owner,public,snapshot_id,tracks,followers"), 
        Some(Market::Country(Country::UnitedKingdom)));
    
    let playlist = tracks.await.unwrap();
    for item in playlist.tracks.items {
        let x = item.track.unwrap();
        match x {
            PlayableItem::Track(x) => {
                let song_name = x.name.replace(" ", "-");
                let artist_name = x.album.artists.get(0).unwrap().name.replace(" ", "-");
                let song_path = format!("lyrics/{1}/{0}", song_name, artist_name);      
                tokio::task::spawn_blocking(move || {
                    println!("{}", song_path);
                    single_song_scrap(song_path);
                });
            },       
            PlayableItem::Episode(x) => println!("{} Description:{}", x.name, x.description),
            PlayableItem::Unknown(_) => println!("unavailable", )

    }; 
    }
}
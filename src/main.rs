use clap::{App, Arg};
use dirs;
use rspotify::client::{Spotify, SpotifyBuilder};
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder};
use std::path::PathBuf;

async fn get_playing(spotify: Spotify) -> Result<(), Box<dyn std::error::Error>> {
    let playing = spotify.current_user_playing_track().await?;
    if let Some(p) = playing {
        if let Some(pi) = p.item {
            match pi {
                rspotify::model::PlayingItem::Track(f) => {
                    if f.artists.len() > 0 {
                        println!("{} - {}", f.artists[0].name, f.name);
                    } else {
                        println!("{}", f.name);
                    }
                }
                rspotify::model::PlayingItem::Episode(e) => {
                    println!("{} - {}", e.show.name, e.name);
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Spotify Control")
        .version("0.1")
        .author("Boris Faure <boris.faure@gmail.com>")
        .about("My own dumb spotify controller")
        .arg(
            Arg::with_name("CMD")
                .help("Command to run")
                .index(1)
                .possible_values(&["get"])
                .required(true),
        )
        .get_matches();

    let creds = CredentialsBuilder::from_env().build().unwrap();
    let oauth = OAuthBuilder::from_env()
        .scope("user-read-playback-state user-modify-playback-state")
        .build()
        .unwrap();
    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    spotify.cache_path = [dirs::config_dir().unwrap(), PathBuf::from(".spotify.token")]
        .iter()
        .collect();

    // Obtaining the access token
    spotify.prompt_for_user_token().await.unwrap();

    get_playing(spotify).await
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_presence::DiscordPresence;
use song::Song;
use tauri::{Manager, State};

mod discord_presence;
mod song;

#[tauri::command]
fn on_new_song_playing(song: serde_json::Value, discord_presence: State<'_, DiscordPresence>) {
    if let Some(current_song) = song.get("current_song") {
        if let Ok(song) = serde_json::from_value::<Song>(current_song.clone()) {
            let discord_presence = discord_presence.clone();

            _ = discord_presence.update_presence(song);
        } else {
            println!("Couldn't parse received song data: {}", current_song);
        }
    } else {
        println!(
            "Received song data doesn't contain a 'current' key: {}",
            song
        );
    }
}

#[tokio::main]
async fn main() {
    let mut discord_presence = DiscordPresence::new(1105865495875424376);
    _ = discord_presence.start();

    tauri::Builder::default()
        .manage(discord_presence)
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            window
                .eval(
                    r#"

                    let current_song = null

                    function onNewSongPlaying() {
                        const invoke = window.__TAURI__.invoke
                        invoke('on_new_song_playing', { "song": {current_song} });
                    }

                    function parseSongInfo() {

                        // "image": player.thumbnail_.thumbnails[5].url

                        if (player.playerApi_.getVideoData().title === '' || player.playerApi_.getVideoData().author === '') {
                            return;
                        }

                        let song = null;

                        try { 
                            song = { "title": player.playerApi_.getVideoData().title, "author": player.playerApi_.getVideoData().author, "image": player.thumbnail_.thumbnails[5].url }
                        } catch(e) { 
                            song = { "title": player.playerApi_.getVideoData().title, "author": player.playerApi_.getVideoData().author, "image": null }
                        }

                        if (current_song === null) {
                            current_song = song
                            onNewSongPlaying(song);
                        }

                        if (song.title === current_song.title && song.author === current_song.author) {
                            return;
                        }

                        current_song = song
                        onNewSongPlaying();
                    }

                    // call the function every second
                    setInterval(parseSongInfo, 1000);
                "#,
                )
                .expect("Failed to inject script");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![on_new_song_playing])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::discord_presence::DiscordPresence;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Song {
    pub title: String,
    pub author: String,
    pub image: Option<String>,
}

#[tauri::command]
pub fn on_new_song_playing(song: serde_json::Value, discord_presence: State<'_, DiscordPresence>) {
    if let Some(current_song) = song.get("current_song") {
        if let Ok(song) = serde_json::from_value::<Song>(current_song.clone()) {
            let discord_presence = discord_presence.clone();
            _ = discord_presence.update_presence(song);
        }
    }
}

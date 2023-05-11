use std::sync::Mutex;

use discord_rich_presence::{
    activity::{self, Assets},
    DiscordIpc, DiscordIpcClient,
};

use crate::song::Song;

pub struct DiscordPresence {
    pub client: Mutex<DiscordIpcClient>,
}

impl DiscordPresence {
    pub fn new(application_id: u64) -> Self {
        let client = Mutex::new(DiscordIpcClient::new(&application_id.to_string()).unwrap());
        Self { client }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        _ = self.client.lock().unwrap().connect();
        Ok(())
    }

    pub fn update_presence(&self, song: Song) -> Result<(), Box<dyn std::error::Error>> {
        let details = song.title.clone();
        let state = format!("By {}", song.author);

        if let Some(image) = &song.image {
            self.client.lock().unwrap().set_activity(
                activity::Activity::new()
                    .state(&state)
                    .details(&details)
                    .assets(Assets::new().large_image(image).large_text(&details)),
            )?;
        } else {
            self.client.lock().unwrap().set_activity(
                activity::Activity::new()
                    .state(&state)
                    .assets(Assets::new().small_text(&details))
                    .details(&details),
            )?;
        };

        Ok(())
    }
}

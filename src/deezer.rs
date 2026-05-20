use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeezerPlaylist {
    pub id: u32,
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlaylistDto {
    pub id: u32,
}

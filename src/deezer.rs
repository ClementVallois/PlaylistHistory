use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlaylistDto {
    pub id: PlaylistId,
    pub title: PlaylistName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct PlaylistId(u64);

impl PlaylistId {
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct PlaylistName(String);

impl PlaylistName {
    pub fn value(&self) -> &str {
        &self.0
    }
}

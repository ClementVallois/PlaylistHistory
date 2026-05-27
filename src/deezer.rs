use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlaylistDto {
    pub id: PlaylistId,
    pub title: PlaylistTitle,
    pub tracks: PlaylistTracksData,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistTracksData {
    pub data: Vec<TrackDto>,
}

#[derive(Deserialize, Debug)]
pub struct TrackDto {
    pub id: TrackId,
    pub title: TrackTitle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct PlaylistId(u64);

impl PlaylistId {
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistTitle(String);

impl PlaylistTitle {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct TrackId(u64);

impl TrackId {
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackTitle(String);

impl TrackTitle {
    pub fn value(&self) -> &str {
        &self.0
    }
}

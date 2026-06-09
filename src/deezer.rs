use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct PlaylistDto {
    pub id: PlaylistId,
    pub title: PlaylistTitle,
    pub tracks: TracksWrapper,
}

#[derive(Deserialize, Debug)]
pub struct TracksWrapper {
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
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistTitle(String);

impl PlaylistTitle {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct TrackId(u64);

impl TrackId {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackTitle(String);

impl TrackTitle {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrackTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

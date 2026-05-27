mod deezer;

use deezer::PlaylistDto;
use std::fs;

const DEEZER_PLAYLIST_FILE_PATH: &str = "DeezerPlaylist.json";
fn main() -> anyhow::Result<()> {
    let playlist_json = fs::read_to_string(DEEZER_PLAYLIST_FILE_PATH)?;
    let playlist: PlaylistDto = serde_json::from_str(&playlist_json)?;
    println!("playlist id is: {}", playlist.id.value());
    println!("playlist title is: {}", playlist.title.value());
    let tracks_ids: Vec<u64> = playlist.tracks.data.iter().map(|t| t.id.value()).collect();
    println!("playlist track ids are: {:#?}", tracks_ids);
    Ok(())
}

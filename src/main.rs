mod deezer;

use deezer::PlaylistDto;
use std::fs;

const DEEZER_PLAYLIST_FILE_PATH: &str = "DeezerPlaylist.json";
fn main() -> anyhow::Result<()> {
    let playlist_json = fs::read_to_string(DEEZER_PLAYLIST_FILE_PATH)?;
    let playlist: PlaylistDto = serde_json::from_str(&playlist_json)?;
    println!("playlist id is: {}", playlist.id.as_u64());
    println!("playlist title is: {}", playlist.title.as_str());
    let tracks_ids: Vec<u64> = playlist.tracks.data.iter().map(|t| t.id.as_u64()).collect();
    println!("playlist track ids are: {:#?}", tracks_ids);

    for track in &playlist.tracks.data {
        println!("{} - {}", track.id, track.title)
    }
    Ok(())
}

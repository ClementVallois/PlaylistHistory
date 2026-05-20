use std::fs;

mod deezer;
use deezer::PlaylistDto;

const DEEZER_PLAYLIST_FILE_PATH: &str = "./DeezerPlaylist.json";
fn main() -> anyhow::Result<()> {
    let playlist_json = fs::read_to_string(DEEZER_PLAYLIST_FILE_PATH)?;
    let playlist: PlaylistDto = serde_json::from_str(&playlist_json)?;
    println!("playlist id is: {:#?}", playlist.id);
    Ok(())
}

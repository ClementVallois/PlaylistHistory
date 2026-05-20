use std::fs;

mod deezer;
use deezer::DeezerPlaylist;

fn main() -> anyhow::Result<()> {
    let playlist_data = fs::read_to_string("./DeezerPlaylist.json")?;
    let deezer_playlist: DeezerPlaylist = serde_json::from_str(&playlist_data)?;
    println!("playlist id is: {}", deezer_playlist.id);
    Ok(())
}

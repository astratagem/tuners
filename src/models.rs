use std::path::PathBuf;

/// A single audio file with extracted metadata.
#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<u32>,
    pub total_tracks: Option<u32>,
    pub disc_number: Option<u32>,
    pub total_discs: Option<u32>,
    pub genre: Option<String>,
    pub duration: Option<u32>,
}

/// A cluser of files that are likely to belong to the same album.
///
/// TODO: Define "likely"?
/// TODO: handle multi-disc albums
#[derive(Debug, Clone)]
pub struct AlbumCluster {
    pub album: String,
    pub album_artist: String,
    pub tracks: Vec<AudioFile>,
    pub base_path: PathBuf,
    pub total_discs: u32,
}

impl AlbumCluster {
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

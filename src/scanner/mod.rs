use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use walkdir::WalkDir;

use crate::models::{AlbumCluster, AudioFile};

mod metadata;

const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "flac"];

const UNKNOWN_ARTIST_NAME: &str = "Unknown Artist";

const UNKNOWN_ALBUM_NAME: &str = "Unknown Album";

const DEFAULT_TOTAL_DISCS: u8 = 1;

/// Scan a directory recursively for audio files and extract their
/// metadata.
pub fn scan_directory(path: &Path) -> Result<Vec<AudioFile>> {
    let mut files: Vec<AudioFile> = Vec::new();

    for entry in WalkDir::new(path).follow_links(false) {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if is_supported_audio_file(path) {
            if let Ok(file) = metadata::extract(path) {
                files.push(file)
            }
        }
    }

    Ok(files)
}

fn is_supported_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|v| v.to_str())
        .map(|it| SUPPORTED_AUDIO_EXTENSIONS.contains(&it.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn cluster_files(files: Vec<AudioFile>) -> Vec<AlbumCluster> {
    let mut clusters: HashMap<ClusterKey, Vec<AudioFile>> = HashMap::new();

    for file in files {
        let key = ClusterKey::from_file(&file);
        clusters.entry(key).or_insert_with(Vec::new).push(file);
    }

    clusters
        .into_iter()
        .map(|(key, mut tracks)| {
            // FIXME: handle multi-disc
            tracks.sort_by_key(|it| it.track_number);
            AlbumCluster {
                album_artist: key.album_artist,
                album: key.album,
                tracks,
                base_path: key.base_path,
                total_discs: key.total_discs,
            }
        })
        .collect()
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct ClusterKey {
    base_path: PathBuf,
    album_artist: String,
    album: String,
    total_discs: u32,
}

impl ClusterKey {
    fn from_file(file: &AudioFile) -> Self {
        let base_path = file.path.parent().unwrap_or(Path::new("")).to_path_buf();
        let album_artist = file
            .album_artist
            .clone()
            .unwrap_or_else(|| UNKNOWN_ARTIST_NAME.to_string());
        let album = file
            .album
            .clone()
            .unwrap_or_else(|| UNKNOWN_ALBUM_NAME.to_string());
        let total_discs = file
            .total_discs
            .clone()
            .unwrap_or_else(|| DEFAULT_TOTAL_DISCS as u32);
        Self {
            base_path,
            album_artist,
            album,
            total_discs,
        }
    }
}

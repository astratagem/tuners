// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc::{Sender, SyncSender},
};

use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use rayon::prelude::*;

use crate::models::{AlbumCluster, AudioFile};

mod metadata;

const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "flac"];

const UNKNOWN_ARTIST_NAME: &str = "Unknown Artist";

const UNKNOWN_ALBUM_NAME: &str = "Unknown Album";

const DEFAULT_TOTAL_DISCS: u8 = 1;

pub struct ScanProgress {
    pub current_dir: String,
    pub clusters_found: usize,
}

/// Scan a directory recursively for audio files and extract their
/// metadata.
pub fn scan_directory(
    path: &Path,
    cluster_tx: SyncSender<AlbumCluster>,
    progress_tx: Option<Sender<ScanProgress>>,
) -> Result<()> {
    let mut clusters_found = 0;
    scan_directory_recursive(path, &cluster_tx, &progress_tx, &mut clusters_found)?;
    Ok(())
}

pub fn scan_directory_recursive(
    path: &Path,
    cluster_tx: &SyncSender<AlbumCluster>,
    progress_tx: &Option<Sender<ScanProgress>>,
    clusters_found: &mut usize,
) -> Result<()> {
    let entries =
        std::fs::read_dir(path).context(format!("Failed to read directory: {}", path.display()))?;
    let mut files = Vec::new();
    let mut subdirs = Vec::new();

    // Separate files and directories.
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if is_hidden(&path) {
                continue;
            }
            subdirs.push(path);
        } else if path.is_file() && is_supported_audio_file(&path) {
            files.push(path);
        }
    }

    // Process subdirectories first (depth-first).
    for subdir in subdirs {
        scan_directory_recursive(&subdir, cluster_tx, progress_tx, clusters_found)?;
    }

    // Process files in the current directory.
    if !files.is_empty() {
        let audio_files: Vec<AudioFile> = files
            .par_iter()
            .filter_map(|it| metadata::extract(it).ok())
            .collect();

        if !audio_files.is_empty() {
            let clusters = cluster_files(audio_files);

            for cluster in clusters {
                cluster_tx
                    .send(cluster)
                    .context("Failed to send cluster to queue")?;
                *clusters_found += 1;
                if let Some(tx) = progress_tx {
                    let _ = tx.send(ScanProgress {
                        current_dir: path.display().to_string(),
                        clusters_found: *clusters_found,
                    });
                }
            }
        }
    }

    Ok(())
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
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
        clusters.entry(key).or_default().push(file);
    }

    clusters
        .into_iter()
        .map(|(key, mut tracks)| {
            tracks.sort_by_key(|it| (it.disc_number.unwrap_or(1), it.track_number.unwrap_or(0)));
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
        let total_discs = file.total_discs.unwrap_or(DEFAULT_TOTAL_DISCS as u32);
        Self {
            base_path,
            album_artist,
            album,
            total_discs,
        }
    }
}

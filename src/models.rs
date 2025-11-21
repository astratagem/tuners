// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::PathBuf;

use crate::codecs::AudioCodec;

/// A single audio file with extracted metadata.
#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: PathBuf,
    pub codec: AudioCodec,
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

    /// Get the audio codec shared by all files in the cluster, if any.
    pub fn codec(&self) -> Option<AudioCodec> {
        let first_track = self.tracks.first()?;
        let codec = first_track.codec.clone();
        if !self.tracks.iter().all(|it| it.codec == codec) {
            return None;
        }
        Some(codec)
    }
}

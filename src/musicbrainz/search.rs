// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::sync::mpsc::Sender;

use color_eyre::eyre::Result;
pub use musicbrainz_rs::entity::release::Release;

use crate::{models::AlbumCluster, musicbrainz::client::Client};

pub enum SearchMessage {
    Searching(AlbumCluster, String),
    Results(AlbumCluster, Vec<Release>),
    NoResults(AlbumCluster),
    Error(AlbumCluster, String),
}

pub async fn search_for_cluster(
    client: &mut Client,
    tx: Sender<SearchMessage>,
    cluster: AlbumCluster,
) -> Result<Vec<Release>> {
    let AlbumCluster {
        album_artist,
        album,
        ..
    } = &cluster;
    let status = format!("Searching for {} - {}...", album_artist, album);

    let _ = tx.send(SearchMessage::Searching(cluster.clone(), status));

    match client.search_release(&album_artist, &album).await {
        Ok(releases) => {
            if releases.is_empty() {
                let _ = tx.send(SearchMessage::NoResults(cluster));
            } else {
                let _ = tx.send(SearchMessage::Results(cluster, releases.clone()));
            }
            Ok(releases)
        }
        Err(e) => {
            let msg = format!("Search failed: {}", e);
            let _ = tx.send(SearchMessage::Error(cluster, msg));
            Err(e)
        }
    }
}

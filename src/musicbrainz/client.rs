// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::{Duration, Instant};

use color_eyre::eyre::{Result, eyre};
use musicbrainz_rs::{
    entity::release::{Release, ReleaseSearchQuery},
    prelude::*,
};
use tokio::time::sleep;

const RATE_LIMIT: Duration = Duration::from_secs(1);

pub struct Client {
    last_request: Option<Instant>,
}

impl Client {
    pub fn new() -> Self {
        Self { last_request: None }
    }

    /// Enforce the API's rate limit.
    async fn throttle(&mut self) {
        if let Some(last) = self.last_request {
            let elapsed = last.elapsed();
            if elapsed < RATE_LIMIT {
                let wait = RATE_LIMIT - elapsed;
                sleep(wait).await;
            }
        }
        self.last_request = Some(Instant::now());
    }

    /// Search for releases by album artist and album.
    pub async fn search_release(
        &mut self,
        album_artist: &str,
        album: &str,
    ) -> Result<Vec<Release>> {
        self.throttle().await;

        let query = ReleaseSearchQuery::query_builder()
            .artist_name(album_artist)
            .and()
            .release(album)
            .build();

        let result = Release::search(query)
            .execute()
            .await
            .map_err(|e| eyre!("MusicBrainz API error: {}", e))?;

        Ok(result.entities)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{collections::VecDeque, io::Stdout, path::PathBuf, sync::mpsc, thread, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use musicbrainz_rs::entity::release::Release;
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    models::{AlbumCluster, AudioFile},
    musicbrainz::{
        client::Client as MbClient,
        search::{SearchMessage, search_for_cluster},
    },
    scanner::{self, ScanProgress},
    ui,
};

const TICK_RATE: Duration = Duration::from_millis(100);

struct PendingCluster {
    cluster: AlbumCluster,
    results: Vec<Release>,
}

pub struct App {
    pending_clusters: VecDeque<PendingCluster>,
    state: AppState,
    should_quit: bool,
    scan_rx: Option<mpsc::Receiver<ScanMessage>>,
    scan_progress_rx: Option<mpsc::Receiver<ScanProgress>>,
    search_rx: Option<mpsc::Receiver<SearchMessage>>,
}

#[derive(Debug)]
pub enum AppState {
    Scanning {
        path: PathBuf,
        files_found: Vec<AudioFile>,
        current_file: Option<String>,
        is_complete: bool,
    },
    AutoTagging {
        cluster: AlbumCluster,
        results: Vec<Release>,
        selected_idx: usize,
    },
    ClusterList {
        clusters: Vec<AlbumCluster>,
        selected_idx: usize,
    },
    Error {
        message: String,
    },
}

enum ScanMessage {
    Complete(Vec<AudioFile>),
    Error(String),
}

struct AutoTagging {
    cluster: AlbumCluster,
    results: Option<Vec<Release>>,
    selected_idx: usize,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(path: PathBuf) -> Self {
        Self {
            pending_clusters: VecDeque::new(),
            state: AppState::Scanning {
                path,
                files_found: Vec::new(),
                current_file: None,
                is_complete: false,
            },
            should_quit: false,
            scan_rx: None,
            scan_progress_rx: None,
            search_rx: None,
        }
    }

    /// Run the main application loop.
    pub fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
    ) -> color_eyre::Result<()> {
        self.start_scan();

        while !self.should_quit {
            terminal.draw(|frame| ui::render(frame, &self.state))?;

            self.handle_messages();

            if event::poll(TICK_RATE)? {
                self.handle_events()?;
            }
        }

        Ok(())
    }

    fn start_scan(&mut self) {
        if let AppState::Scanning { path, .. } = &self.state {
            let scan_path = path.clone();
            let (cluster_tx, cluster_rx) = mpsc::sync_channel(5);
            let (progress_tx, progress_rx) = mpsc::channel();
            let (search_tx, search_rx) = mpsc::channel();

            thread::spawn(move || {
                let _ = scanner::scan_directory(&scan_path, cluster_tx, Some(progress_tx));
            });

            thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();

                rt.block_on(async {
                    let mut client = MbClient::new();

                    while let Ok(cluster) = cluster_rx.recv() {
                        let result =
                            search_for_cluster(&mut client, search_tx.clone(), cluster).await;
                        if let Err(e) = result {
                            eprintln!("Search error: {}", e);
                        }
                    }
                })
            });

            self.scan_progress_rx = Some(progress_rx);
            self.search_rx = Some(search_rx);
        }
    }

    fn handle_messages(&mut self) {
        if let Some(rx) = &self.scan_progress_rx {
            while let Ok(progress) = rx.try_recv() {
                if let AppState::Scanning { current_file, .. } = &mut self.state {
                    *current_file = Some(format!(
                        "Scanning: {} ({} clusters found)",
                        progress.current_dir, progress.clusters_found
                    ));
                }
            }
        }

        // Collect search messages first to avoid borrow issues.
        let mut messages = Vec::new();
        if let Some(rx) = &self.search_rx {
            while let Ok(message) = rx.try_recv() {
                messages.push(message);
            }
        }

        for message in messages {
            match message {
                SearchMessage::Searching(_cluster, status) => {
                    if let AppState::Scanning { current_file, .. } = &mut self.state {
                        *current_file = Some(format!("🔍 {}", status));
                    }
                }
                SearchMessage::Results(cluster, releases) => {
                    self.pending_clusters.push_back(PendingCluster {
                        cluster,
                        results: releases,
                    });

                    if !matches!(self.state, AppState::AutoTagging { .. }) {
                        self.show_next_cluster();
                    }
                }
                SearchMessage::NoResults(cluster) => {
                    // todo!("Prompt user for manual search or skip etc.");
                    if let AppState::Scanning { current_file, .. } = &mut self.state {
                        *current_file = Some(format!(
                            "∅ No matches for {} - {}",
                            cluster.album_artist, cluster.album
                        ));
                    }
                }
                SearchMessage::Error(_cluster, msg) => {
                    if let AppState::Scanning { current_file, .. } = &mut self.state {
                        *current_file = Some(format!("⚠ Error: {}", msg));
                    }
                    // todo!("Show error to user");
                }
            }
        }

        if let Some(rx) = &self.scan_rx
            && let Ok(message) = rx.try_recv()
        {
            match message {
                ScanMessage::Complete(files) => self.complete_scan(files),
                ScanMessage::Error(msg) => self.set_error(msg),
            }
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            let is_ctrl_c = matches!(key.code, KeyCode::Char('c'))
                && key.modifiers.contains(KeyModifiers::CONTROL);
            let should_quit = is_ctrl_c || matches!(key.code, KeyCode::Char('q'));
            if should_quit {
                self.should_quit = true;
                return Ok(());
            }

            match &self.state {
                AppState::Scanning {
                    files_found,
                    is_complete,
                    ..
                } => {
                    if matches!(key.code, KeyCode::Enter) && *is_complete {
                        let files = files_found.clone();
                        let clusters = scanner::cluster_files(files);
                        self.state = AppState::ClusterList {
                            clusters,
                            selected_idx: 0,
                        };
                    }
                }
                AppState::AutoTagging { .. } => match key.code {
                    KeyCode::Char('k') | KeyCode::Up => self.select_previous_match(),
                    KeyCode::Char('j') | KeyCode::Down => self.select_next_match(),
                    KeyCode::Char('A') => self.handle_apply(),
                    KeyCode::Char('s') => self.handle_skip(),
                    KeyCode::Char('M') => self.handle_manual_search(),
                    _ => {}
                },
                AppState::ClusterList { .. } => match key.code {
                    KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => self.select_next(),
                    KeyCode::Enter => {
                        todo!("Start MusicBrainz search for selected cluster");
                    }
                    _ => {}
                },
                AppState::Error { .. } => {
                    // Error state only allows quitting.
                }
            }
        }

        Ok(())
    }

    fn complete_scan(&mut self, files: Vec<AudioFile>) {
        if let AppState::Scanning { path, .. } = &self.state {
            let path = path.clone();
            self.state = AppState::Scanning {
                path,
                files_found: files,
                current_file: None,
                is_complete: true,
            };
        }
    }

    fn show_next_cluster(&mut self) {
        if let Some(pending) = self.pending_clusters.pop_front() {
            self.state = AppState::AutoTagging {
                cluster: pending.cluster,
                results: pending.results,
                selected_idx: 0,
            };
        }
    }

    fn handle_apply(&mut self) {
        // TODO: do something
        self.show_next_cluster();
    }

    fn handle_skip(&mut self) {
        self.show_next_cluster();
    }

    fn handle_manual_search(&mut self) {
        // TODO: Prompt for manual search
        self.show_next_cluster();
    }

    fn select_next_match(&mut self) {
        if let AppState::AutoTagging {
            results,
            selected_idx,
            ..
        } = &mut self.state
        {
            if !results.is_empty() {
                *selected_idx = (*selected_idx + 1).min(results.len() - 1);
            }
        }
    }

    fn select_previous_match(&mut self) {
        if let AppState::AutoTagging {
            results,
            selected_idx,
            ..
        } = &mut self.state
        {
            if !results.is_empty() {
                *selected_idx = selected_idx.saturating_sub(1);
            }
        }
    }

    fn select_next(&mut self) {
        if let AppState::ClusterList {
            clusters,
            selected_idx,
        } = &mut self.state
            && !clusters.is_empty()
        {
            *selected_idx = (*selected_idx + 1).min(clusters.len() - 1);
        }
    }

    fn select_previous(&mut self) {
        if let AppState::ClusterList { selected_idx, .. } = &mut self.state {
            *selected_idx = selected_idx.saturating_sub(1);
        }
    }

    fn set_error(&mut self, message: String) {
        self.state = AppState::Error { message };
    }
}

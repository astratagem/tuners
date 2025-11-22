// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{app::AppState, codecs::codec_name, credit::UNKNOWN_ARTIST_NAME, models::AlbumCluster};
use musicbrainz_rs::entity::release::Release;
use ratatui::{prelude::*, widgets::*};
use ratatui_macros::vertical;

const SECONDS_PER_MINUTE: u32 = 60;
const SECONDS_PER_HOUR: u32 = 3600;

const HIGHLIGHT_SYMBOL: &str = "» ";

pub fn render(frame: &mut Frame, state: &AppState) {
    match state {
        AppState::Scanning {
            path,
            files_found,
            current_file,
            is_complete,
        } => render_scanning(frame, path, files_found, current_file, *is_complete),
        AppState::AutoTagging {
            cluster,
            results,
            selected_idx,
        } => render_autotagging(frame, cluster, results, *selected_idx),
        AppState::ClusterList {
            clusters,
            selected_idx,
        } => render_clusters(frame, clusters, *selected_idx),
        AppState::Error { message } => render_error(frame, message),
    }
}

fn render_scanning(
    frame: &mut Frame,
    path: &std::path::Path,
    files: &[crate::models::AudioFile],
    current: &Option<String>,
    is_complete: bool,
) {
    let [header_area, main_area, footer_area] = vertical![==3, >=5, ==3].areas(frame.area());

    let header = Paragraph::new(format!("Scanning: {}", path.display())).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Directory Scanner"),
    );
    frame.render_widget(header, header_area);

    let status = if is_complete {
        format!(
            "Scan complete.  Found {} audio files.\n\nPress Enter to continue...",
            files.len()
        )
    } else if let Some(current_file) = current {
        current_file.clone()
    } else {
        format!("Found {} audio files so far...", files.len())
    };

    let content = Paragraph::new(status)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: true });

    frame.render_widget(content, main_area);

    let help = if is_complete {
        "<RET> : Continue to clusters... | q : Quit"
    } else {
        "Scanning... | <q> : Quit"
    };

    let footer = Paragraph::new(help).block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, footer_area);
}

fn render_autotagging(
    frame: &mut Frame,
    cluster: &AlbumCluster,
    results: &[Release],
    selected_idx: usize,
) {
    let [header_area, main_area, footer_area] = vertical![==5, >=10, ==3].areas(frame.area());

    let cluster_info = format!(
        "Album Artist: {}\nAlbum: {}\nTracks: {}\nPath: {}",
        cluster.album_artist,
        cluster.album,
        cluster.tracks.len(),
        cluster.base_path.display()
    );
    let header = Paragraph::new(cluster_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Cluster"),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(header, header_area);

    if results.is_empty() {
        let no_results =
            Paragraph::new("No matches found\n\nPress [m] for manual search or [s] to skip")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Search Results"),
                )
                .wrap(Wrap { trim: true });
        frame.render_widget(no_results, main_area);
    } else {
        let items: Vec<ListItem> = results
            .iter()
            .enumerate()
            .map(render_search_result)
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Found {} matches", results.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(HIGHLIGHT_SYMBOL);

        let mut state = ListState::default();
        state.select(Some(selected_idx));
        frame.render_stateful_widget(list, main_area, &mut state);
    }

    let help =
        Paragraph::new("j/k or ↑/↓ : Navigate | [a]pply | [s]kip | [m]anual search | q : Quit")
            .block(Block::default().borders(Borders::ALL).title("Actions"));
    frame.render_widget(help, footer_area);
}

fn render_search_result(result: (usize, &Release)) -> ListItem<'_> {
    let (idx, release) = result;
    let artist = release
        .artist_credit
        .as_ref()
        .and_then(|ac| ac.first())
        .and_then(|a| Some(a.name.clone()))
        .unwrap_or_else(|| UNKNOWN_ARTIST_NAME.to_string());
    let date = release
        .date
        .as_ref()
        .map(|it| it.0.as_str())
        .unwrap_or("????");
    let track_count = release
        .media
        .as_ref()
        .map(|media| media.iter().map(|it| it.track_count).sum::<u32>())
        .unwrap_or(0);
    let text = format!(
        "{}. {} - {} ({}) [Tracks: {}] [Country: {}]",
        idx + 1,
        artist,
        release.title,
        date,
        track_count,
        release.country.clone().unwrap_or(String::from("??"))
    );

    ListItem::new(text)
}

fn render_clusters(
    frame: &mut Frame,
    clusters: &[crate::models::AlbumCluster],
    selected_idx: usize,
) {
    let [header_area, results_area, tracklist_area, footer_area] =
        vertical![==3, >=10, ==12, ==3].areas(frame.area());

    let header = Paragraph::new(format!("Found {} album clusters", clusters.len())).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Album Clusters"),
    );
    frame.render_widget(header, header_area);

    let items: Vec<ListItem> = clusters
        .iter()
        .map(|it| {
            ListItem::new(format!(
                "{} - {} [{}] ({} tracks) [{}]",
                it.album_artist,
                it.album,
                match it.codec() {
                    Some(codec) => codec_name(codec),
                    None => String::from("Mutt"),
                },
                it.track_count(),
                it.base_path.display()
            ))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Clusters"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("» ");
    let mut state = ListState::default();
    state.select(Some(selected_idx));
    frame.render_stateful_widget(list, results_area, &mut state);

    if let Some(cluster) = clusters.get(selected_idx) {
        let tracklist: Vec<Line> = cluster
            .tracks
            .iter()
            .map(|it| {
                Line::raw(format!(
                    "{}{}{} ({})\n",
                    it.disc_number.map_or(String::new(), |n| {
                        if it.total_discs.unwrap_or(1) <= 1 {
                            return String::new();
                        }
                        format!("{:02}-", n)
                    }),
                    it.track_number
                        .map_or(String::new(), |n| format!("{:02}. ", n)),
                    it.title.clone().unwrap_or_default(),
                    it.duration
                        .map_or(String::from("???"), |n| seconds_to_timecode(
                            n.as_secs() as u32
                        )),
                ))
            })
            .collect();
        frame.render_widget(Paragraph::new(tracklist), tracklist_area);
    } else {
        frame.render_widget(Clear, tracklist_area);
    }

    let help = Paragraph::new("j/k : Navigate | <RET> : Lookup (TODO) | q : Quit")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, footer_area);
}

fn render_error(frame: &mut Frame, message: &str) {
    let [header_area, main_area, footer_area] = vertical![==3, >=5, ==3].areas(frame.area());

    let header = Paragraph::new("Error")
        .block(Block::default().borders(Borders::ALL).title("Error"))
        .style(Style::default().fg(Color::Red));
    frame.render_widget(header, header_area);

    let error = Paragraph::new(message)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error, main_area);

    let footer = Paragraph::new("q : Quit").block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, footer_area);
}

pub fn seconds_to_timecode(seconds: u32) -> String {
    let hours = seconds / SECONDS_PER_HOUR;
    let minutes = (seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds = seconds % SECONDS_PER_MINUTE;
    format!(
        "{}{:02}:{:02}",
        if hours > 0 {
            format!("{}:", hours)
        } else {
            String::new()
        },
        minutes,
        seconds
    )
}

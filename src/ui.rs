// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{app::AppState, codecs::codec_name};
use ratatui::{prelude::*, widgets::*};
use ratatui_macros::vertical;

pub fn render(frame: &mut Frame, state: &AppState) {
    match state {
        AppState::Scanning {
            path,
            files_found,
            current_file,
            is_complete,
        } => render_scanning(frame, path, files_found, current_file, *is_complete),
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
        format!(
            "Found {} audio files so far...\n\nCurrently scanning: {}",
            files.len(),
            current_file
        )
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

    let selected_cluster = clusters.get(selected_idx);
    if selected_cluster.is_some() {
        let tracklist: Vec<Line> = selected_cluster
            .unwrap()
            .tracks
            .iter()
            .map(|it| {
                Line::raw(format!(
                    "{}{}{} ({})\n",
                    it.disc_number
                        .map_or(String::new(), |n| format!("{}-", n.to_string())),
                    it.track_number
                        .map_or(String::new(), |n| format!("{}. ", n.to_string())),
                    it.title.clone().unwrap_or_default(),
                    it.duration
                        .map_or(String::from("???"), |n| n.as_secs().to_string()),
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

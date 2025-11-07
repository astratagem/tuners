use crate::app::AppState;
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
        // FIXME: do something better
        AppState::Idle => render_error(frame, "App is idle but don't worry!"),
    }
}

fn render_scanning(
    frame: &mut Frame,
    path: &std::path::Path,
    files: &[crate::models::AudioFile],
    current: &Option<String>,
    is_complete: bool,
) {
    let chunks = vertical![==3, >=5, ==3].split(frame.area());
    let header = Paragraph::new(format!("Scanning: {}", path.display())).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Directory Scanner"),
    );
    frame.render_widget(header, chunks[0]);

    let status = if is_complete {
        format!(
            "Scan complete.  Found {} audio files.\n\nPress Enter to continue...",
            files.len()
        )
    } else if let Some(ref current_file) = current {
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

    frame.render_widget(content, chunks[1]);

    let help = if is_complete {
        "<RET> : Continue to clusters... | q : Quit"
    } else {
        "Scanning... | <q> : Quit"
    };

    let footer = Paragraph::new(help).block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, chunks[2]);
}

fn render_clusters(
    frame: &mut Frame,
    clusters: &[crate::models::AlbumCluster],
    selected_idx: usize,
) {
    let chunks = vertical![==3, >=10, ==3].split(frame.area());

    let header = Paragraph::new(format!("Found {} album clusters", clusters.len())).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Album Clusters"),
    );
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = clusters
        .iter()
        .map(|it| {
            ListItem::new(format!(
                "{} - {} ({} tracks) [{}]",
                it.album_artist,
                it.album,
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
    frame.render_stateful_widget(list, chunks[1], &mut state);

    let help = Paragraph::new("j/k : Navigate | <RET> : Lookup (TODO) | q : Quit")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}

fn render_error(frame: &mut Frame, message: &str) {
    let chunks = vertical![==3, >=5, ==3].split(frame.area());

    let header = Paragraph::new("Error")
        .block(Block::default().borders(Borders::ALL).title("Error"))
        .style(Style::default().fg(Color::Red));
    frame.render_widget(header, chunks[0]);

    let error = Paragraph::new(message)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error, chunks[1]);

    let footer = Paragraph::new("q : Quit").block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}

use std::{env, path::PathBuf};

use color_eyre::eyre::bail;
use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;

mod app;
mod codecs;
mod models;
mod scanner;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let scan_path = parse_args()?;
    let mut app = app::App::new(scan_path);
    let app_res = app.run(terminal);
    ratatui::restore();
    app_res
}

fn parse_args() -> Result<PathBuf> {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().context("Failed to get current directory")?
    };

    if !path.exists() {
        bail!("Path '{}' does not exist", path.display());
    }

    if !path.is_dir() {
        bail!("Path '{}' is not a directory", path.display());
    }

    Ok(path)
}

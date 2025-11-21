use crate::{codecs::AudioCodec, models::AudioFile};

use color_eyre::Result;
use id3::TagLike;
use std::path::Path;

/// Extract metadata from an audio file.
pub fn extract(path: &Path) -> Result<AudioFile> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    let res = match ext.as_str() {
        "mp3" => extract_mp3(path)?,
        "m4a" => extract_mp4(path)?,
        "flac" => extract_flac(path)?,
        // FIXME: provide some kind of logging for these, or prompt?
        _ => todo!(),
    };

    Ok(res)
}

fn extract_mp3(path: &Path) -> Result<AudioFile> {
    let tag = id3::Tag::read_from_path(path)?;
    Ok(AudioFile {
        path: path.to_path_buf(),
        codec: AudioCodec::Mp3,
        artist: tag.artist().map(String::from),
        title: tag.title().map(String::from),
        album_artist: tag.album_artist().map(String::from),
        album: tag.album().map(String::from),
        track_number: tag.track(),
        total_tracks: tag.total_tracks(),
        disc_number: tag.disc(),
        total_discs: tag.total_discs(),
        genre: tag.genre().map(String::from),
        duration: tag.duration(),
    })
}

fn extract_mp4(path: &Path) -> Result<AudioFile> {
    let tag = mp4ameta::Tag::read_from_path(path)?;
    Ok(AudioFile {
        path: path.to_path_buf(),
        codec: AudioCodec::Mp4,
        artist: tag.artist().map(String::from),
        title: tag.title().map(String::from),
        album_artist: tag.album_artist().map(String::from),
        album: tag.album().map(String::from),
        track_number: tag.track_number().and_then(|n| Some(n as u32)),
        total_tracks: tag.total_tracks().and_then(|n| Some(n as u32)),
        disc_number: tag.disc_number().and_then(|n| Some(n as u32)),
        total_discs: tag.total_discs().and_then(|n| Some(n as u32)),
        genre: tag.genre().map(String::from),
        duration: Some(tag.duration().as_secs() as u32),
    })
}

fn extract_flac(path: &Path) -> Result<AudioFile> {
    let tag = metaflac::Tag::read_from_path(path)?;
    let vorbis = tag.vorbis_comments();

    let artist = vorbis
        .and_then(|v| v.artist())
        .and_then(|v| v.iter().next())
        .map(String::from);

    let album_artist = vorbis
        .and_then(|v| v.album_artist())
        .and_then(|v| v.iter().next())
        .map(String::from);

    let album = vorbis
        .and_then(|v| v.album())
        .and_then(|v| v.iter().next())
        .map(String::from);

    let title = vorbis
        .and_then(|v| v.title())
        .and_then(|t| t.iter().next())
        .map(String::from);

    let track_number = vorbis.and_then(|v| v.track());
    let total_tracks = vorbis.and_then(|v| v.total_tracks());

    let duration = tag
        .get_streaminfo()
        .map(|v| (v.total_samples / v.sample_rate as u64) as u32);

    Ok(AudioFile {
        path: path.to_path_buf(),
        codec: AudioCodec::Flac,
        title,
        artist,
        album_artist,
        album,
        track_number,
        total_tracks,
        duration,
        // TODO
        disc_number: None,
        total_discs: None,
        genre: None,
    })
}

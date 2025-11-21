// SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#[derive(Debug, Clone, PartialEq)]
pub enum AudioCodec {
    Flac,
    Mp3,
    Mp4,
}

pub fn codec_name(codec: AudioCodec) -> String {
    match codec {
        AudioCodec::Flac => String::from("FLAC"),
        // TODO: add bitrate
        AudioCodec::Mp3 => String::from("MP3"),
        // I don't know who uses this format aside from Apple, hence M4A.
        AudioCodec::Mp4 => String::from("M4A"),
    }
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

// Define a struct to hold metadata information about a music track.
#[cw_serde] // Apply serialization capabilities to the struct.
#[derive(Default)] // Automatically implement the Default trait for `Metadata`.
pub struct Metadata {
    pub artist: String,          // Name of the artist.
    pub album: String,           // Album name where the track is featured.
    pub artwork_url: String,     // URL to the album artwork.
    pub year: i32,               // Year of release.
    pub track_name: String,      // Name of the track.
    pub audio_track_url: String, // URL to the audio file of the track.
}

// Define a type alias for an optional `Metadata` structure.
pub type Extension = Option<Metadata>;

#[cfg(test)]
mod tests {
    use crate::state::{Metadata};
    use cosmwasm_std::Uint128;
    use serde_json::json;

    #[test]
    fn test_metadata_default() {
        let metadata = Metadata::default();
        assert_eq!(metadata.artist, "");
        assert_eq!(metadata.album, "");
        assert_eq!(metadata.artwork_url, "");
        assert_eq!(metadata.year, 0);
        assert_eq!(metadata.track_name, "");
        assert_eq!(metadata.audio_track_url, "");
    }

    #[test]
    fn test_metadata_serialization() {
        let metadata = Metadata {
            artist: "Artist Name".to_string(),
            album: "Album Title".to_string(),
            artwork_url: "http://example.com/artwork.jpg".to_string(),
            year: 2022,
            track_name: "Track Name".to_string(),
            audio_track_url: "http://example.com/track.mp3".to_string(),
        };
        let serialized = serde_json::to_string(&metadata).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        let expected_json = json!({
            "artist": "Artist Name",
            "album": "Album Title",
            "artwork_url": "http://example.com/artwork.jpg",
            "year": 2022,
            "track_name": "Track Name",
            "audio_track_url": "http://example.com/track.mp3"
        });
        let expected_deserialized: serde_json::Value =
            serde_json::from_value(expected_json).unwrap();

        assert_eq!(deserialized, expected_deserialized);
    }
}

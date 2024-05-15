use cosmwasm_schema::cw_serde;

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

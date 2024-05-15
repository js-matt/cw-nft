#[cfg(test)]
mod tests {
    use crate::{contract::Cw721MetadataContract, state::Metadata};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;
    use cw721_base::{ExecuteMsg, InstantiateMsg};
    use serde_json::json;

    const CREATOR: &str = "creator";

    // Tests the metadata properties of a CW721 token
    #[test]
    fn test_cw721_nft_metadata() {
        // Setup
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();
        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "cw721-nft".to_string(),
            symbol: "cw_mock".to_string(),
            minter: CREATOR.to_string(),
        };
        let _ = contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "mock_token_id";
        let token_uri = Some("https://example.com/test".to_string());
        let extension = Some(Metadata {
            artist: "mock_artist".to_string(),
            album: "https://example.com/test-album".to_string(),
            ..Metadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "mock_owner".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };

        // Action
        let _ = contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        // Assert
        let res = contract
            .nft_info(deps.as_ref(), token_id.to_string())
            .unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, extension);
    }

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

use crate::state::Extension;
use cosmwasm_std::Empty;
use cw721_base::Cw721Contract;

pub type Cw721MetadataContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Metadata};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{to_json_binary, CosmosMsg, Response, Uint128, WasmMsg};
    use cw721::{Cw721Query, Cw721ReceiveMsg};
    use cw721_base::{ContractError, ExecuteMsg, InstantiateMsg};
    use cw_ownable::OwnershipError;

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
}

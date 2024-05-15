use crate::state::Extension;
use cosmwasm_std::Empty;
use cw721_base::Cw721Contract;

pub type Cw721MetadataContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

use crate::state::Extension;
use cosmwasm_std::Empty;
use cw721_base;

// Define a type alias for ExecuteMsg using the base ExecuteMsg type from cw721_base.
// This allows ExecuteMsg to be customized with the Extension type for additional data
// and use Empty where no additional data is needed.
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

// Define a type alias for QueryMsg using the base QueryMsg type from cw721_base.
// This uses Empty as the type for extensions, indicating no additional data is needed in queries.
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

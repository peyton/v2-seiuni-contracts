use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, JsonSchema)]
pub struct Fees {
    pub protocol_fee_recipient: Addr,
    pub protocol_fee_percent: Decimal,
    pub lp_fee_percent: Decimal,
}

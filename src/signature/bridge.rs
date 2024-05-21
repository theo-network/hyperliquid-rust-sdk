use ethers::contract::{Eip712, EthAbiType};

use serde::{Deserialize, Serialize};

pub(crate) mod mainnet {
    use super::*;

    #[derive(Debug, Eip712, Clone, EthAbiType, Serialize, Deserialize)]
    #[eip712(
        name = "Exchange",
        version = "1",
        chain_id = 42161,
        verifying_contract = "0x0000000000000000000000000000000000000000"
    )]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct WithdrawFromBridge2SignPayload {
        pub(crate) destination: String,
        pub(crate) usd: String,
        pub(crate) time: u64,
    }
}

pub(crate) mod testnet {
    use super::*;
    #[derive(Debug, Eip712, Clone, EthAbiType)]
    #[eip712(
        name = "Exchange",
        version = "1",
        chain_id = 421613,
        verifying_contract = "0x0000000000000000000000000000000000000000"
    )]
    pub(crate) struct WithdrawFromBridge2SignPayload {
        pub(crate) destination: String,
        pub(crate) usd: String,
        pub(crate) time: u64,
    }
}

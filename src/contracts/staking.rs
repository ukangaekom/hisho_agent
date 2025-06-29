use ethers::{prelude::abigen};


abigen!(HishoStaking,
    "../abis/hisho_staking.json",
    event_derives(serde::Deserialize, serde::Serialize)  
);

pub use HishoStaking::*;
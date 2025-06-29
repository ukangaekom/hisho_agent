use ethers::{prelude::abigen};


abigen!(HishoStaking,
    "./src/abis/hisho_staking.json",
    event_derives(serde::Deserialize, serde::Serialize)  
);

// pub HishoStaking;
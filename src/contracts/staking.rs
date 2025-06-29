
use ethers_contract::abigen;

abigen!(HishoStaking,
    "../abis/",
    event_derives(serde::Deserialize, serde::Serialize)  
);
use ethers_contract::abigen;

abigen!(HishoNFTs,
     "../abis/",
     event_derives(serde::Deserialize, serde::Serialize)
    );
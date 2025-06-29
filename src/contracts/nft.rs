use ethers::{prelude::abigen};

abigen!(
    HishoNFTs,
     "./src/abis/hisho_nft.json",
     event_derives(serde::Deserialize, serde::Serialize)
    );
    
// pub HishoNFTs;


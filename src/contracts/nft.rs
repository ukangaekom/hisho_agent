use ethers::{prelude::abigen};

abigen!(HishoNFTs,
     "../abis/hisho_nft.json",
     event_derives(serde::Deserialize, serde::Serialize)
    );
    
pub use HishoNFTs::*;


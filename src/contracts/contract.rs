
use super::token::IERC20;
use super::custom::*;
use super::nft::*;

use ethers::{
    core::{types::TransactionRequest},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils::{parse_ether},
    prelude::*
};
use ethers::types::Address;
use std::sync::Arc;

// ALL CONTRACT GETTER FUNCTIONS ARE FOUND HERE

// #[tokio::main]
// pub async fn get_token_symbol() -> Result<String, Box<dyn std::error::Error>>  {

//     Ok(())

// }

#[tokio::main]
async fn get_token_balance(
    caller: Arc<Provider<Http>>,
    token_address:&str,
    user_address:&str,
)-> String{
    let token: Address = token_address.parse().expect("Address");
    let user: Address = user_address.parse().expect("Address");
    let balance = IERC20::new(token, caller)
        .balance_of(user).await.expect("balance of ERC20");

    let result = format!("{:#?}",balance);
    println!("Result: {}",result);

    return result
}


// #[tokio::main]
// pub async fn get_token_name() -> Result<String, Box<dyn std::error::Error>>  {

//     Ok(())

// }


// #[tokio::main]
// pub async fn get_balance_symbol() -> Result<String, Box<dyn std::error::Error>>  {

//     Ok(())

// }



// GETTER FUNCTION ENDS



// ALL CONTRACT SETTER FUNCTIONS ARE FOUND HERE




// SETTER FUNCTION ENDS




// ALL CONTRACT TRANSFER FUNCTION ARE FOUND HERE



// ALL MINTING FUNCTIONS ARE FOUND HERE

// #[tokio::main]
// pub async fn mint_nft() -> Result<String, Box<dyn std::error::Error>>  {

//     Ok(())

// }

// pub async fn mint_token() -> Result<String, Box<dyn std::error::Error>>{
//     Ok(())

// }



// All MINTING FUNCTIONS ENDS

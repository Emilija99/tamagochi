use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::ContractInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PetInfo{
    pub full_hours:u64,
    pub alive_hours:u64,
    pub feeding_price:Uint128//feeding price in FOOD tokens
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub snip_info:ContractInfo,
    pub pet_info:PetInfo,
    pub market_addr:HumanAddr,
  
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    
    FeedPet {
        amount: Uint128,
        viewing_key: String,
        pet_name:String
    },
    CreateNewPet{
        pet_name:String,
        owner:HumanAddr
    },
    CreateViewingKey{
        entropy:String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Pet{name:String},
  Pets{page_num:u64,page_size:u64,viewing_key:String,address:HumanAddr}
   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LastFeedingResponse {
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ViewingKeyResponse{
    pub key:String
}



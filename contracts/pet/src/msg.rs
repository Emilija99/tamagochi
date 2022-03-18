use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub snip_addr: HumanAddr,
    pub snip_hash: String,
    pub market_addr:HumanAddr
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
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Pet{name:String},
  Pets{page_num:u64,page_size:u64}
   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LastFeedingResponse {
    pub timestamp: u64,
}



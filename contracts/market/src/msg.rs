use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
  
    pub snip_addr:HumanAddr,
    pub snip_hash:String,
    pub pet_addr:HumanAddr,
    pub pet_hash:String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
   BuyFood{},
   BuyPet{pet_name:String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
   Amount{}
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalAmountResponse{
    pub amount:Uint128
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PetMsg{
    CreateNewPet{pet_name:String,owner:HumanAddr},
}

impl PetMsg{
    pub fn create_new_pet(pet_name:&str,owner:HumanAddr)->Self{
        Self::CreateNewPet { pet_name:pet_name.to_string(), owner }
    }
}
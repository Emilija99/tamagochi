use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Api, CosmosMsg, Extern, HandleResponse, HumanAddr, Querier, StdError, StdResult,
    Storage, Uint128, WasmMsg, CanonicalAddr,
};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use secret_toolkit::snip20::mint_msg;

use crate::msg::PetMsg;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub addr: HumanAddr,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_amount: Uint128,
    pub pet_price: Uint128,
    pub food_contract: ContractInfo,
    pub owner:CanonicalAddr
}

impl State {
    pub fn get_snip_addr<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<HumanAddr, StdError> {
        Ok(config_read(&deps.storage).load()?.food_contract.addr)
    }

    pub fn get_snip_hash<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<String, StdError> {
        Ok(config_read(&deps.storage).load()?.food_contract.hash)
    }
    pub fn get_pet_price<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<Uint128, StdError> {
        Ok(config_read(&deps.storage).load()?.pet_price)
    }

    pub fn get_owner<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>)-> Result<CanonicalAddr, StdError>{
        Ok(config_read(&deps.storage).load()?.owner)
    }
    pub fn change_pet_price<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>,price:Uint128)->Result<(),StdError>{
        let mut conf=config(&mut deps.storage);
        let mut state=conf.load()?;
        state.pet_price=price;
        conf.save(&state)?;
        Ok(())


    }

    pub fn increase_total_amount<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        amount: Uint128,
    ) -> Result<(), StdError> {
        let mut conf = config(&mut deps.storage);
        let mut state = conf.load()?;
        state.total_amount += amount;
        conf.save(&state)?;
        Ok(())
    }
    pub fn mint_tokens(
        recipient: HumanAddr,
        amount: Uint128,
        contract_hash: String,
        contract_addr: HumanAddr,
    ) -> StdResult<HandleResponse> {
        let message = mint_msg(
            recipient,
            amount,
            None,
            None,
            0,
            contract_hash,
            contract_addr,
        )?;

        return Ok(HandleResponse {
            messages: vec![message],
            log: vec![],
            data: None,
        });
    }
    pub fn buy_pet(
        owner_addr: HumanAddr,
        pet_name: &str,
        pet_hash: String,
        pet_addr: HumanAddr,
    ) -> StdResult<HandleResponse> {
        let msg = to_binary(&PetMsg::create_new_pet(pet_name, owner_addr))?;
        let message = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: pet_addr,
            callback_code_hash: pet_hash,
            msg,
            send: vec![],
        });
        Ok(HandleResponse {
            messages: vec![message],
            log: vec![],
            data: None,
        })
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}
pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

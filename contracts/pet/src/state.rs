use cosmwasm_std::CanonicalAddr;
use cosmwasm_std::HandleResponse;
use cosmwasm_std::Uint128;
use cosmwasm_std::{Api, Extern, HumanAddr, Querier, StdError, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use secret_toolkit::snip20::balance_query;
use secret_toolkit::snip20::burn_from_msg;
use serde::{Deserialize, Serialize};
use crate::msg::PetInfo;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub food_contract: ContractInfo,
    pub pet_info: PetInfo,
    pub market_addr: HumanAddr,
    pub owner: CanonicalAddr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub addr: HumanAddr,
    pub hash: String,
}



impl State {
    pub fn get_owner<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<CanonicalAddr, StdError> {
        Ok(config_read(&deps.storage).load()?.owner)
    }
    pub fn get_market_addr<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<HumanAddr, StdError> {
        Ok(config_read(&deps.storage).load()?.market_addr)
    }
    pub fn get_pet_info<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<PetInfo, StdError> {
        Ok(config_read(&deps.storage).load()?.pet_info)
    }

    pub fn get_snip_info<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<ContractInfo, StdError> {
        Ok(config_read(&deps.storage).load()?.food_contract)
    }

    pub fn check_balance<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        view_key: String,
        address: HumanAddr,
    ) -> Result<Uint128, StdError> {
        let result = balance_query(
            &deps.querier,
            address,
            view_key,
            0,
            Self::get_snip_info(deps)?.hash,
            Self::get_snip_info(deps)?.addr,
        )?;

        Ok(result.amount)
    }
    pub fn burn_tokens<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        amount: Uint128,
        address: HumanAddr,
    ) -> StdResult<HandleResponse> {
        let message = burn_from_msg(
            address,
            amount,
            None,
            None,
            0,
            Self::get_snip_info(deps)?.hash,
            Self::get_snip_info(deps)?.addr,
        )?;
        Ok(HandleResponse {
            messages: vec![message],
            log: vec![],
            data: None,
        })
    }

    pub fn can_pet_be_fed<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        last_feeding_timestamp: u64,
        current_time: u64,
    ) -> Result<bool, StdError> {
        let time_difference = current_time - last_feeding_timestamp;
        if time_difference < hours_to_seconds(State::get_pet_info(deps)?.full_hours)
            || time_difference > hours_to_seconds(State::get_pet_info(deps)?.alive_hours)
        {
            Ok(false)
        } else {
            Ok(true)
        }
    }
    pub fn change_pet_info<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        pet_info: PetInfo,
    ) -> Result<(), StdError> {
        let mut conf = config(&mut deps.storage);
        let mut state = conf.load()?;
        state.pet_info = pet_info;
        conf.save(&state)?;
        Ok(())
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

fn hours_to_seconds(num_of_hours: u64) -> u64 {
    num_of_hours * 3600
}

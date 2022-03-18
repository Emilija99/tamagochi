use cosmwasm_std::HandleResponse;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Api, Extern, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use secret_toolkit::snip20::balance_query;
use secret_toolkit::snip20::burn_from_msg;
use secret_toolkit::snip20::register_receive_msg;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub snip_info: SnipInfo,
    pub pet: Pet,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SnipInfo {
    pub addr: HumanAddr,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pet {
    pub last_feeding: u64,
}

impl State {
    pub fn get_snip_addr<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<HumanAddr, StdError> {
        Ok(config_read(&deps.storage).load()?.snip_info.addr)
    }

    pub fn get_snip_hash<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<String, StdError> {
        Ok(config_read(&deps.storage).load()?.snip_info.hash)
    }

    pub fn get_last_feeding<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<u64, StdError> {
        Ok(config_read(&deps.storage).load()?.pet.last_feeding)
    }

    pub fn set_last_feeding<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        current_time: u64,
    ) -> Result<(), StdError> {
        let mut conf = config(&mut deps.storage);
        let mut state = conf.load()?;
        state.pet.last_feeding = current_time;
        conf.save(&state)?;
        Ok(())
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
            Self::get_snip_hash(deps)?,
            Self::get_snip_addr(deps)?,
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
            Self::get_snip_hash(deps)?,
            Self::get_snip_addr(deps)?,
        )?;
        Ok(HandleResponse {
            messages: vec![message],
            log: vec![],
            data: None,
        })
    }

    pub fn is_pet_alive<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        curent_time: u64,
    ) -> Result<bool, StdError> {
        let last_feeding = Self::get_last_feeding(deps)?;
        let time_passed = curent_time - last_feeding; //time passed since last feeding in seconds
        if time_passed < 14400 {
            //14400seconds=4hours
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

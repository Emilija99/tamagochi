use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    Api, Extern, HandleResponse, HumanAddr, Querier, StdError, StdResult, Storage,
    Uint128, Binary,
};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use secret_toolkit::snip20::mint_msg;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_amount: Uint128,
    pub snip_addr: HumanAddr,
    pub snip_hash: String,
}

impl State {
    pub fn get_snip_addr<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<HumanAddr, StdError> {
       Ok(config_read(&deps.storage).load()?.snip_addr)
    }

    pub fn get_snip_hash<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<String, StdError> {
        Ok(config_read(&deps.storage).load()?.snip_hash)
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
        data:Binary
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
            data:Some(data),
        });
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}
pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

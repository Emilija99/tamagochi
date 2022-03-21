use std::convert::TryInto;

use cosmwasm_std::from_slice;
use cosmwasm_std::to_vec;
use cosmwasm_std::CanonicalAddr;
use cosmwasm_std::HandleResponse;
use cosmwasm_std::Uint128;
use cosmwasm_std::{Api, Extern, HumanAddr, Querier, StdError, StdResult, Storage};
use cosmwasm_storage::PrefixedStorage;
use cosmwasm_storage::ReadonlyPrefixedStorage;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use secret_toolkit::serialization::Json;
use secret_toolkit::snip20::balance_query;
use secret_toolkit::snip20::burn_from_msg;
use secret_toolkit::storage::AppendStore;
use secret_toolkit::storage::AppendStoreMut;
use serde::{Deserialize, Serialize};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub food_contract: ContractInfo,

    pub market_addr: HumanAddr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub addr: HumanAddr,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub last_feeding: u64,
    pub owner: CanonicalAddr,
}

impl Pet {
    pub fn new(id: u64, name: String, last_feeding: u64, owner: CanonicalAddr) -> Pet {
        Pet {
            id,
            name,
            last_feeding,
            owner,
        }
    }

    pub fn next_id<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
    ) -> Result<u64, StdError> {
        let total = from_slice::<u64>(&deps.storage.get(b"total").unwrap())?;
        deps.storage.set(b"total", &to_vec(&(total + 1))?);
        Ok(total + 1)
    }

    pub fn add_new_pet<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        pet: &Pet,
    ) -> Result<(), StdError> {
        let mut store = PrefixedStorage::new(b"/pets/", &mut deps.storage);
        let mut a_store =
            AppendStoreMut::<Pet, _, _>::attach_or_create_with_serialization(&mut store, Json)?;
        a_store.push(pet)?;
        Ok(())
    }

    pub fn name_already_exists<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        pet_name: &str,
    ) -> Result<bool, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/pets/", &deps.storage);

        let a_store = AppendStore::<Pet, _, _>::attach_with_serialization(&store, Json);
        if let Some(store) = a_store {
            Ok(store?
                .iter()
                .any(|pet| pet.as_ref().unwrap().name.eq(pet_name)))
        } else {
            //kad se kreira prvi pet
            Ok(false)
        }
    }

    pub fn get_pet<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        pet_name: &str,
    ) -> Result<Pet, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/pets/", &deps.storage);

        let a_store = AppendStore::<Pet, _, _>::attach_with_serialization(&store, Json)
            .ok_or(StdError::generic_err("Pets not created"))??;

        a_store
            .iter()
            .find(|x| x.as_ref().unwrap().name.eq(pet_name))
            .ok_or(StdError::generic_err("Pet not found"))?
    }
    pub fn get_pets<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        page_num: usize,
        page_size: usize,
    ) -> Result<Vec<Pet>, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/pets/", &deps.storage);

        let a_store = AppendStore::<Pet, _, _>::attach_with_serialization(&store, Json)
            .ok_or(StdError::generic_err("Pets not created"))??;
        Ok(a_store
            .iter()
            .map(|x| x.unwrap())
            .skip(page_size * (page_num - 1))
            .take(page_size)
            .collect::<Vec<Pet>>())
    }

    pub fn update_pet<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        pet: &Pet,
    ) -> Result<(), StdError> {
        let mut store = PrefixedStorage::new(b"/pets/", &mut deps.storage);
        let mut a_store =
            AppendStoreMut::<Pet, _, _>::attach_or_create_with_serialization(&mut store, Json)?;
        let index = a_store
            .iter()
            .position(|r| r.unwrap().name.eq(&pet.name))
            .ok_or(StdError::generic_err("Pet not found"))?;

        a_store.set_at(index.try_into().unwrap(), &pet)?;
        Ok(())
    }
    pub fn can_pet_be_fed(last_feeding_timestamp: u64, current_time: u64) -> bool {
        let time_difference = current_time - last_feeding_timestamp;
        if time_difference > 14400 || time_difference < 10800 {
            //ako je proslo vise od 4h ljubimac je mrtav, ako je proslo manje od 3h ljubimac je sit
            false
        } else {
            true
        }
    }
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

    pub fn get_market_addr<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
    ) -> Result<HumanAddr, StdError> {
        Ok(config_read(&deps.storage).load()?.market_addr)
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
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

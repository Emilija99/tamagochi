use std::convert::TryInto;

use cosmwasm_std::{from_slice, to_vec, Api, CanonicalAddr, Extern, Querier, StdError, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use schemars::JsonSchema;
use secret_toolkit::{
    serialization::Json,
    storage::{AppendStore, AppendStoreMut},
};
use serde::{Deserialize, Serialize};

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

    pub fn create<S: Storage, A: Api, Q: Querier>(
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
        deps: &mut Extern<S, A, Q>,
        pet_name: &str,
    ) -> Result<bool, StdError> {
        let mut store = PrefixedStorage::new(b"/pets/", &mut deps.storage);

        let a_store =
            AppendStoreMut::<Pet, _, _>::attach_or_create_with_serialization(&mut store, Json)?;
        Ok(a_store
            .iter()
            .any(|pet| pet.as_ref().unwrap().name.eq(pet_name)))
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
        owner: CanonicalAddr,
    ) -> Result<Vec<Pet>, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/pets/", &deps.storage);

        let a_store = AppendStore::<Pet, _, _>::attach_with_serialization(&store, Json)
            .ok_or(StdError::generic_err("Pets not created"))??;
        Ok(a_store
            .iter()
            .map(|x| x.unwrap())
            .filter(|x| x.owner.eq(&owner))
            .skip(page_size * (page_num - 1))
            .take(page_size)
            .collect::<Vec<Pet>>())
    }

    pub fn update<S: Storage, A: Api, Q: Querier>(
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
}

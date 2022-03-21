use std::convert::TryInto;
//use secret_toolkit::crypto
use cosmwasm_std::{
    to_binary, to_vec, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, ViewingKeyResponse};
use crate::state::{config, ContractInfo, Pet, State};
use crate::view_key::{ViewingKey, ViewingKeyStore};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    deps.storage.set(b"total", &to_vec(&0)?);
    let state = State {
        food_contract: msg.snip_info,
        market_addr: msg.market_addr,
        pet_info: msg.pet_info,
    };

    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::FeedPet {
            amount,
            viewing_key,
            pet_name,
        } => try_feed(deps, env, amount, viewing_key, pet_name),
        HandleMsg::CreateNewPet { pet_name, owner } => try_create_pet(deps, env, pet_name, owner),
        HandleMsg::CreateViewingKey { entropy } => try_create_viewing_key(deps, env, entropy),
    }
}

pub fn try_create_viewing_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    entropy: String,
) -> StdResult<HandleResponse>{
    let viewing_key=ViewingKey::create(&mut deps.storage, &env, &env.message.sender, entropy.as_bytes());
    Ok(HandleResponse{
        messages: vec![],
        log: vec![],
        data:Some(to_binary(&ViewingKeyResponse{ key:viewing_key })?),
    })
}

pub fn try_create_pet<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pet_name: String,
    owner: HumanAddr,
) -> StdResult<HandleResponse> {
    if env.message.sender.ne(&State::get_market_addr(deps)?) {
        return Err(StdError::unauthorized());
    }
    if Pet::name_already_exists(deps, &pet_name)? {
        return Err(StdError::generic_err("Pet name already exists"));
    }
    let new_pet = Pet::new(
        Pet::next_id(deps)?,
        pet_name,
        env.block.time,
        deps.api.canonical_address(&owner)?,
    );
    Pet::add_new_pet(deps, &new_pet)?;
    Ok(HandleResponse::default())
}

pub fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: Uint128,
    viewing_key: String,
    pet_name: String,
) -> StdResult<HandleResponse> {
    let mut pet = Pet::get_pet(deps, &pet_name)?;
    if pet
        .owner
        .ne(&deps.api.canonical_address(&env.message.sender)?)
    {
        return Err(StdError::unauthorized());
    }
    if State::can_pet_be_fed(deps, pet.last_feeding, env.block.time)? == false {
        return Err(StdError::generic_err("Pet can't be fed now"));
    }
    if State::check_balance(deps, viewing_key, env.message.sender.clone())?
        < State::get_pet_info(deps)?.feeding_price
    {
        return Err(StdError::generic_err("Not enough tokens"));
    }

    pet.last_feeding = env.block.time;
    Pet::update_pet(deps, &pet)?;
    State::burn_tokens(deps, amount, env.message.sender)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pet { name } => to_binary(&query_pet(deps, &name)?),
        QueryMsg::Pets {
            page_num,
            page_size,
            viewing_key,
            address,
        } => to_binary(&query_pets(
            deps,
            page_num,
            page_size,
            viewing_key,
            address,
        )?),
    }
}
fn query_pet<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, name: &str) -> StdResult<Pet> {
    Ok(Pet::get_pet(deps, name)?)
}

fn query_pets<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    page_num: u64,
    page_size: u64,
    viewing_key: String,
    address: HumanAddr,
) -> StdResult<Vec<Pet>> {
    ViewingKey::check(&deps.storage, &address, &viewing_key)?;
    Ok(Pet::get_pets(
        deps,
        page_num.try_into().unwrap(),
        page_size.try_into().unwrap(),
        deps.api.canonical_address(&address)?,
    )?)
}
/*fn query_feeding<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<LastFeedingResponse> {
    Ok(LastFeedingResponse {
        timestamp: State::get_last_feeding(deps)?,
    })
}*/

/*#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // not anyone can reset
        let unauth_env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let res = handle(&mut deps, unauth_env, msg);
        match res {
            Err(StdError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_env = mock_env("creator", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let _res = handle(&mut deps, auth_env, msg).unwrap();

        // should now be 5
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
*/

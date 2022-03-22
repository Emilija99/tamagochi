use cosmwasm_std::{
    to_binary, Api, Binary, Coin, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, TotalAmountResponse};
use crate::state::{config, config_read, ContractInfo, State};


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let state = State {
        total_amount: cosmwasm_std::Uint128(0),
        food_contract: ContractInfo {
            addr: msg.snip_addr,
            hash: msg.snip_hash,
        },
        pet_price: msg.pet_price,
        owner: deps.api.canonical_address(&env.message.sender)?,
       
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
        HandleMsg::BuyFood {} => try_buy_food(deps, env),
        HandleMsg::BuyPet {
            pet_name,
            pet_addr,
            pet_hash,
        } => try_buy_pet(deps, env, pet_name, pet_addr, pet_hash),
        HandleMsg::ChangePetPrice { price } => try_change_pet_price(deps, env, price),
    }
}

pub fn try_buy_food<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let amount = calculate_amount(env.message.sent_funds);
    let food_amount = Uint128(amount.u128() / 10000);

    State::increase_total_amount(deps, amount)?;
    return State::mint_tokens(
        env.message.sender,
        food_amount,
        State::get_snip_hash(deps)?,
        State::get_snip_addr(deps)?,
    );
}

pub fn try_buy_pet<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pet_name: String,
    pet_addr: HumanAddr,
    pet_hash: String,
) -> StdResult<HandleResponse> {
    let amount = calculate_amount(env.message.sent_funds);
    if amount < State::get_pet_price(deps)? {
        return Err(StdError::generic_err("Not enough tokens"));
    }
    State::increase_total_amount(deps, amount)?;
    return State::buy_pet(env.message.sender, &pet_name, pet_hash, pet_addr);
}

pub fn try_change_pet_price<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>,
    env: Env,price:Uint128)-> StdResult<HandleResponse>{
       if deps.api.canonical_address(&env.message.sender)?.ne(&State::get_owner(deps)?){
            return Err(StdError::unauthorized())
       }
       State::change_pet_price(deps, price)?;
       Ok(HandleResponse::default())
}

pub fn calculate_amount(coins: Vec<Coin>) -> Uint128 {
    coins
        .iter()
        .filter(|coin| coin.denom.eq("uscrt"))
        .map(|coin| coin.amount)
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Amount {} => to_binary(&query_amount(deps)?),
    }
}

fn query_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<TotalAmountResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(TotalAmountResponse {
        amount: state.total_amount,
    })
}

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

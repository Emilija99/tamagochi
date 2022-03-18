use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Coin, Env, Extern, HandleResponse, InitResponse, Querier,
    StdResult, Storage, Uint128,
};

use crate::msg::{ HandleMsg, InitMsg, QueryMsg, TotalAmountResponse};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    
    let state = State {
        total_amount: cosmwasm_std::Uint128(0),
        snip_addr: msg.snip_addr,
        snip_hash: msg.snip_hash,
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
    }
}

pub fn try_buy_food<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let amount = calculate_amount(env.message.sent_funds);
    let food_amount = Uint128(amount.u128() / 10000);
    let data=format!("Contract code and hash: {} {},amount: {},food_amount:{},recipient:{}",State::get_snip_hash(deps)?,State::get_snip_addr(deps)?,&amount,&food_amount,&env.message.sender);
    State::increase_total_amount(deps, amount)?;
    return State::mint_tokens(
        env.message.sender,
        food_amount,
        State::get_snip_hash(deps)?,
        State::get_snip_addr(deps)?,
        to_binary(&data)?
    );
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

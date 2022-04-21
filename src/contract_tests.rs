#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{coins, Coin, MemoryStorage, OwnedDeps, Uint128};

    use crate::execute_messages::msg::{InstantiateMsg, ExecuteMsg};

    use crate::contract::{instantiate, execute};
    use crate::execute_messages::msg_admin::AdminExecuteMsg;
    use crate::structs::{Prize, PrizePool, LotteryStatus};

    const TEST_DENOM: &str = "uusd";
    const TEST_CREATOR: &str = "creator";
    const TEST_USER: &str = "user";
    const TEST_USER2: &str = "user2";

    const _TEST_PRICE: u64 = 10000000;

    const _TEST_INVALID_DENOM: &str = "notuusd";

    fn instantiate_contract() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies(&[Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(100u64),
        }]);

        let msg = InstantiateMsg {};
        let info = mock_info(TEST_CREATOR, &coins(1000, TEST_DENOM));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        return deps;
    }

    #[test]
    fn instantiate_success() {
        let _deps = instantiate_contract();
    }

    #[test]
    fn full_cycle() {
        let mut deps = instantiate_contract();

        // create new lottery 
        let msg = ExecuteMsg::Admin(AdminExecuteMsg::StartNewLottery {  });
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        
        // set prizes  
        let nft_address = "YEP_NFT".to_string();
        let prizes: Vec<Prize> = (0..10)
            .into_iter()
            .map(|elem| Prize{nft_address: nft_address.clone(), token_id: elem.to_string()})
            .collect();

        let prize_pool = PrizePool {prizes: prizes};

        let msg = ExecuteMsg::Admin(AdminExecuteMsg::SetPrizePool { target_id: 1, prize_pool: prize_pool });
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


        // pricing? later 

        // lock prices 
        // not implemented yet 

        // open registration  
        let msg = ExecuteMsg::Admin(AdminExecuteMsg::UpdateLotteryState { new_state: LotteryStatus::Registration });
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


        // register 
        let msg = ExecuteMsg::Register { id_lottery: 0 };
        let info = mock_info(TEST_USER, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::Register { id_lottery: 0 };
        let info = mock_info(TEST_USER2, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


        // close registrations 
        let msg = ExecuteMsg::Admin(AdminExecuteMsg::UpdateLotteryState { new_state: LotteryStatus::WaitingForDraw });
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


        // draw 
        // not implemented  


        // allow claims 
        let msg = ExecuteMsg::Admin(AdminExecuteMsg::UpdateLotteryState { new_state: LotteryStatus::PrizeDistribution });
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


        // claim prizes
        // need draw to work

    }
}

use casper_liquid::{CasperLiquid, Error};
use odra::prelude::*;
use odra::host::{Deployer, HostRef};

/// Integration tests for CasperLiquid contract
/// These tests simulate real-world usage scenarios and multi-user interactions
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test end-to-end stake/unstake flow for a single user
    #[test]
    fn test_end_to_end_single_user_flow() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        // Set caller to user
        test_env.set_caller(user);
        
        // Verify initial state
        assert_eq!(contract.balance_of(&user), U256::zero());
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.contract_cspr_balance(), U256::zero());
        
        // Step 1: User stakes 100 CSPR
        let stake_amount = U256::from(100);
        let stake_result = contract.stake(stake_amount);
        assert!(stake_result.is_ok(), "Stake operation should succeed");
        
        // Verify state after staking
        assert_eq!(contract.balance_of(&user), stake_amount);
        assert_eq!(contract.total_supply(), stake_amount);
        assert_eq!(contract.contract_cspr_balance(), stake_amount);
        assert!(contract.validate_supply_consistency());
        
        // Step 2: User stakes additional 50 CSPR
        let additional_stake = U256::from(50);
        let stake_result2 = contract.stake(additional_stake);
        assert!(stake_result2.is_ok(), "Second stake operation should succeed");
        
        let total_staked = stake_amount + additional_stake;
        assert_eq!(contract.balance_of(&user), total_staked);
        assert_eq!(contract.total_supply(), total_staked);
        assert_eq!(contract.contract_cspr_balance(), total_staked);
        
        // Step 3: User unstakes 75 CSPR
        let unstake_amount = U256::from(75);
        let unstake_result = contract.unstake(unstake_amount);
        assert!(unstake_result.is_ok(), "Unstake operation should succeed");
        
        let remaining_balance = total_staked - unstake_amount;
        assert_eq!(contract.balance_of(&user), remaining_balance);
        assert_eq!(contract.total_supply(), remaining_balance);
        assert_eq!(contract.contract_cspr_balance(), remaining_balance);
        
        // Step 4: User unstakes remaining balance
        let final_unstake_result = contract.unstake(remaining_balance);
        assert!(final_unstake_result.is_ok(), "Final unstake should succeed");
        
        // Verify final state (back to initial)
        assert_eq!(contract.balance_of(&user), U256::zero());
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.contract_cspr_balance(), U256::zero());
        assert!(contract.validate_supply_consistency());
    }

    /// Test multi-user scenario with concurrent operations
    #[test]
    fn test_multi_user_concurrent_operations() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user1 = test_env.get_account(0);
        let user2 = test_env.get_account(1);
        let user3 = test_env.get_account(2);
        
        // User 1 stakes 100 CSPR
        test_env.set_caller(user1);
        let stake1_result = contract.stake(U256::from(100));
        assert!(stake1_result.is_ok());
        
        // User 2 stakes 200 CSPR
        test_env.set_caller(user2);
        let stake2_result = contract.stake(U256::from(200));
        assert!(stake2_result.is_ok());
        
        // User 3 stakes 150 CSPR
        test_env.set_caller(user3);
        let stake3_result = contract.stake(U256::from(150));
        assert!(stake3_result.is_ok());
        
        // Verify individual balances
        assert_eq!(contract.balance_of(&user1), U256::from(100));
        assert_eq!(contract.balance_of(&user2), U256::from(200));
        assert_eq!(contract.balance_of(&user3), U256::from(150));
        
        // Verify total supply
        let expected_total = U256::from(450);
        assert_eq!(contract.total_supply(), expected_total);
        assert_eq!(contract.contract_cspr_balance(), expected_total);
        
        // User 1 transfers 30 stCSPR to User 3
        test_env.set_caller(user1);
        let transfer_result = contract.transfer(&user3, U256::from(30));
        assert!(transfer_result.is_ok());
        
        // Verify balances after transfer
        assert_eq!(contract.balance_of(&user1), U256::from(70));
        assert_eq!(contract.balance_of(&user2), U256::from(200));
        assert_eq!(contract.balance_of(&user3), U256::from(180));
        
        // Total supply should remain unchanged after transfer
        assert_eq!(contract.total_supply(), expected_total);
        assert_eq!(contract.contract_cspr_balance(), expected_total);
        
        // User 2 unstakes 100 CSPR
        test_env.set_caller(user2);
        let unstake_result = contract.unstake(U256::from(100));
        assert!(unstake_result.is_ok());
        
        // Verify balances after unstaking
        assert_eq!(contract.balance_of(&user1), U256::from(70));
        assert_eq!(contract.balance_of(&user2), U256::from(100));
        assert_eq!(contract.balance_of(&user3), U256::from(180));
        
        let new_total = U256::from(350);
        assert_eq!(contract.total_supply(), new_total);
        assert_eq!(contract.contract_cspr_balance(), new_total);
        
        // Verify supply consistency throughout
        assert!(contract.validate_supply_consistency());
    }

    /// Test approval and transfer_from functionality in multi-user scenario
    #[test]
    fn test_multi_user_approval_flow() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let owner = test_env.get_account(0);
        let spender = test_env.get_account(1);
        let recipient = test_env.get_account(2);
        
        // Owner stakes 200 CSPR
        test_env.set_caller(owner);
        contract.stake(U256::from(200)).unwrap();
        
        // Owner approves spender for 100 stCSPR
        let approval_result = contract.approve(&spender, U256::from(100));
        assert!(approval_result.is_ok());
        assert_eq!(contract.allowance(&owner, &spender), U256::from(100));
        
        // Spender transfers 60 stCSPR from owner to recipient
        test_env.set_caller(spender);
        let transfer_from_result = contract.transfer_from(&owner, &recipient, U256::from(60));
        assert!(transfer_from_result.is_ok());
        
        // Verify balances
        assert_eq!(contract.balance_of(&owner), U256::from(140));
        assert_eq!(contract.balance_of(&spender), U256::zero());
        assert_eq!(contract.balance_of(&recipient), U256::from(60));
        
        // Verify remaining allowance
        assert_eq!(contract.allowance(&owner, &spender), U256::from(40));
        
        // Spender tries to transfer more than remaining allowance
        let excessive_transfer = contract.transfer_from(&owner, &recipient, U256::from(50));
        assert!(excessive_transfer.is_err());
        match excessive_transfer.unwrap_err() {
            Error::InsufficientAllowance => {},
            _ => panic!("Expected InsufficientAllowance error"),
        }
        
        // Balances should remain unchanged after failed transfer
        assert_eq!(contract.balance_of(&owner), U256::from(140));
        assert_eq!(contract.balance_of(&recipient), U256::from(60));
        assert_eq!(contract.allowance(&owner, &spender), U256::from(40));
        
        // Total supply should remain consistent
        assert_eq!(contract.total_supply(), U256::from(200));
        assert!(contract.validate_supply_consistency());
    }

    /// Test error handling in multi-user scenarios
    #[test]
    fn test_multi_user_error_scenarios() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user1 = test_env.get_account(0);
        let user2 = test_env.get_account(1);
        
        // User 1 stakes 100 CSPR
        test_env.set_caller(user1);
        contract.stake(U256::from(100)).unwrap();
        
        // User 2 tries to unstake without having any balance
        test_env.set_caller(user2);
        let unstake_result = contract.unstake(U256::from(50));
        assert!(unstake_result.is_err());
        match unstake_result.unwrap_err() {
            Error::InsufficientBalance => {},
            _ => panic!("Expected InsufficientBalance error"),
        }
        
        // User 2 tries to transfer without having any balance
        let transfer_result = contract.transfer(&user1, U256::from(25));
        assert!(transfer_result.is_err());
        match transfer_result.unwrap_err() {
            Error::InsufficientBalance => {},
            _ => panic!("Expected InsufficientBalance error"),
        }
        
        // User 1 tries to transfer to themselves
        test_env.set_caller(user1);
        let self_transfer = contract.transfer(&user1, U256::from(10));
        assert!(self_transfer.is_err());
        match self_transfer.unwrap_err() {
            Error::SelfTransfer => {},
            _ => panic!("Expected SelfTransfer error"),
        }
        
        // User 1 tries to approve themselves
        let self_approve = contract.approve(&user1, U256::from(50));
        assert!(self_approve.is_err());
        match self_approve.unwrap_err() {
            Error::SelfTransfer => {},
            _ => panic!("Expected SelfTransfer error"),
        }
        
        // All operations should fail without changing state
        assert_eq!(contract.balance_of(&user1), U256::from(100));
        assert_eq!(contract.balance_of(&user2), U256::zero());
        assert_eq!(contract.total_supply(), U256::from(100));
        assert!(contract.validate_supply_consistency());
    }

    /// Test complex multi-user workflow with mixed operations
    #[test]
    fn test_complex_multi_user_workflow() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let alice = test_env.get_account(0);
        let bob = test_env.get_account(1);
        let charlie = test_env.get_account(2);
        let dave = test_env.get_account(3);
        
        // Phase 1: Initial staking
        test_env.set_caller(alice);
        contract.stake(U256::from(500)).unwrap();
        
        test_env.set_caller(bob);
        contract.stake(U256::from(300)).unwrap();
        
        test_env.set_caller(charlie);
        contract.stake(U256::from(200)).unwrap();
        
        // Verify initial state
        assert_eq!(contract.total_supply(), U256::from(1000));
        assert_eq!(contract.balance_of(&alice), U256::from(500));
        assert_eq!(contract.balance_of(&bob), U256::from(300));
        assert_eq!(contract.balance_of(&charlie), U256::from(200));
        assert_eq!(contract.balance_of(&dave), U256::zero());
        
        // Phase 2: Transfers and approvals
        test_env.set_caller(alice);
        contract.transfer(&dave, U256::from(100)).unwrap();
        contract.approve(&bob, U256::from(150)).unwrap();
        
        test_env.set_caller(bob);
        contract.transfer_from(&alice, &charlie, U256::from(100)).unwrap();
        
        // Verify state after transfers
        assert_eq!(contract.balance_of(&alice), U256::from(300)); // 500 - 100 - 100
        assert_eq!(contract.balance_of(&bob), U256::from(300));
        assert_eq!(contract.balance_of(&charlie), U256::from(300)); // 200 + 100
        assert_eq!(contract.balance_of(&dave), U256::from(100));
        assert_eq!(contract.total_supply(), U256::from(1000)); // Unchanged
        
        // Phase 3: Mixed operations
        test_env.set_caller(charlie);
        contract.unstake(U256::from(150)).unwrap(); // Charlie unstakes some
        
        test_env.set_caller(dave);
        contract.stake(U256::from(50)).unwrap(); // Dave stakes more
        
        // Verify final state
        assert_eq!(contract.balance_of(&alice), U256::from(300));
        assert_eq!(contract.balance_of(&bob), U256::from(300));
        assert_eq!(contract.balance_of(&charlie), U256::from(150)); // 300 - 150
        assert_eq!(contract.balance_of(&dave), U256::from(150)); // 100 + 50
        
        let expected_total = U256::from(900); // 1000 - 150 + 50
        assert_eq!(contract.total_supply(), expected_total);
        assert_eq!(contract.contract_cspr_balance(), expected_total);
        
        // Verify supply consistency
        let sum_of_balances = contract.balance_of(&alice) + 
                             contract.balance_of(&bob) + 
                             contract.balance_of(&charlie) + 
                             contract.balance_of(&dave);
        assert_eq!(sum_of_balances, contract.total_supply());
        assert!(contract.validate_supply_consistency());
    }

    /// Test contract metadata consistency across operations
    #[test]
    fn test_contract_metadata_consistency() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user1 = test_env.get_account(0);
        let user2 = test_env.get_account(1);
        
        // Verify initial metadata
        assert_eq!(contract.name(), "Staked CSPR");
        assert_eq!(contract.symbol(), "stCSPR");
        assert_eq!(contract.decimals(), 9u8);
        
        // Perform various operations
        test_env.set_caller(user1);
        contract.stake(U256::from(100)).unwrap();
        
        test_env.set_caller(user2);
        contract.stake(U256::from(200)).unwrap();
        
        test_env.set_caller(user1);
        contract.transfer(&user2, U256::from(50)).unwrap();
        
        test_env.set_caller(user2);
        contract.unstake(U256::from(100)).unwrap();
        
        // Verify metadata remains unchanged
        assert_eq!(contract.name(), "Staked CSPR");
        assert_eq!(contract.symbol(), "stCSPR");
        assert_eq!(contract.decimals(), 9u8);
        
        // Verify metadata consistency across multiple calls
        for _ in 0..10 {
            assert_eq!(contract.name(), "Staked CSPR");
            assert_eq!(contract.symbol(), "stCSPR");
            assert_eq!(contract.decimals(), 9u8);
        }
    }

    /// Test large-scale operations with many users
    #[test]
    fn test_large_scale_multi_user_operations() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        
        let num_users = 10;
        let stake_amount = U256::from(100);
        
        // Phase 1: All users stake
        for i in 0..num_users {
            let user = test_env.get_account(i);
            test_env.set_caller(user);
            let result = contract.stake(stake_amount);
            assert!(result.is_ok(), "User {} stake should succeed", i);
            assert_eq!(contract.balance_of(&user), stake_amount);
        }
        
        let expected_total = stake_amount * U256::from(num_users);
        assert_eq!(contract.total_supply(), expected_total);
        assert_eq!(contract.contract_cspr_balance(), expected_total);
        
        // Phase 2: Half the users transfer to the other half
        for i in 0..(num_users / 2) {
            let sender = test_env.get_account(i);
            let recipient = test_env.get_account(i + num_users / 2);
            
            test_env.set_caller(sender);
            let transfer_amount = U256::from(25);
            let result = contract.transfer(&recipient, transfer_amount);
            assert!(result.is_ok(), "Transfer from user {} should succeed", i);
        }
        
        // Verify balances after transfers
        for i in 0..(num_users / 2) {
            let sender = test_env.get_account(i);
            let recipient = test_env.get_account(i + num_users / 2);
            
            assert_eq!(contract.balance_of(&sender), U256::from(75)); // 100 - 25
            assert_eq!(contract.balance_of(&recipient), U256::from(125)); // 100 + 25
        }
        
        // Total supply should remain unchanged
        assert_eq!(contract.total_supply(), expected_total);
        assert_eq!(contract.contract_cspr_balance(), expected_total);
        
        // Phase 3: Some users unstake
        for i in 0..(num_users / 4) {
            let user = test_env.get_account(i);
            test_env.set_caller(user);
            let unstake_amount = U256::from(50);
            let result = contract.unstake(unstake_amount);
            assert!(result.is_ok(), "User {} unstake should succeed", i);
        }
        
        // Verify final state
        let unstaked_total = U256::from(50) * U256::from(num_users / 4);
        let final_total = expected_total - unstaked_total;
        assert_eq!(contract.total_supply(), final_total);
        assert_eq!(contract.contract_cspr_balance(), final_total);
        
        // Verify supply consistency
        assert!(contract.validate_supply_consistency());
        
        // Calculate and verify sum of all balances
        let mut sum_of_balances = U256::zero();
        for i in 0..num_users {
            let user = test_env.get_account(i);
            sum_of_balances += contract.balance_of(&user);
        }
        assert_eq!(sum_of_balances, contract.total_supply());
    }

    /// Test edge cases in multi-user environment
    #[test]
    fn test_multi_user_edge_cases() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user1 = test_env.get_account(0);
        let user2 = test_env.get_account(1);
        
        // Test zero amount operations
        test_env.set_caller(user1);
        
        // Zero stake should fail
        let zero_stake = contract.stake(U256::zero());
        assert!(zero_stake.is_err());
        
        // Stake some amount first
        contract.stake(U256::from(100)).unwrap();
        
        // Zero unstake should fail
        let zero_unstake = contract.unstake(U256::zero());
        assert!(zero_unstake.is_err());
        
        // Zero transfer should fail
        let zero_transfer = contract.transfer(&user2, U256::zero());
        assert!(zero_transfer.is_err());
        
        // Test boundary conditions
        let user_balance = contract.balance_of(&user1);
        
        // Unstake exact balance should succeed
        let exact_unstake = contract.unstake(user_balance);
        assert!(exact_unstake.is_ok());
        assert_eq!(contract.balance_of(&user1), U256::zero());
        
        // Unstake when balance is zero should fail
        let unstake_zero_balance = contract.unstake(U256::from(1));
        assert!(unstake_zero_balance.is_err());
        
        // Transfer when balance is zero should fail
        let transfer_zero_balance = contract.transfer(&user2, U256::from(1));
        assert!(transfer_zero_balance.is_err());
        
        // Verify contract state remains consistent
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.contract_cspr_balance(), U256::zero());
        assert!(contract.validate_supply_consistency());
    }
}
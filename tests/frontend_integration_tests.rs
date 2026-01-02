use casper_liquid::{CasperLiquid, Error};
use odra::prelude::*;
use odra::host::{Deployer, HostRef};

/// Frontend integration tests for CasperLiquid contract
/// These tests simulate the interactions that would occur through the web frontend
#[cfg(test)]
mod frontend_integration_tests {
    use super::*;

    /// Simulate the complete user journey from the frontend perspective
    #[test]
    fn test_frontend_user_journey() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        test_env.set_caller(user);
        
        // Step 1: User connects wallet and checks initial state
        // This simulates what happens when the frontend loads
        assert_eq!(contract.name(), "Staked CSPR");
        assert_eq!(contract.symbol(), "stCSPR");
        assert_eq!(contract.decimals(), 9u8);
        assert_eq!(contract.balance_of(&user), U256::zero());
        assert_eq!(contract.total_supply(), U256::zero());
        
        // Step 2: User clicks "Stake 10 CSPR" button
        // Frontend converts 10 CSPR to motes (10 * 10^9)
        let stake_amount_motes = U256::from(10_000_000_000u64); // 10 CSPR in motes
        let stake_result = contract.stake(stake_amount_motes);
        assert!(stake_result.is_ok(), "Frontend stake operation should succeed");
        
        // Step 3: Frontend updates balance display
        assert_eq!(contract.balance_of(&user), stake_amount_motes);
        assert_eq!(contract.total_supply(), stake_amount_motes);
        
        // Step 4: User stakes another 10 CSPR (simulating multiple stakes)
        let second_stake = contract.stake(stake_amount_motes);
        assert!(second_stake.is_ok(), "Second frontend stake should succeed");
        
        let total_staked = stake_amount_motes * U256::from(2);
        assert_eq!(contract.balance_of(&user), total_staked);
        assert_eq!(contract.total_supply(), total_staked);
        
        // Step 5: User clicks "Unstake All" button
        // Frontend gets current balance and unstakes all
        let current_balance = contract.balance_of(&user);
        let unstake_result = contract.unstake(current_balance);
        assert!(unstake_result.is_ok(), "Frontend unstake all should succeed");
        
        // Step 6: Frontend verifies final state
        assert_eq!(contract.balance_of(&user), U256::zero());
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.contract_cspr_balance(), U256::zero());
    }

    /// Test frontend error handling scenarios
    #[test]
    fn test_frontend_error_handling() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        test_env.set_caller(user);
        
        // Test 1: User tries to stake 0 CSPR (frontend validation should catch this)
        let zero_stake = contract.stake(U256::zero());
        assert!(zero_stake.is_err(), "Zero stake should fail");
        match zero_stake.unwrap_err() {
            Error::InvalidAmount => {},
            _ => panic!("Expected InvalidAmount error"),
        }
        
        // Test 2: User tries to unstake without having any tokens
        let unstake_without_balance = contract.unstake(U256::from(1_000_000_000u64));
        assert!(unstake_without_balance.is_err(), "Unstake without balance should fail");
        match unstake_without_balance.unwrap_err() {
            Error::InsufficientBalance => {},
            _ => panic!("Expected InsufficientBalance error"),
        }
        
        // Test 3: User stakes some tokens first
        let stake_amount = U256::from(5_000_000_000u64); // 5 CSPR
        contract.stake(stake_amount).unwrap();
        
        // Test 4: User tries to unstake more than they have
        let excessive_unstake = contract.unstake(U256::from(10_000_000_000u64)); // 10 CSPR
        assert!(excessive_unstake.is_err(), "Excessive unstake should fail");
        match excessive_unstake.unwrap_err() {
            Error::InsufficientBalance => {},
            _ => panic!("Expected InsufficientBalance error"),
        }
        
        // Verify state remains unchanged after failed operations
        assert_eq!(contract.balance_of(&user), stake_amount);
        assert_eq!(contract.total_supply(), stake_amount);
    }

    /// Test frontend balance display accuracy
    #[test]
    fn test_frontend_balance_display() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        test_env.set_caller(user);
        
        // Test various stake amounts that frontend might encounter
        let test_amounts = vec![
            1_000_000_000u64,      // 1 CSPR
            10_000_000_000u64,     // 10 CSPR (standard stake amount)
            100_000_000_000u64,    // 100 CSPR
            1_500_000_000u64,      // 1.5 CSPR (fractional)
            999_999_999u64,        // Just under 1 CSPR
        ];
        
        let mut total_staked = U256::zero();
        
        for amount in test_amounts {
            let stake_amount = U256::from(amount);
            let stake_result = contract.stake(stake_amount);
            assert!(stake_result.is_ok(), "Stake of {} motes should succeed", amount);
            
            total_staked += stake_amount;
            
            // Verify balance matches expected
            assert_eq!(contract.balance_of(&user), total_staked);
            assert_eq!(contract.total_supply(), total_staked);
            
            // Simulate frontend conversion back to CSPR for display
            let cspr_balance = total_staked.as_u64() as f64 / 1_000_000_000.0;
            assert!(cspr_balance > 0.0, "CSPR balance should be positive");
        }
        
        // Test partial unstaking (frontend "unstake specific amount" feature)
        let partial_unstake = U256::from(5_000_000_000u64); // 5 CSPR
        let unstake_result = contract.unstake(partial_unstake);
        assert!(unstake_result.is_ok(), "Partial unstake should succeed");
        
        let remaining_balance = total_staked - partial_unstake;
        assert_eq!(contract.balance_of(&user), remaining_balance);
        assert_eq!(contract.total_supply(), remaining_balance);
    }

    /// Test frontend transaction confirmation flow
    #[test]
    fn test_frontend_transaction_flow() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        test_env.set_caller(user);
        
        // Simulate frontend transaction flow:
        // 1. User initiates stake
        // 2. Frontend shows "preparing transaction"
        // 3. Transaction is submitted
        // 4. Frontend shows "transaction pending"
        // 5. Transaction completes
        // 6. Frontend updates UI
        
        let stake_amount = U256::from(10_000_000_000u64); // 10 CSPR
        
        // Record state before transaction
        let initial_balance = contract.balance_of(&user);
        let initial_supply = contract.total_supply();
        
        // Execute transaction (simulating successful blockchain submission)
        let transaction_result = contract.stake(stake_amount);
        assert!(transaction_result.is_ok(), "Transaction should succeed");
        
        // Verify state after transaction (what frontend would check)
        let final_balance = contract.balance_of(&user);
        let final_supply = contract.total_supply();
        
        assert_eq!(final_balance, initial_balance + stake_amount);
        assert_eq!(final_supply, initial_supply + stake_amount);
        
        // Simulate frontend checking transaction success
        assert!(contract.validate_supply_consistency());
        
        // Test unstake transaction flow
        let unstake_amount = U256::from(3_000_000_000u64); // 3 CSPR
        
        let pre_unstake_balance = contract.balance_of(&user);
        let pre_unstake_supply = contract.total_supply();
        
        let unstake_result = contract.unstake(unstake_amount);
        assert!(unstake_result.is_ok(), "Unstake transaction should succeed");
        
        let post_unstake_balance = contract.balance_of(&user);
        let post_unstake_supply = contract.total_supply();
        
        assert_eq!(post_unstake_balance, pre_unstake_balance - unstake_amount);
        assert_eq!(post_unstake_supply, pre_unstake_supply - unstake_amount);
    }

    /// Test frontend multi-user interaction scenarios
    #[test]
    fn test_frontend_multi_user_scenarios() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let alice = test_env.get_account(0);
        let bob = test_env.get_account(1);
        
        // Simulate Alice using the frontend
        test_env.set_caller(alice);
        let alice_stake = U256::from(15_000_000_000u64); // 15 CSPR
        contract.stake(alice_stake).unwrap();
        
        // Simulate Bob using the frontend simultaneously
        test_env.set_caller(bob);
        let bob_stake = U256::from(25_000_000_000u64); // 25 CSPR
        contract.stake(bob_stake).unwrap();
        
        // Verify both users see correct balances
        assert_eq!(contract.balance_of(&alice), alice_stake);
        assert_eq!(contract.balance_of(&bob), bob_stake);
        assert_eq!(contract.total_supply(), alice_stake + bob_stake);
        
        // Alice transfers some tokens to Bob (using frontend transfer feature)
        test_env.set_caller(alice);
        let transfer_amount = U256::from(5_000_000_000u64); // 5 CSPR
        let transfer_result = contract.transfer(&bob, transfer_amount);
        assert!(transfer_result.is_ok(), "Frontend transfer should succeed");
        
        // Verify balances after transfer
        assert_eq!(contract.balance_of(&alice), alice_stake - transfer_amount);
        assert_eq!(contract.balance_of(&bob), bob_stake + transfer_amount);
        assert_eq!(contract.total_supply(), alice_stake + bob_stake); // Unchanged
        
        // Bob unstakes some tokens
        test_env.set_caller(bob);
        let bob_unstake = U256::from(10_000_000_000u64); // 10 CSPR
        let unstake_result = contract.unstake(bob_unstake);
        assert!(unstake_result.is_ok(), "Bob's unstake should succeed");
        
        // Verify final state
        let expected_alice_balance = alice_stake - transfer_amount;
        let expected_bob_balance = bob_stake + transfer_amount - bob_unstake;
        let expected_total = expected_alice_balance + expected_bob_balance;
        
        assert_eq!(contract.balance_of(&alice), expected_alice_balance);
        assert_eq!(contract.balance_of(&bob), expected_bob_balance);
        assert_eq!(contract.total_supply(), expected_total);
        assert_eq!(contract.contract_cspr_balance(), expected_total);
    }

    /// Test frontend approval workflow (for future DEX integration)
    #[test]
    fn test_frontend_approval_workflow() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        let dex_contract = test_env.get_account(1); // Simulating a DEX contract
        
        test_env.set_caller(user);
        
        // User stakes tokens first
        let stake_amount = U256::from(20_000_000_000u64); // 20 CSPR
        contract.stake(stake_amount).unwrap();
        
        // User approves DEX to spend their stCSPR tokens
        let approval_amount = U256::from(10_000_000_000u64); // 10 CSPR worth
        let approve_result = contract.approve(&dex_contract, approval_amount);
        assert!(approve_result.is_ok(), "Approval should succeed");
        
        // Verify allowance
        assert_eq!(contract.allowance(&user, &dex_contract), approval_amount);
        
        // Simulate DEX using the allowance
        test_env.set_caller(dex_contract);
        let spend_amount = U256::from(7_000_000_000u64); // 7 CSPR worth
        let transfer_from_result = contract.transfer_from(&user, &dex_contract, spend_amount);
        assert!(transfer_from_result.is_ok(), "DEX transfer_from should succeed");
        
        // Verify balances and remaining allowance
        assert_eq!(contract.balance_of(&user), stake_amount - spend_amount);
        assert_eq!(contract.balance_of(&dex_contract), spend_amount);
        assert_eq!(contract.allowance(&user, &dex_contract), approval_amount - spend_amount);
        
        // User revokes remaining allowance (sets to zero)
        test_env.set_caller(user);
        let revoke_result = contract.approve(&dex_contract, U256::zero());
        assert!(revoke_result.is_ok(), "Allowance revocation should succeed");
        assert_eq!(contract.allowance(&user, &dex_contract), U256::zero());
    }

    /// Test frontend edge cases and boundary conditions
    #[test]
    fn test_frontend_edge_cases() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        test_env.set_caller(user);
        
        // Test minimum stake amount (1 mote)
        let min_stake = U256::from(1u64);
        let min_stake_result = contract.stake(min_stake);
        assert!(min_stake_result.is_ok(), "Minimum stake should succeed");
        assert_eq!(contract.balance_of(&user), min_stake);
        
        // Test unstaking exact balance
        let exact_unstake = contract.unstake(min_stake);
        assert!(exact_unstake_result.is_ok(), "Exact balance unstake should succeed");
        assert_eq!(contract.balance_of(&user), U256::zero());
        
        // Test large stake amount (simulating whale user)
        let large_stake = U256::from(1_000_000_000_000_000_000u64); // 1 billion CSPR
        let large_stake_result = contract.stake(large_stake);
        assert!(large_stake_result.is_ok(), "Large stake should succeed");
        assert_eq!(contract.balance_of(&user), large_stake);
        
        // Test partial unstake of large amount
        let partial_unstake = U256::from(500_000_000_000_000_000u64); // 500 million CSPR
        let partial_result = contract.unstake(partial_unstake);
        assert!(partial_result.is_ok(), "Partial unstake should succeed");
        
        let remaining = large_stake - partial_unstake;
        assert_eq!(contract.balance_of(&user), remaining);
        assert_eq!(contract.total_supply(), remaining);
        
        // Test multiple small operations (simulating frequent user interactions)
        for i in 1..=10 {
            let small_stake = U256::from(i * 1_000_000_000u64); // i CSPR
            let result = contract.stake(small_stake);
            assert!(result.is_ok(), "Small stake {} should succeed", i);
        }
        
        // Verify final state consistency
        assert!(contract.validate_supply_consistency());
        assert_eq!(contract.total_supply(), contract.contract_cspr_balance());
    }

    /// Test frontend metadata queries (for UI display)
    #[test]
    fn test_frontend_metadata_queries() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        // Test metadata queries that frontend would make
        assert_eq!(contract.name(), "Staked CSPR");
        assert_eq!(contract.symbol(), "stCSPR");
        assert_eq!(contract.decimals(), 9u8);
        
        // Test balance queries
        assert_eq!(contract.balance_of(&user), U256::zero());
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.contract_cspr_balance(), U256::zero());
        
        // Perform some operations
        test_env.set_caller(user);
        contract.stake(U256::from(10_000_000_000u64)).unwrap();
        
        // Test queries after operations
        assert_eq!(contract.balance_of(&user), U256::from(10_000_000_000u64));
        assert_eq!(contract.total_supply(), U256::from(10_000_000_000u64));
        assert_eq!(contract.contract_cspr_balance(), U256::from(10_000_000_000u64));
        
        // Metadata should remain unchanged
        assert_eq!(contract.name(), "Staked CSPR");
        assert_eq!(contract.symbol(), "stCSPR");
        assert_eq!(contract.decimals(), 9u8);
        
        // Test supply consistency check (frontend health check)
        assert!(contract.validate_supply_consistency());
    }

    /// Test frontend reconnection scenarios
    #[test]
    fn test_frontend_reconnection_scenarios() {
        let test_env = odra_test::env();
        let mut contract = CasperLiquid::deploy(&test_env, NoArgs);
        let user = test_env.get_account(0);
        
        test_env.set_caller(user);
        
        // Simulate user session 1: stake some tokens
        let initial_stake = U256::from(15_000_000_000u64); // 15 CSPR
        contract.stake(initial_stake).unwrap();
        
        // Simulate user disconnecting and reconnecting
        // Frontend would query current balance to restore state
        let reconnect_balance = contract.balance_of(&user);
        assert_eq!(reconnect_balance, initial_stake);
        
        // User continues with more operations after reconnection
        let additional_stake = U256::from(5_000_000_000u64); // 5 CSPR
        contract.stake(additional_stake).unwrap();
        
        let total_balance = initial_stake + additional_stake;
        assert_eq!(contract.balance_of(&user), total_balance);
        assert_eq!(contract.total_supply(), total_balance);
        
        // Simulate another disconnection/reconnection
        let final_balance = contract.balance_of(&user);
        assert_eq!(final_balance, total_balance);
        
        // User unstakes after reconnection
        let unstake_amount = U256::from(8_000_000_000u64); // 8 CSPR
        contract.unstake(unstake_amount).unwrap();
        
        let remaining_balance = total_balance - unstake_amount;
        assert_eq!(contract.balance_of(&user), remaining_balance);
        assert_eq!(contract.total_supply(), remaining_balance);
    }
}
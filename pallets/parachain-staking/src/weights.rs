// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-19, STEPS: `[32, ]`, REPEAT: 64, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/moonbeam
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// parachain_staking
// --extrinsic
// *
// --steps
// 32
// --repeat
// 64
// --raw
// --template=./benchmarking/frame-weight-template.hbs
// --output
// /tmp/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
	fn hotfix_remove_delegation_requests(x: u32) -> Weight;
	fn hotfix_update_candidate_pool_value(x: u32) -> Weight;
	fn set_staking_expectations() -> Weight;
	fn set_inflation() -> Weight;
	fn set_parachain_bond_account() -> Weight;
	fn set_parachain_bond_reserve_percent() -> Weight;
	fn set_total_selected() -> Weight;
	fn set_collator_commission() -> Weight;
	fn set_blocks_per_round() -> Weight;
	fn join_candidates(x: u32) -> Weight;
	fn schedule_leave_candidates(x: u32) -> Weight;
	fn execute_leave_candidates(x: u32) -> Weight;
	fn cancel_leave_candidates(x: u32) -> Weight;
	fn go_offline() -> Weight;
	fn go_online() -> Weight;
	fn candidate_bond_more() -> Weight;
	fn schedule_candidate_bond_less() -> Weight;
	fn execute_candidate_bond_less() -> Weight;
	fn cancel_candidate_bond_less() -> Weight;
	fn delegate(x: u32, y: u32) -> Weight;
	fn schedule_leave_delegators() -> Weight;
	fn execute_leave_delegators(x: u32) -> Weight;
	fn cancel_leave_delegators() -> Weight;
	fn schedule_revoke_delegation() -> Weight;
	fn delegator_bond_more() -> Weight;
	fn schedule_delegator_bond_less() -> Weight;
	fn execute_revoke_delegation() -> Weight;
	fn execute_delegator_bond_less() -> Weight;
	fn cancel_revoke_delegation() -> Weight;
	fn cancel_delegator_bond_less() -> Weight;
	fn round_transition_on_initialize(x: u32, y: u32) -> Weight;
	fn base_on_initialize() -> Weight;
	fn pay_one_collator_reward(y: u32) -> Weight;
}

/// Weights for parachain_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn hotfix_remove_delegation_requests(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 3_000
			.saturating_add((8_132_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	fn hotfix_update_candidate_pool_value(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 147_000
			.saturating_add((26_825_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_staking_expectations() -> Weight {
		(20_719_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_inflation() -> Weight {
		(63_011_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_parachain_bond_account() -> Weight {
		(20_434_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_parachain_bond_reserve_percent() -> Weight {
		(19_239_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_total_selected() -> Weight {
		(18_402_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_collator_commission() -> Weight {
		(18_178_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_blocks_per_round() -> Weight {
		(65_939_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn join_candidates(x: u32) -> Weight {
		(84_807_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((333_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn schedule_leave_candidates(x: u32) -> Weight {
		(64_426_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((332_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn execute_leave_candidates(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 8_000
			.saturating_add((27_557_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn cancel_leave_candidates(x: u32) -> Weight {
		(48_521_000 as Weight)
			.saturating_add((347_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn go_offline() -> Weight {
		(36_577_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn go_online() -> Weight {
		(36_134_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn candidate_bond_more() -> Weight {
		(58_445_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn schedule_candidate_bond_less() -> Weight {
		(59_421_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn execute_candidate_bond_less() -> Weight {
		(58_228_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn cancel_candidate_bond_less() -> Weight {
		(58_327_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn delegate(x: u32, y: u32) -> Weight {
		(71_656_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_049_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 5_000
			.saturating_add((947_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn schedule_leave_delegators() -> Weight {
		(33_227_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn execute_leave_delegators(x: u32) -> Weight {
		(36_354_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((694_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn cancel_leave_delegators() -> Weight {
		(32_992_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn schedule_revoke_delegation() -> Weight {
		(37_580_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn delegator_bond_more() -> Weight {
		(71_021_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn schedule_delegator_bond_less() -> Weight {
		(70_859_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn execute_revoke_delegation() -> Weight {
		(36_912_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn execute_delegator_bond_less() -> Weight {
		(71_419_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn cancel_revoke_delegation() -> Weight {
		(37_923_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn cancel_delegator_bond_less() -> Weight {
		(70_813_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn round_transition_on_initialize(x: u32, y: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 1_378_000
			.saturating_add((47_519_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 12_000
			.saturating_add((1_275_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn base_on_initialize() -> Weight {
		(4_913_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn pay_one_collator_reward(y: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 6_000
			.saturating_add((23_284_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn hotfix_remove_delegation_requests(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 3_000
			.saturating_add((8_132_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	fn hotfix_update_candidate_pool_value(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 147_000
			.saturating_add((26_825_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_staking_expectations() -> Weight {
		(20_719_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_inflation() -> Weight {
		(63_011_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_parachain_bond_account() -> Weight {
		(20_434_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_parachain_bond_reserve_percent() -> Weight {
		(19_239_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_total_selected() -> Weight {
		(18_402_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_collator_commission() -> Weight {
		(18_178_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_blocks_per_round() -> Weight {
		(65_939_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn join_candidates(x: u32) -> Weight {
		(84_807_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((333_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn schedule_leave_candidates(x: u32) -> Weight {
		(64_426_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((332_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn execute_leave_candidates(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 8_000
			.saturating_add((27_557_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn cancel_leave_candidates(x: u32) -> Weight {
		(48_521_000 as Weight)
			.saturating_add((347_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn go_offline() -> Weight {
		(36_577_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn go_online() -> Weight {
		(36_134_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn candidate_bond_more() -> Weight {
		(58_445_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn schedule_candidate_bond_less() -> Weight {
		(59_421_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn execute_candidate_bond_less() -> Weight {
		(58_228_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn cancel_candidate_bond_less() -> Weight {
		(58_327_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// execute cancel
	fn delegate(x: u32, y: u32) -> Weight {
		(71_656_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_049_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 5_000
			.saturating_add((947_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn schedule_leave_delegators() -> Weight {
		(33_227_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn execute_leave_delegators(x: u32) -> Weight {
		(36_354_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((692_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn cancel_leave_delegators() -> Weight {
		(32_992_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn schedule_revoke_delegation() -> Weight {
		(37_580_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn delegator_bond_more() -> Weight {
		(71_021_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn schedule_delegator_bond_less() -> Weight {
		(70_859_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn execute_revoke_delegation() -> Weight {
		(36_912_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn execute_delegator_bond_less() -> Weight {
		(71_419_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn cancel_revoke_delegation() -> Weight {
		(37_923_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn cancel_delegator_bond_less() -> Weight {
		(70_813_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn round_transition_on_initialize(x: u32, y: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 1_378_000
			.saturating_add((47_519_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 12_000
			.saturating_add((1_275_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn base_on_initialize() -> Weight {
		(4_913_000 as Weight).saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn pay_one_collator_reward(y: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 6_000
			.saturating_add((23_284_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
}

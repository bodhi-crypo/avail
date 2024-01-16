// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_succinct`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `fedora`, CPU: `13th Gen Intel(R) Core(TM) i7-13700K`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/data-avail
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_succinct
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/pallet_succinct.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_succinct`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_succinct::WeightInfo for WeightInfo<T> {
	/// Storage: `Succinct::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Succinct::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn send_message_arbitrary_message(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `41487`
		// Minimum execution time: 8_142_000 picoseconds.
		Weight::from_parts(8_494_599, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(209, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Succinct::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Succinct::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn send_message_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `41487`
		// Minimum execution time: 43_326_000 picoseconds.
		Weight::from_parts(43_924_000, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Succinct::SyncCommitteePoseidons` (r:0 w:1)
	/// Proof: `Succinct::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_poseidon_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_703_000 picoseconds.
		Weight::from_parts(6_926_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Succinct::Broadcasters` (r:1 w:1)
	/// Proof: `Succinct::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_broadcaster() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `3501`
		// Minimum execution time: 9_122_000 picoseconds.
		Weight::from_parts(9_642_000, 0)
			.saturating_add(Weight::from_parts(0, 3501))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Succinct::WhitelistedDomains` (r:0 w:1)
	/// Proof: `Succinct::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	fn set_whitelisted_domains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_562_000 picoseconds.
		Weight::from_parts(5_926_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Succinct::ConfigurationStorage` (r:0 w:1)
	/// Proof: `Succinct::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	fn set_configuration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_581_000 picoseconds.
		Weight::from_parts(5_844_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Succinct::SourceChainFrozen` (r:0 w:1)
	/// Proof: `Succinct::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	fn source_chain_froze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_550_000 picoseconds.
		Weight::from_parts(5_890_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Succinct::ConfigurationStorage` (r:1 w:0)
	/// Proof: `Succinct::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::SyncCommitteePoseidons` (r:1 w:0)
	/// Proof: `Succinct::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::Head` (r:1 w:1)
	/// Proof: `Succinct::Head` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::Headers` (r:1 w:1)
	/// Proof: `Succinct::Headers` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::ExecutionStateRoots` (r:1 w:1)
	/// Proof: `Succinct::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::Timestamps` (r:0 w:1)
	/// Proof: `Succinct::Timestamps` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	fn fulfill_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `202`
		//  Estimated: `3505`
		// Minimum execution time: 7_807_768_000 picoseconds.
		Weight::from_parts(7_840_637_000, 0)
			.saturating_add(Weight::from_parts(0, 3505))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Succinct::MessageStatus` (r:1 w:1)
	/// Proof: `Succinct::MessageStatus` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::Broadcasters` (r:1 w:0)
	/// Proof: `Succinct::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::SourceChainFrozen` (r:1 w:0)
	/// Proof: `Succinct::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	/// Storage: `Succinct::ExecutionStateRoots` (r:1 w:0)
	/// Proof: `Succinct::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn execute_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6196`
		// Minimum execution time: 84_089_000 picoseconds.
		Weight::from_parts(86_128_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// The range of component `l` is `[0, 102400]`.
	fn execute_arbitrary_message(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_976_000 picoseconds.
		Weight::from_parts(11_283_893, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_161, 0).saturating_mul(l.into()))
	}
}
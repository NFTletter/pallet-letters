
//! Autogenerated weights for `pallet_letters`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-27, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_letters
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --json-file=raw.json
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn init_letter(t: u32, _a: u32, ) -> Weight;
	fn write_page(p: u32, ) -> Weight;
	fn set_price() -> Weight;
	fn transfer() -> Weight;
	fn buy_letter() -> Weight;
}

/// Weight functions for `pallet_letters`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Letters Nonce (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:1)
	// Storage: Letters OwnedLettersCount (r:1 w:1)
	// Storage: Letters AllLettersCount (r:1 w:1)
	// Storage: Letters Letters (r:0 w:1)
	// Storage: Letters AllLettersArray (r:0 w:1)
	// Storage: Letters OwnedLettersArray (r:0 w:1)
	// Storage: Letters OwnedLettersIndex (r:0 w:1)
	// Storage: Letters AllLettersIndex (r:0 w:1)
	fn init_letter(_t: u32, _a: u32, ) -> Weight {
		(68_547_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Letters Letters (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:0)
	fn write_page(p: u32, ) -> Weight {
		(60_317_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((1_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Letters Letters (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:0)
	fn set_price() -> Weight {
		(29_718_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Letters LetterOwner (r:1 w:1)
	// Storage: Letters Letters (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Letters OwnedLettersCount (r:2 w:2)
	// Storage: Letters OwnedLettersIndex (r:1 w:2)
	// Storage: Letters OwnedLettersArray (r:1 w:3)
	fn transfer() -> Weight {
		(87_866_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	// Storage: Letters Letters (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Letters OwnedLettersCount (r:2 w:2)
	// Storage: Letters OwnedLettersIndex (r:1 w:2)
	// Storage: Letters OwnedLettersArray (r:1 w:3)
	fn buy_letter() -> Weight {
		(114_926_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
}

impl WeightInfo for () {
	// Storage: Letters Nonce (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:1)
	// Storage: Letters OwnedLettersCount (r:1 w:1)
	// Storage: Letters AllLettersCount (r:1 w:1)
	// Storage: Letters Letters (r:0 w:1)
	// Storage: Letters AllLettersArray (r:0 w:1)
	// Storage: Letters OwnedLettersArray (r:0 w:1)
	// Storage: Letters OwnedLettersIndex (r:0 w:1)
	// Storage: Letters AllLettersIndex (r:0 w:1)
	fn init_letter(_t: u32, _a: u32, ) -> Weight {
		(68_547_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	// Storage: Letters Letters (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:0)
	fn write_page(p: u32, ) -> Weight {
		(60_317_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((1_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Letters Letters (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:0)
	fn set_price() -> Weight {
		(29_718_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Letters LetterOwner (r:1 w:1)
	// Storage: Letters OwnedLettersCount (r:2 w:2)
	// Storage: Letters OwnedLettersIndex (r:1 w:2)
	// Storage: Letters OwnedLettersArray (r:1 w:3)
	fn transfer() -> Weight {
		(87_866_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	// Storage: Letters Letters (r:1 w:1)
	// Storage: Letters LetterOwner (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Letters OwnedLettersCount (r:2 w:2)
	// Storage: Letters OwnedLettersIndex (r:1 w:2)
	// Storage: Letters OwnedLettersArray (r:1 w:3)
	fn buy_letter() -> Weight {
		(114_926_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
}

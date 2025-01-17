//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_vesting (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::weights::WeightInfo for WeightInfo<T> {
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vest_locked(l: u32, s: u32) -> Weight {
        (62_732_000 as Weight)
            // Standard Error: 12_000
            .saturating_add((139_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 25_000
            .saturating_add((172_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vest_unlocked(l: u32, _s: u32) -> Weight {
        (66_055_000 as Weight)
            // Standard Error: 12_000
            .saturating_add((211_000 as Weight).saturating_mul(l as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn vest_other_locked(l: u32, s: u32) -> Weight {
        (65_909_000 as Weight)
            // Standard Error: 14_000
            .saturating_add((121_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 29_000
            .saturating_add((149_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn vest_other_unlocked(l: u32, s: u32) -> Weight {
        (62_644_000 as Weight)
            // Standard Error: 10_000
            .saturating_add((129_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 20_000
            .saturating_add((55_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vested_transfer(l: u32, _s: u32) -> Weight {
        (109_233_000 as Weight)
            // Standard Error: 13_000
            .saturating_add((163_000 as Weight).saturating_mul(l as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    // Storage: Balances Locks (r:1 w:1)
    fn force_vested_transfer(l: u32, s: u32) -> Weight {
        (96_185_000 as Weight)
            // Standard Error: 18_000
            .saturating_add((283_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 36_000
            .saturating_add((87_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn not_unlocking_merge_schedules(l: u32, s: u32) -> Weight {
        (72_530_000 as Weight)
            // Standard Error: 16_000
            .saturating_add((108_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 34_000
            .saturating_add((3_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Vesting Vesting (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn unlocking_merge_schedules(l: u32, _s: u32) -> Weight {
        (80_344_000 as Weight)
            // Standard Error: 13_000
            .saturating_add((34_000 as Weight).saturating_mul(l as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}

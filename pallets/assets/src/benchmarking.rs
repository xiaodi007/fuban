#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;

benchmarks! {
    create_asset {
        let a in 1 .. 1000;
        let initial_supply = 1000u32.into();
    }: _(RawOrigin::Signed(account("test", a, SEED)), a, initial_supply)

    transfer_asset {
        let a in 1 .. 1000;
        let b = account("test", a, SEED);
        let asset_id = a;
        let amount = 500u32.into();
        AssetPallet::create_asset(RawOrigin::Signed(a).into(), asset_id, 1000u32.into())?;
    }: _(RawOrigin::Signed(a), b, asset_id, amount)

    freeze_asset {
        let a in 1 .. 1000;
        let asset_id = a;
        AssetPallet::create_asset(RawOrigin::Signed(a).into(), asset_id, 1000u32.into())?;
    }: _(RawOrigin::Signed(a), asset_id)

    unfreeze_asset {
        let a in 1 .. 1000;
        let asset_id = a;
        AssetPallet::create_asset(RawOrigin::Signed(a).into(), asset_id, 1000u32.into())?;
        AssetPallet::freeze_asset(RawOrigin::Signed(a).into(), asset_id)?;
    }: _(RawOrigin::Signed(a), asset_id)
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);

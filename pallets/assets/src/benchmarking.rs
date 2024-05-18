#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    register_asset {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), 1, AssetMetadata { name: b"Test Asset".to_vec() })
    verify {
        assert_eq!(Assets::<T>::get(1), Some(AssetMetadata { name: b"Test Asset".to_vec() }));
    }

    mint {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = whitelisted_caller();
        Assets::<T>::insert(1, AssetMetadata { name: b"Test Asset".to_vec() });
    }: _(RawOrigin::Signed(caller), 1, recipient.clone(), 100u32.into())
    verify {
        assert_eq!(Balances::<T>::get((1, recipient)), 100u32.into());
    }

    burn {
        let caller: T::AccountId = whitelisted_caller();
        let holder: T::AccountId = whitelisted_caller();
        Assets::<T>::insert(1, AssetMetadata { name: b"Test Asset".to_vec() });
        Balances::<T>::insert((1, holder.clone()), 100u32.into());
    }: _(RawOrigin::Signed(caller), 1, holder.clone(), 50u32.into())
    verify {
        assert_eq!(Balances::<T>::get((1, holder)), 50u32.into());
    }
}

impl_benchmark_test_suite!(AssetModule, crate::mock::new_test_ext(), crate::mock::Test);

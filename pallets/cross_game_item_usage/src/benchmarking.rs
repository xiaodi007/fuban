#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    register_game {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), 1, GameMetadata { name: b"Test Game".to_vec() })
    verify {
        assert_eq!(Games::<T>::get(1), Some(GameMetadata { name: b"Test Game".to_vec() }));
    }

    transfer_item {
        let caller: T::AccountId = whitelisted_caller();
        Games::<T>::insert(1, GameMetadata { name: b"Test Game".to_vec() });
        Games::<T>::insert(2, GameMetadata { name: b"Test Game 2".to_vec() });
        GameItems::<T>::insert((1, 1), caller.clone());
    }: _(RawOrigin::Signed(caller), 1, 1, 2)
    verify {
        assert_eq!(GameItems::<T>::get((2, 1)), caller);
    }
}

impl_benchmark_test_suite!(CrossGameItemUsageModule, crate::mock::new_test_ext(), crate::mock::Test);

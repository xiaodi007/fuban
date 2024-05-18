#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    borrow {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), 1, 100u32.into(), 10u32.into())
    verify {
        assert_eq!(Loans::<T>::get((1, caller)), (100u32.into(), 10u32.into()));
    }

    repay {
        let caller: T::AccountId = whitelisted_caller();
        Loans::<T>::insert((1, caller.clone()), (100u32.into(), 10u32.into()));
    }: _(RawOrigin::Signed(caller), 1, 50u32.into())
    verify {
        assert_eq!(Loans::<T>::get((1, caller)), (50u32.into(), 10u32.into()));
    }
}

impl_benchmark_test_suite!(BorrowingModule, crate::mock::new_test_ext(), crate::mock::Test);

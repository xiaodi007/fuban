#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    propose_change {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), 1, ProposalDetails { description: b"Test Proposal".to_vec() })
    verify {
        assert_eq!(Proposals::<T>::get(1), ProposalDetails { description: b"Test Proposal".to_vec() });
    }

    vote {
        let caller: T::AccountId = whitelisted_caller();
        Proposals::<T>::insert(1, ProposalDetails { description: b"Test Proposal".to_vec() });
    }: _(RawOrigin::Signed(caller), 1, Vote::Yes)
    verify {
        assert_eq!(Votes::<T>::get((1, caller)), Vote::Yes);
    }

    implement_proposal {
        let caller: T::AccountId = whitelisted_caller();
        Proposals::<T>::insert(1, ProposalDetails { description: b"Test Proposal".to_vec() });
    }: _(RawOrigin::Signed(caller), 1)
    verify {
        assert!(!Proposals::<T>::contains_key(1));
    }
}

impl_benchmark_test_suite!(GovernanceModule, crate::mock::new_test_ext(), crate::mock::Test);

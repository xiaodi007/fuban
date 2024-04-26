#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;

benchmarks! {
    update_interest_rate {
        let r in 0.0..1.0;
    }: _(RawOrigin::Signed(account("user", 1, 1000)), r)

    impl_benchmark_test_suite!(InterestRateModel, crate::mock::new_test_ext(), crate::mock::Test);
}

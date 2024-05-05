#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use frame_benchmarking::{benchmarks, account, whitelisted_caller};
    use frame_system::RawOrigin;

    const SEED: u32 = 0;

    benchmarks! {
        deposit {
            let d in 1 .. 1000;
            let depositor: T::AccountId = account("depositor", d, SEED);
            let asset_id = 1u32;
            let amount: BalanceOf<T> = 100u32.into();

            // Make sure the depositor has enough balance to deposit
            T::Currency::make_free_balance_be(&depositor, amount + 100u32.into());

            // Ensure the pallet has an account to accept deposits
            let pallet_account = Pallet::<T>::account_id();
            T::Currency::make_free_balance_be(&pallet_account, BalanceOf::<T>::zero());

        }: _(RawOrigin::Signed(depositor.clone()), asset_id, amount)
        verify {
            assert_eq!(LoanMarket::total_deposits(asset_id), amount);
        }

        borrow {
            let b in 1 .. 1000;
            let borrower: T::AccountId = account("borrower", b, SEED);
            let asset_id = 1u32;
            let deposit_amount: BalanceOf<T> = 1000u32.into();
            let borrow_amount: BalanceOf<T> = 500u32.into();

            // Setup initial conditions: deposit some amount into the market
            let depositor: T::AccountId = whitelisted_caller();
            T::Currency::make_free_balance_be(&depositor, deposit_amount + 100u32.into());
            assert_ok!(LoanMarket::deposit(RawOrigin::Signed(depositor.clone()).into(), asset_id, deposit_amount));

            // Ensure the borrower has an account and a little balance for transaction fees
            T::Currency::make_free_balance_be(&borrower, 100u32.into());

        }: _(RawOrigin::Signed(borrower), asset_id, borrow_amount)
        verify {
            assert_eq!(T::Currency::free_balance(&borrower), 100u32.into() + borrow_amount);
        }
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

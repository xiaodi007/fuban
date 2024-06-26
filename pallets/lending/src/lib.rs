#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_runtime::traits::Saturating;

pub trait Lending {
    type AssetId;
    type AccountId;
    type Balance;
    type Moment;

    fn deposit(asset_id: Self::AssetId, from: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult;
    fn withdraw(asset_id: Self::AssetId, to: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult;
    fn borrow(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance, duration: Self::Moment) -> dispatch::DispatchResult;
    fn repay(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult;
    fn calculate_interest(asset_id: Self::AssetId, amount: Self::Balance, duration: Self::Moment) -> Self::Balance;
}

decl_storage! {
    trait Store for Module<T: Config> as LendingModule {
        pub Deposits get(fn deposits): map hasher(blake2_128_concat) (T::AssetId, T::AccountId) => T::Balance;
        pub Loans get(fn loans): map hasher(blake2_128_concat) (T::AssetId, T::AccountId) => (T::Balance, T::Moment);
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit(origin, asset_id: T::AssetId, amount: T::Balance) -> dispatch::DispatchResult {
            let from = ensure_signed(origin)?;
            let new_balance = Self::deposits((asset_id, from.clone())).saturating_add(amount);
            <Deposits<T>>::insert((asset_id, from.clone()), new_balance);
            Self::deposit_event(RawEvent::Deposited(asset_id, from, amount));
            Ok(())
        }

        fn withdraw(origin, asset_id: T::AssetId, amount: T::Balance) -> dispatch::DispatchResult {
            let to = ensure_signed(origin)?;
            let current_balance = Self::deposits((asset_id, to.clone()));
            let new_balance = current_balance.checked_sub(&amount).ok_or(Error::<T>::InsufficientBalance)?;
            <Deposits<T>>::insert((asset_id, to.clone()), new_balance);
            Self::deposit_event(RawEvent::Withdrawn(asset_id, to, amount));
            Ok(())
        }

        fn borrow(origin, asset_id: T::AssetId, amount: T::Balance, duration: T::Moment) -> dispatch::DispatchResult {
            let borrower = ensure_signed(origin)?;
            let new_loan = (amount, duration);
            <Loans<T>>::insert((asset_id, borrower.clone()), new_loan);
            Self::deposit_event(RawEvent::Borrowed(asset_id, borrower, amount, duration));
            Ok(())
        }

        fn repay(origin, asset_id: T::AssetId, amount: T::Balance) -> dispatch::DispatchResult {
            let borrower = ensure_signed(origin)?;
            let (current_loan, duration) = Self::loans((asset_id, borrower.clone()));
            let new_loan = current_loan.checked_sub(&amount).ok_or(Error::<T>::InsufficientLoan)?;
            <Loans<T>>::insert((asset_id, borrower.clone()), (new_loan, duration));
            Self::deposit_event(RawEvent::Repaid(asset_id, borrower, amount));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, AssetId = <T as Config>::AssetId, Balance = <T as Config>::Balance, Moment = <T as Config>::Moment {
        Deposited(AssetId, AccountId, Balance),
        Withdrawn(AssetId, AccountId, Balance),
        Borrowed(AssetId, AccountId, Balance, Moment),
        Repaid(AssetId, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        InsufficientBalance,
        InsufficientLoan,
    }
}

impl<T: Config> Lending for Module<T> {
    type AssetId = T::AssetId;
    type AccountId = T::AccountId;
    type Balance = T::Balance;
    type Moment = T::Moment;

    fn deposit(asset_id: Self::AssetId, from: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult {
        let new_balance = Self::deposits((asset_id, from.clone())).saturating_add(amount);
        <Deposits<T>>::insert((asset_id, from.clone()), new_balance);
        Ok(())
    }

    fn withdraw(asset_id: Self::AssetId, to: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult {
        let current_balance = Self::deposits((asset_id, to.clone()));
        let new_balance = current_balance.checked_sub(&amount).ok_or(Error::<T>::InsufficientBalance)?;
        <Deposits<T>>::insert((asset_id, to.clone()), new_balance);
        Ok(())
    }

    fn borrow(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance, duration: Self::Moment) -> dispatch::DispatchResult {
        let new_loan = (amount, duration);
        <Loans<T>>::insert((asset_id, borrower.clone()), new_loan);
        Ok(())
    }

    fn repay(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult {
        let (current_loan, duration) = Self::loans((asset_id, borrower.clone()));
        let new_loan = current_loan.checked_sub(&amount).ok_or(Error::<T>::InsufficientLoan)?;
        <Loans<T>>::insert((asset_id, borrower.clone()), (new_loan, duration));
        Ok(())
    }

    fn calculate_interest(asset_id: Self::AssetId, amount: Self::Balance, duration: Self::Moment) -> Self::Balance {
        // Implement interest calculation logic here
        // For simplicity, we'll use a fixed rate of 5% per duration unit
        let rate: Self::Balance = 5.into();
        amount.saturating_mul(rate).saturating_mul(duration.into())
    }
}

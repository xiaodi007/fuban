#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_runtime::traits::Saturating;

pub trait Borrowing {
    type AssetId;
    type AccountId;
    type Balance;
    type Moment;

    fn borrow(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance, duration: Self::Moment) -> dispatch::DispatchResult;
    fn repay(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult;
    fn get_borrowed_amount(asset_id: Self::AssetId, borrower: Self::AccountId) -> Self::Balance;
    fn get_borrowing_duration(asset_id: Self::AssetId, borrower: Self::AccountId) -> Self::Moment;
}

decl_storage! {
    trait Store for Module<T: Config> as BorrowingModule {
        pub Loans get(fn loans): map hasher(blake2_128_concat) (T::AssetId, T::AccountId) => (T::Balance, T::Moment);
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
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
        Borrowed(AssetId, AccountId, Balance, Moment),
        Repaid(AssetId, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        InsufficientLoan,
    }
}

impl<T: Config> Borrowing for Module<T> {
    type AssetId = T::AssetId;
    type AccountId = T::AccountId;
    type Balance = T::Balance;
    type Moment = T::Moment;

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

    fn get_borrowed_amount(asset_id: Self::AssetId, borrower: Self::AccountId) -> Self::Balance {
        let (amount, _) = Self::loans((asset_id, borrower));
        amount
    }

    fn get_borrowing_duration(asset_id: Self::AssetId, borrower: Self::AccountId) -> Self::Moment {
        let (_, duration) = Self::loans((asset_id, borrower));
        duration
    }
}

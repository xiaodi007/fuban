#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, ExistenceRequirement::AllowDeath},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, CheckedAdd, Zero};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    type AssetIdOf<T> = u32; // Simplified asset ID type for demonstration

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Deposited(T::AccountId, AssetIdOf<T>, BalanceOf<T>),
        Borrowed(T::AccountId, AssetIdOf<T>, BalanceOf<T>),
        InterestAccrued(T::AccountId, AssetIdOf<T>, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        InsufficientBalance,
        Overflow,
        InterestCalculationError,
    }

    // This would store the total balances deposited per asset type
    #[pallet::storage]
    #[pallet::getter(fn total_deposits)]
    pub type TotalDeposits<T: Config> = StorageMap<_, Blake2_128Concat, AssetIdOf<T>, BalanceOf<T>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn deposit(origin: OriginFor<T>, asset_id: AssetIdOf<T>, amount: BalanceOf<T>) -> DispatchResult {
            let depositor = ensure_signed(origin)?;

            T::Currency::transfer(&depositor, &Self::account_id(), amount, AllowDeath)?;

            TotalDeposits::<T>::mutate(asset_id, |balance| *balance = balance.checked_add(&amount).ok_or(Error::<T>::Overflow)?);

            Self::deposit_event(Event::Deposited(depositor, asset_id, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn borrow(origin: OriginFor<T>, asset_id: AssetIdOf<T>, amount: BalanceOf<T>) -> DispatchResult {
            let borrower = ensure_signed(origin)?;

            // Simplified borrowing logic assuming no collateral requirements
            T::Currency::transfer(&Self::account_id(), &borrower, amount, AllowDeath)?;

            Self::deposit_event(Event::Borrowed(borrower, asset_id, amount));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn account_id() -> T::AccountId {
            PALLET_ID.into_account()
        }
    }

    const PALLET_ID: frame_support::PalletId = frame_support::PalletId(*b"ex/lnmrk");
}

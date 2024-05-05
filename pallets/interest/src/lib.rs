#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    pub type CurrentInterestRate<T: Config> = StorageValue<_, f32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        InterestRateUpdated(f32),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        InvalidRate,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn update_interest_rate(origin: OriginFor<T>, new_rate: f32) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Validate new rate
            ensure!(new_rate >= 0.0 && new_rate <= 1.0, Error::<T>::InvalidRate);

            CurrentInterestRate::<T>::put(new_rate);
            Self::deposit_event(Event::InterestRateUpdated(new_rate));
            Ok(())
        }
    }
}

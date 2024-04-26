#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::{Currency, ReservableCurrency}};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::StaticLookup;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type AssetId: Parameter + Member + Copy + MaybeSerializeDeserialize;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AssetCreated(T::AccountId, T::AssetId),
        AssetTransferred(T::AccountId, T::AccountId, T::AssetId, BalanceOf<T>),
        AssetFrozen(T::AssetId),
        AssetUnfrozen(T::AssetId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        InsufficientBalance,
        NotPermitted,
        AssetFrozen,
    }

    #[pallet::storage]
    pub type Assets<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type FrozenAssets<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, bool, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn create_asset(origin: OriginFor<T>, asset_id: T::AssetId, initial_supply: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Creating an asset with initial supply
            Assets::<T>::insert(asset_id, initial_supply);
            T::Currency::deposit_creating(&who, initial_supply);

            Self::deposit_event(Event::AssetCreated(who, asset_id));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_asset(origin: OriginFor<T>, to: T::AccountId, asset_id: T::AssetId, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if asset is frozen
            ensure!(!FrozenAssets::<T>::get(&asset_id), Error::<T>::AssetFrozen);

            // Logic to transfer assets
            T::Currency::transfer(&who, &to, amount, ExistenceRequirement::KeepAlive)?;

            Self::deposit_event(Event::AssetTransferred(who, to, asset_id, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn freeze_asset(origin: OriginFor<T>, asset_id: T::AssetId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure only the asset creator can freeze it (simplified permission check)
            ensure!(Assets::<T>::contains_key(&asset_id), Error::<T>::NotPermitted);

            FrozenAssets::<T>::insert(asset_id, true);
            Self::deposit_event(Event::AssetFrozen(asset_id));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn unfreeze_asset(origin: OriginFor<T>, asset_id: T::AssetId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(Assets::<T>::contains_key(&asset_id), Error::<T>::NotPermitted);

            FrozenAssets::<T>::insert(asset_id, false);
            Self::deposit_event(Event::AssetUnfrozen(asset_id));
            Ok(())
        }
    }
}

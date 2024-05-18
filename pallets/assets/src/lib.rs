#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::vec::Vec;
use codec::{Decode, Encode};
use sp_std::vec::Vec;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct AssetMetadata {
    pub name: Vec<u8>,
    pub symbol: Vec<u8>,
    pub decimals: u8,
}
pub trait AssetManager {
    type AssetId;
    type AccountId;
    type Balance;

    fn register_asset(asset_id: Self::AssetId, metadata: AssetMetadata) -> dispatch::DispatchResult;
    fn get_asset(asset_id: Self::AssetId) -> Option<AssetMetadata>;
    fn mint(asset_id: Self::AssetId, to: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult;
    fn burn(asset_id: Self::AssetId, from: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult;
}

decl_storage! {
    trait Store for Module<T: Config> as AssetModule {
        pub Assets get(fn assets): map hasher(blake2_128_concat) T::AssetId => Option<AssetMetadata>;
        pub Balances get(fn balances): map hasher(blake2_128_concat) (T::AssetId, T::AccountId) => T::Balance;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn register_asset(origin, asset_id: T::AssetId, metadata: AssetMetadata) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            // Register the asset
            <Assets<T>>::insert(asset_id, metadata);
            Self::deposit_event(RawEvent::AssetRegistered(asset_id));
            Ok(())
        }

        fn mint(origin, asset_id: T::AssetId, to: T::AccountId, amount: T::Balance) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            // Mint the asset
            let new_balance = Self::balances((asset_id, to.clone())).saturating_add(amount);
            <Balances<T>>::insert((asset_id, to.clone()), new_balance);
            Self::deposit_event(RawEvent::AssetMinted(asset_id, to, amount));
            Ok(())
        }

        fn burn(origin, asset_id: T::AssetId, from: T::AccountId, amount: T::Balance) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            // Burn the asset
            let current_balance = Self::balances((asset_id, from.clone()));
            let new_balance = current_balance.checked_sub(&amount).ok_or(Error::<T>::InsufficientBalance)?;
            <Balances<T>>::insert((asset_id, from.clone()), new_balance);
            Self::deposit_event(RawEvent::AssetBurned(asset_id, from, amount));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, AssetId = <T as Config>::AssetId, Balance = <T as Config>::Balance {
        AssetRegistered(AssetId),
        AssetMinted(AssetId, AccountId, Balance),
        AssetBurned(AssetId, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        InsufficientBalance,
    }
}

impl<T: Config> AssetManager for Module<T> {
    type AssetId = T::AssetId;
    type AccountId = T::AccountId;
    type Balance = T::Balance;

    fn register_asset(asset_id: Self::AssetId, metadata: AssetMetadata) -> dispatch::DispatchResult {
        <Assets<T>>::insert(asset_id, metadata);
        Ok(())
    }

    fn get_asset(asset_id: Self::AssetId) -> Option<AssetMetadata> {
        <Assets<T>>::get(asset_id)
    }

    fn mint(asset_id: Self::AssetId, to: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult {
        let new_balance = Self::balances((asset_id, to.clone())).saturating_add(amount);
        <Balances<T>>::insert((asset_id, to.clone()), new_balance);
        Ok(())
    }

    fn burn(asset_id: Self::AssetId, from: Self::AccountId, amount: Self::Balance) -> dispatch::DispatchResult {
        let current_balance = Self::balances((asset_id, from.clone()));
        let new_balance = current_balance.checked_sub(&amount).ok_or(Error::<T>::InsufficientBalance)?;
        <Balances<T>>::insert((asset_id, from.clone()), new_balance);
        Ok(())
    }
}

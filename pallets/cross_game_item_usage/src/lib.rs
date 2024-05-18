#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

pub trait CrossGameItemUsage {
    type GameItemId;
    type AccountId;

    fn register_game(game_id: u32, game_metadata: GameMetadata) -> dispatch::DispatchResult;
    fn get_game(game_id: u32) -> Option<GameMetadata>;
    fn transfer_item(item_id: Self::GameItemId, from_game: u32, to_game: u32, owner: Self::AccountId) -> dispatch::DispatchResult;
}

decl_storage! {
    trait Store for Module<T: Config> as CrossGameItemUsageModule {
        pub Games get(fn games): map hasher(blake2_128_concat) u32 => Option<GameMetadata>;
        pub GameItems get(fn game_items): map hasher(blake2_128_concat) (u32, T::GameItemId) => T::AccountId;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn register_game(origin, game_id: u32, game_metadata: GameMetadata) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            <Games>::insert(game_id, game_metadata);
            Self::deposit_event(RawEvent::GameRegistered(game_id));
            Ok(())
        }

        fn transfer_item(origin, item_id: T::GameItemId, from_game: u32, to_game: u32) -> dispatch::DispatchResult {
            let owner = ensure_signed(origin)?;
            ensure!(<GameItems<T>>::contains_key((from_game, item_id)), Error::<T>::ItemNotFound);
            let item_owner = Self::game_items((from_game, item_id));
            ensure!(item_owner == owner, Error::<T>::NotItemOwner);
            <GameItems<T>>::remove((from_game, item_id));
            <GameItems<T>>::insert((to_game, item_id), owner.clone());
            Self::deposit_event(RawEvent::ItemTransferred(item_id, from_game, to_game, owner));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, GameItemId = <T as Config>::GameItemId {
        GameRegistered(u32),
        ItemTransferred(GameItemId, u32, u32, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        ItemNotFound,
        NotItemOwner,
    }
}

impl<T: Config> CrossGameItemUsage for Module<T> {
    type GameItemId = T::GameItemId;
    type AccountId = T::AccountId;

    fn register_game(game_id: u32, game_metadata: GameMetadata) -> dispatch::DispatchResult {
        <Games>::insert(game_id, game_metadata);
        Ok(())
    }

    fn get_game(game_id: u32) -> Option<GameMetadata> {
        <Games>::get(game_id)
    }

    fn transfer_item(item_id: Self::GameItemId, from_game: u32, to_game: u32, owner: Self::AccountId) -> dispatch::DispatchResult {
        ensure!(<GameItems<T>>::contains_key((from_game, item_id)), Error::<T>::ItemNotFound);
        let item_owner = Self::game_items((from_game, item_id));
        ensure!(item_owner == owner, Error::<T>::NotItemOwner);
        <GameItems<T>>::remove((from_game, item_id));
        <GameItems<T>>::insert((to_game, item_id), owner.clone());
        Ok(())
    }
}

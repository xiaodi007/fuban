#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_cross_game_item_usage;
    use frame_support::{assert_ok, impl_outer_origin, parameter_types};
    use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
    use sp_core::H256;

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }
    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
    }

    impl Config for Test {
        type Event = ();
    }

    type CrossGameItemUsageModule = Module<Test>;

    #[test]
    fn it_registers_a_game() {
        new_test_ext().execute_with(|| {
            // Test registration of game
            assert_ok!(CrossGameItemUsageModule::register_game(Origin::signed(1), 1, GameMetadata { name: b"Test Game".to_vec() }));
            assert_eq!(CrossGameItemUsageModule::get_game(1), Some(GameMetadata { name: b"Test Game".to_vec() }));
        });
    }

    #[test]
    fn it_transfers_item() {
        new_test_ext().execute_with(|| {
            // Test item transfer between games
            CrossGameItemUsageModule::register_game(Origin::signed(1), 1, GameMetadata { name: b"Test Game".to_vec() }).unwrap();
            CrossGameItemUsageModule::register_game(Origin::signed(1), 2, GameMetadata { name: b"Test Game 2".to_vec() }).unwrap();
            CrossGameItemUsageModule::game_items((1, 1)).insert(1, 1);
            assert_ok!(CrossGameItemUsageModule::transfer_item(Origin::signed(1), 1, 1, 2));
            assert_eq!(CrossGameItemUsageModule::game_items((2, 1)), 1);
        });
    }
}

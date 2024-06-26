#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_lending;
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

    type LendingModule = Module<Test>;

    #[test]
    fn it_deposits_assets() {
        new_test_ext().execute_with(|| {
            // Test deposit of assets
            assert_ok!(LendingModule::deposit(Origin::signed(1), 1, 100));
            assert_eq!(LendingModule::deposits((1, 1)), 100);
        });
    }

    #[test]
    fn it_withdraws_assets() {
        new_test_ext().execute_with(|| {
            // Test withdrawal of assets
            assert_ok!(LendingModule::deposit(Origin::signed(1), 1, 100));
            assert_ok!(LendingModule::withdraw(Origin::signed(1), 1, 50));
            assert_eq!(LendingModule::deposits((1, 1)), 50);
        });
    }
}

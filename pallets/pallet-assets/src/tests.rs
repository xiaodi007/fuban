#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop, parameter_types, traits::{GenesisBuild, OnFinalize, OnInitialize}};
    use sp_core::H256;
    use frame_system as system;

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    // This will be our runtime configuration for tests
    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Module, Call, Config, Storage, Event<T>},
            AssetsPallet: pallet::{Module, Call, Storage, Event<T>, Config},
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub static ExistentialDeposit: u64 = 1;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = sp_runtime::traits::BlakeTwo256;
        type AccountId = u32; // using simple type for account ID for testing
        type Lookup = frame_system::IdentityLookup<Self::AccountId>;
        type Header = sp_runtime::testing::Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u64>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
    }

    impl Config for Test {
        type Event = Event;
        type Currency = pallet_balances::Module<Test>;
        type AssetId = u32;
    }

    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
        let balances: Vec<(u32, u64)> = vec![(1, 5000), (2, 5000)];
        pallet_balances::GenesisConfig::<Test> {
            balances,
        }.assimilate_storage(&mut t).unwrap();
        t.into()
    }

    #[test]
    fn create_asset_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(AssetsPallet::create_asset(Origin::signed(1), 1, 1000));
            assert_eq!(Assets::<Test>::get(1), 1000);
            // Check events
            let expected_event = Event::pallet(Event::AssetCreated(1, 1));
            assert!(System::events().iter().any(|record| record.event == expected_event));
        });
    }

    #[test]
    fn transfer_asset_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(AssetsPallet::create_asset(Origin::signed(1), 1, 1000));
            assert_ok!(AssetsPallet::transfer_asset(Origin::signed(1), 2, 1, 500));
            // Check balances
            assert_eq!(Assets::<Test>::get(1), 500); // Reduced by 500
            // Check events
            let expected_event = Event::pallet(Event::AssetTransferred(1, 2, 1, 500));
            assert!(System::events().iter().any(|record| record.event == expected_event));
        });
    }

    #[test]
    fn freeze_asset_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(AssetsPallet::create_asset(Origin::signed(1), 1, 1000));
            assert_ok!(AssetsPallet::freeze_asset(Origin::signed(1), 1));
            assert_noop!(
                AssetsPallet::transfer_asset(Origin::signed(1), 2, 1, 500),
                Error::<Test>::AssetFrozen
            );
            // Check frozen state
            assert_eq!(FrozenAssets::<Test>::get(1), true);
            // Check event
            let expected_event = Event::pallet(Event::AssetFrozen(1));
            assert!(System::events().iter().any(|record| record.event == expected_event));
        });
    }

    #[test]
    fn unfreeze_asset_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(AssetsPallet::create_asset(Origin::signed(1), 1, 1000));
            assert_ok!(AssetsPallet::freeze_asset(Origin::signed(1), 1));
            assert_ok!(AssetsPallet::unfreeze_asset(Origin::signed(1), 1));
            assert_ok!(AssetsPallet::transfer_asset(Origin::signed(1), 2, 1, 500));
            // Check frozen state
            assert_eq!(FrozenAssets::<Test>::get(1), false);
            // Check event
            let expected_event = Event::pallet(Event::AssetUnfrozen(1));
            assert!(System::events().iter().any(|record| record.event == expected_event));
        });
    }
}

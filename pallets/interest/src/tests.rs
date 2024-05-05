#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop, parameter_types};
    use sp_core::H256;
    use frame_system as system;

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Module, Call, Config, Storage, Event<T>},
            InterestRateModel: pallet::{Module, Call, Storage, Event<T>, Config},
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl system::Config for Test {
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
        type AccountId = u32;
        type Lookup = frame_system::IdentityLookup<Self::AccountId>;
        type Header = sp_runtime::testing::Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
    }

    impl Config for Test {
        type Event = Event;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
        t.into()
    }

    #[test]
    fn update_interest_rate_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(InterestRateModel::update_interest_rate(Origin::signed(1), 0.05));
            assert_eq!(CurrentInterestRate::<Test>::get(), 0.05);
        });
    }

    #[test]
    fn prevent_invalid_interest_rate_update() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                InterestRateModel::update_interest_rate(Origin::signed(1), 1.1),
                Error::<Test>::InvalidRate
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop, parameter_types, ord_parameter_types, traits::{OnFinalize, OnInitialize}};
    use sp_core::H256;
    use frame_system as system;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
            LoanMarket: pallet::{Pallet, Call, Storage, Event<T>, Config},
            Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
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
        type Hashing = BlakeTwo256;
        type AccountId = u32;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
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

    impl pallet_balances::Config for Test {
        type Balance = u64;
        type DustRemoval = ();
        type Event = Event;
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type MaxLocks = ();
        type WeightInfo = ();
    }

    impl Config for Test {
        type Event = Event;
        type Currency = Balances;
    }

    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(1, 500), (2, 300)],
        }.assimilate_storage(&mut t).unwrap();
        t.into()
    }

    #[test]
    fn deposit_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(LoanMarket::deposit(Origin::signed(1), 1, 50));
            assert_eq!(LoanMarket::total_deposits(1), 50);
            assert_eq!(Balances::free_balance(&LoanMarket::account_id()), 50);
            assert_eq!(Balances::free_balance(1), 450);
        });
    }

    #[test]
    fn deposit_fails_for_insufficient_balance() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                LoanMarket::deposit(Origin::signed(1), 1, 600),
                pallet_balances::Error::<Test>::InsufficientBalance
            );
        });
    }

    #[test]
    fn borrow_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(LoanMarket::deposit(Origin::signed(1), 1, 300));
            assert_ok!(LoanMarket::borrow(Origin::signed(2), 1, 100));
            assert_eq!(Balances::free_balance(2), 400); // Original 300 + 100 borrowed
            assert_eq!(Balances::free_balance(&LoanMarket::account_id()), 200); // 300 deposited - 100 borrowed
        });
    }

    #[test]
    fn borrow_fails_due_to_lack_of_liquidity() {
        new_test_ext().execute_with(|| {
            assert_ok!(LoanMarket::deposit(Origin::signed(1), 1, 200));
            assert_noop!(
                LoanMarket::borrow(Origin::signed(2), 1, 300),
                pallet_balances::Error::<Test>::InsufficientBalance
            );
        });
    }
}

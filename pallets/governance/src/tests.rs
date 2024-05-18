#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_governance;
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

    type GovernanceModule = Module<Test>;

    #[test]
    fn it_creates_proposal() {
        new_test_ext().execute_with(|| {
            // Test proposal creation
            assert_ok!(GovernanceModule::propose_change(Origin::signed(1), 1, ProposalDetails { description: b"Test Proposal".to_vec() }));
            assert_eq!(GovernanceModule::proposals(1), ProposalDetails { description: b"Test Proposal".to_vec() });
        });
    }

    #[test]
    fn it_votes_on_proposal() {
        new_test_ext().execute_with(|| {
            // Test voting on proposal
            GovernanceModule::propose_change(Origin::signed(1), 1, ProposalDetails { description: b"Test Proposal".to_vec() }).unwrap();
            assert_ok!(GovernanceModule::vote(Origin::signed(1), 1, Vote::Yes));
            assert_eq!(GovernanceModule::votes((1, 1)), Vote::Yes);
        });
    }

    #[test]
    fn it_implements_proposal() {
        new_test_ext().execute_with(|| {
            // Test proposal implementation
            GovernanceModule::propose_change(Origin::signed(1), 1, ProposalDetails { description: b"Test Proposal".to_vec() }).unwrap();
            assert_ok!(GovernanceModule::implement_proposal(Origin::signed(1), 1));
            assert!(!GovernanceModule::proposals(1).is_some());
        });
    }
}

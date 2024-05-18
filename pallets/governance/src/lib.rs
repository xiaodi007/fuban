#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

pub trait Governance {
    type ProposalId;
    type AccountId;
    type Balance;

    fn propose_change(proposal_id: Self::ProposalId, proposer: Self::AccountId, proposal_details: ProposalDetails) -> dispatch::DispatchResult;
    fn vote(proposal_id: Self::ProposalId, voter: Self::AccountId, vote: Vote) -> dispatch::DispatchResult;
    fn implement_proposal(proposal_id: Self::ProposalId) -> dispatch::DispatchResult;
}

decl_storage! {
    trait Store for Module<T: Config> as GovernanceModule {
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) T::ProposalId => ProposalDetails;
        pub Votes get(fn votes): double_map hasher(blake2_128_concat) T::ProposalId, hasher(blake2_128_concat) T::AccountId => Vote;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn propose_change(origin, proposal_id: T::ProposalId, proposal_details: ProposalDetails) -> dispatch::DispatchResult {
            let proposer = ensure_signed(origin)?;
            <Proposals<T>>::insert(proposal_id, proposal_details);
            Self::deposit_event(RawEvent::ProposalCreated(proposal_id, proposer));
            Ok(())
        }

        fn vote(origin, proposal_id: T::ProposalId, vote: Vote) -> dispatch::DispatchResult {
            let voter = ensure_signed(origin)?;
            <Votes<T>>::insert(proposal_id, voter.clone(), vote);
            Self::deposit_event(RawEvent::Voted(proposal_id, voter, vote));
            Ok(())
        }

        fn implement_proposal(origin, proposal_id: T::ProposalId) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            ensure!(<Proposals<T>>::contains_key(proposal_id), Error::<T>::ProposalNotFound);
            // Implement the proposal logic here
            <Proposals<T>>::remove(proposal_id);
            Self::deposit_event(RawEvent::ProposalImplemented(proposal_id));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, ProposalId = <T as Config>::ProposalId {
        ProposalCreated(ProposalId, AccountId),
        Voted(ProposalId, AccountId, Vote),
        ProposalImplemented(ProposalId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        ProposalNotFound,
    }
}

impl<T: Config> Governance for Module<T> {
    type ProposalId = T::ProposalId;
    type AccountId = T::AccountId;
    type Balance = T::Balance;

    fn propose_change(proposal_id: Self::ProposalId, proposer: Self::AccountId, proposal_details: ProposalDetails) -> dispatch::DispatchResult {
        <Proposals<T>>::insert(proposal_id, proposal_details);
        Ok(())
    }

    fn vote(proposal_id: Self::ProposalId, voter: Self::AccountId, vote: Vote) -> dispatch::DispatchResult {
        <Votes<T>>::insert(proposal_id, voter.clone(), vote);
        Ok(())
    }

    fn implement_proposal(proposal_id: Self::ProposalId) -> dispatch::DispatchResult {
        ensure!(<Proposals<T>>::contains_key(proposal_id), Error::<T>::ProposalNotFound);
        <Proposals<T>>::remove(proposal_id);
        Ok(())
    }
}

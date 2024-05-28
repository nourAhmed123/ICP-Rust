use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

use ic_stable_structures::storable::Bound;
use std::{borrow::Cow, cell::RefCell};
type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 5000;
#[derive(CandidType, Deserialize)]
enum Choice {
    Approve,
    Reject,
    Pass,
}
#[derive(CandidType)]
enum VoteError {
    AlreadyVoted,
    ProposalIsNotActive,
  
    NoSuchProposal,
    AccessRejected,
    UpdateError,
}
//Data for proposal
#[derive(CandidType, Deserialize)] //automatically implement the CandidType and Deserialize traits for the Proposal struct, making it possible to serialize and deserialize instances of this struct using Candid.
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}

#[derive(CandidType, Deserialize)]
struct CreateProposal {
    description: String,
    is_active: bool,
}
//The Storable trait is implemented for the Proposal struct to enable it to be stored in stable memory.
impl Storable for Proposal {
    //Serializa
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    //Deserialize
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    // Bound definition for the Storable trait
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
thread_local! {
//we are creating this virtual memory to act as we have multiple memory locations but we have only one

static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
//proposal map to hold proposals
static PROPOSAL_MAP:RefCell<StableBTreeMap<u64,Proposal,Memory>>=RefCell::new(StableBTreeMap::init(
    MEMORY_MANAGER.with(|m|m.borrow().get(MemoryId::new(0)))
))
}

#[ic_cdk::query]
fn get_proposal(key: u64) -> Option<Proposal> {
    PROPOSAL_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_proposal_count() -> u64 {
    PROPOSAL_MAP.with(|p| p.borrow().len())
}

#[ic_cdk::update]
fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal> {
    let value: Proposal = Proposal {
        description: proposal.description,
        approve: 0u32,
        reject: 0u32,
        pass: 0u32,
        is_active: proposal.is_active,
        voted: vec![],
        owner: ic_cdk::caller(),
    };
    //borrow_mut because we want to alter the data
    PROPOSAL_MAP.with(|p| p.borrow_mut().insert(key, value))
}

#[ic_cdk::update]
fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {
    //Save the proposal with map.with
    PROPOSAL_MAP.with(|p| {
        //retrieve a proposal with a given key
        let old_proposal_opt = p.borrow().get(&key);

        let old_proposal = match old_proposal_opt {
            Some(value) => value,
            None => return Err(VoteError::NoSuchProposal),
        };
        //check the caller is the owner
        if ic_cdk::caller() != old_proposal.owner {
            return Err(VoteError::AccessRejected);
        }
        let value = Proposal {
            description: proposal.description,
            approve: old_proposal.approve,
            reject: old_proposal.reject,
            pass: old_proposal.pass,
            is_active: proposal.is_active,
            voted: old_proposal.voted,
            owner: ic_cdk::caller(),
        };
        let res = p.borrow_mut().insert(key, value);

        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}

#[ic_cdk::update]

fn end_proposal(key: u64) -> Result<(), VoteError> {
    //Save the proposal with map.with
    PROPOSAL_MAP.with(|p| {
        //retrieve a proposal with a given key
        let proposal_opt = p.borrow().get(&key);

        let mut proposal: Proposal = match proposal_opt {
            Some(value) => value,
            None => return Err(VoteError::NoSuchProposal),
        };
        //check the caller is the owner
        if ic_cdk::caller() != proposal.owner {
            return Err(VoteError::AccessRejected);
        }
        proposal.is_active = false;
        let res = p.borrow_mut().insert(key, proposal);

        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}

#[ic_cdk::update]
fn vote(key: u64, choice: Choice) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let proposal_opt = p.borrow().get(&key);
        let mut proposal = match proposal_opt {
            Some(value) => value,
            None => return Err(VoteError::NoSuchProposal),
        };
        let caller = ic_cdk::caller();
        //check if voted to the proposal before
        if proposal.voted.contains(&caller) {
            return Err(VoteError::AlreadyVoted);
            //we cannot vote on in active proposal 
        } else if proposal.is_active == false {
            return Err(VoteError::ProposalIsNotActive);
        }
        match choice {
            //If we have choice Approve
            Choice::Approve => proposal.approve += 1,
            Choice::Reject => proposal.reject += 1,
            Choice::Pass => proposal.pass += 1,
        };
        //push it to the vector
        proposal.voted.push(caller);
        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => return Err(VoteError::UpdateError),
        }
    })
}

//the code can convert Rust data structures to and from the Candid format, which is necessary for interacting with the IC's canisters.
//CandidType: A trait that enables a Rust type to be serialized and deserialized using the Candid format, which is an IDL (Interface Definition Language) used in the Internet Computer ecosystem.
///Decode: A function or trait for deserializing data from the Candid format.
// A trait from Serde (used in Candid) for deserializing data structures.
//Encode: A function or trait for serializing data into the Candid format.
use candid::{CandidType, Decode, Deserialize, Encode};

//MemoryId: An identifier for a block of memory managed by the stable memory manager.
//MemoryManger: A structure that handles the allocation and deallocation of memory blocks.
//VirtualMemory: An abstraction over physical memory that allows for easier memory management.
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};

//BoundedStorable: A trait for types that can be stored in a fixed amount of memory.(etghayaret ba bound ony in another import)
//DefaultMemoryImpl: A default implementation of the memory interface.
//StableBTreeMap: A B-tree map data structure optimized for stable memory storage.
//Storable: A trait for types that can be stored in stable memory.
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

//Cow: Stands for "Copy on Write." It's a smart pointer that allows for efficient borrowing or cloning of data depending on whether it needs to be modified.
//RefCell: A type that provides interior mutability, allowing you to mutate data even when the RefCell itself is immutable. It enforces borrow rules at runtime.
use ic_stable_structures::storable::Bound;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100;

//we are using methodology and the structure in icp to mange the state bec we dont want to lose the state whenever we redeploy our canister, as if you lose it we will need to experiment with the same data again and again

//candidtype hena mohem 3ashan file .did has text not string and so on , they are not corresponding , so the frontend can know it
#[derive(CandidType, Deserialize)]
struct Exam {
    out_of: u8,
    //course name
    course: String,
    //exam
    curve: u8,
}

//we need to impl traits for our struct to work with stable memory
//we need to impl bonded storable and storable traits
//and these traits specify how we can serialize and deserialize data so u can use any fun(type) to serialize and deserialize data
//candid is the type used here

//The Storable trait is part of the ic_stable_structures crate
//By implementing the Storable trait for Exam, instances of Exam can be easily stored in and retrieved from stable memory using the StableBTreeMap or other stable structures provided by the ic_stable_structures crate.
//This implementation allows the Exam type to be serialized into bytes and deserialized from bytes, which is essential for storing and retrieving instances of Exam in stable memory.

impl Storable for Exam {
    //The Storable trait is part of the ic_stable_structures crate
    //It returns a Cow<[u8]>, which stands for "copy-on-write" and is a smart pointer that can point to either borrowed data or owned data.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        //Encode!(self): This macro from the candid crate serializes the Exam instance into a byte vector (Vec<u8>).
        //Cow::Owned(...): Wraps the owned byte vector in a Cow::Owned to return it as Cow<[u8]>.
        Cow::Owned(Encode!(self).unwrap())
    }

    //This method deserializes a byte array ([u8]) back into an Exam instance.
    //It takes a Cow<[u8]> as input, which can be either borrowed or owned data.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        //bytes.as_ref(): Converts the Cow<[u8]> to a borrowed slice (&[u8]), whether it is owned or borrowed.
        //Decode!(..., Self): This macro from the candid crate deserializes the byte slice back into an Exam instance.
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    //New part
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

//who we can write this data to our local memory

thread_local! {
    //MEMORY MANGER that manges our memory
    //that is virtually multipling this memoru locations insead one loc in threadlocal , i am having virtually 256 locations so I can use multiple different structs and store them (like If we have more than one struct)
    //we here use only one structure but memory manger will split them
    //I can name it sth else
    //hold data with ref cell and initalize new mem manger
    static MEMORY_MANAGER : RefCell<MemoryManager<DefaultMemoryImpl>>= RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    //create two different locations
    //exam map will have the exams that we wil have in our smart contract
    //StableBTreeMap : specail structure like the dictionary in python (key , value), instead of having array or vector to avoid use indexes and we can name the key as the indexes if we can
    //key , value , memory el fo2
    static EXAM_MAP :RefCell<StableBTreeMap<u64,Exam,Memory>>= RefCell::new(StableBTreeMap::init(
        //will use memory manger to split locations
        //borrow first mem location
        MEMORY_MANAGER.with(|m|m.borrow().get(MemoryId::new(0))),
    ));

    static PARTICIPATION_PERCENTAGE_MAP :RefCell<StableBTreeMap<u64,u64,Memory>>= RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m|m.borrow().get(MemoryId::new(1))),
    ));
}

//The provided Rust function get_participation is designed to retrieve a value from a thread-local storage map (PARTICIPATION_PERCENTAGE_MAP) using a key of type u64.
//This function is marked as an Internet Computer (IC) query function, meaning it does not alter the state and can be called without consuming cycles.
//get participation percentage
#[ic_cdk::query]
fn get_participation(key: u64) -> Option<u64> {
    //with :This will lazily initialize the value if this thread has not referenced this key yet.
    //this key to retrive some data from our participation

    //If the thread has not referenced this key before, with will lazily initialize the value.
    //wroking with participation..Map that's why i'm using with , p:indecates working with PPM what is init
    //p:value
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_exam(key: u64) -> Option<Exam> {
    //.borrow : is borrowing value inside ref cell
    EXAM_MAP.with(|p| p.borrow().get(&key))
}
//after we update here we return last value
//using options as we will get old values from fns, and here will alter the data
#[ic_cdk::update]
fn insert_exam(key: u64, value: Exam) -> Option<Exam> {
    //borrow mut or not from ref cell , but we will borrow mut as we want to alter the data
    EXAM_MAP.with(|p| p.borrow_mut().insert(key, value))
}

#[ic_cdk::update]
fn insert_participation(key: u64, value: u64) -> Option<u64> {
    //borrow mut or not from ref cell
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow_mut().insert(key, value))
}
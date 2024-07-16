use std::cell::RefCell;
use candid::types::number::Nat;

thread_local!{
    //RefCell allow mutability 
static COUNTER: RefCell<Nat>=RefCell::new(Nat::from(0u32));
}

/// Get er.
#[ic_cdk_macros::query] // Query methods can read state but cannot modify it,
fn get()->Nat{ //return natural value
    //The with method provided by thread_local! allows you to access the thread-local variable COUNTER
COUNTER.with(|counter| (*counter.borrow()).clone())
//counter.borrow(): This borrows the RefCell contents immutably.
}
//set the value of the counter
#[ic_cdk_macros::update]
fn set(n:Nat){
    COUNTER.with(|count| *count.borrow_mut()=n);
}

#[ic_cdk_macros::update]
fn increment(){
    COUNTER.with(|counter| *counter.borrow_mut() += Nat::from(5u32));
}

// Enable Candid export
//ic_cdk::export_candid!();
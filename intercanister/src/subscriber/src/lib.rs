use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::Deserialize;
use std::cell::Cell;


thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}


#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

//provides functionality for the publisher canister to subscribe to topics within the subscriber canister. This function is called by the publisher canister.
#[update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let _call_result: Result<(), _> =
        ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
}

//updates the counter record for each published value in a topic within the subscriber canister.
#[update]
fn update_count(counter: Counter) {
    COUNTER.with(|c| {
        c.set(c.get() + counter.value);
    });
}


//allows the Counter value to be queried and returned in a call.
#[query]
fn get_count() -> u64 {
    COUNTER.with(|c| {
        c.get()
    })
}

use candid::{CandidType, Principal};
use ic_cdk::update;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::BTreeMap;
// Define a type alias for a BTreeMap that maps a Principal to a Subscriber
type SubscriberStore = BTreeMap<Principal, Subscriber>;

//to store subscribers 
thread_local! {
    static SUBSCRIBERS: RefCell<SubscriberStore> = RefCell::default();
}
// Define the Counter struct with two fields: topic and value
#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}
// Define the Subscriber struct with one field: topic
#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}


#[update]
//subscribe allows for the publisher canister to make a call to the subscriber canister and subscribe to topics.
fn subscribe(subscriber: Subscriber) {
        // Get the principal ID of the caller
    let subscriber_principal_id = ic_cdk::caller();
    // Insert the subscriber into the SUBSCRIBERS store
    SUBSCRIBERS.with(|subscribers| {
        subscribers
            .borrow_mut()
            .insert(subscriber_principal_id, subscriber)
    });
}
// Define an async function that allows the publisher canister to publish information into a topic in the subscribers canister.

#[update]
async fn publish(counter: Counter) {
        // Iterate over the subscribers and notify those subscribed to the topic
    SUBSCRIBERS.with(|subscribers| {
        for (k, v) in subscribers.borrow().iter() {
            if v.topic == counter.topic {
                    // Notify the subscriber by calling their "update_count" method with the counter
                let _call_result: Result<(), _> =
                    ic_cdk::notify(*k, "update_count", (&counter,));
            }
        }
    });    
}

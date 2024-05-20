use ic_cdk_macros::{query, update};
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static VOTES: RefCell<HashMap<String, u64>> = RefCell::new({
        let mut m = HashMap::new();
        m.insert("Motoko".to_string(),0);
        m.insert("Python".to_string(),0);
        m.insert("Rust".to_string(),0);
        m.insert("TypeScript".to_string(),0);
        m
    });
}

#[query]
fn get_question() -> String {
    "What is your favorite programming language?".to_string()
}

#[query]
fn get_votes() -> String {
    VOTES.with(|votes| {
        let votes = votes.borrow();
        json!(votes
            .iter()
            .map(|(k, v)| vec![k.clone(), v.to_string()])
            .collect::<Vec<_>>())
        .to_string()
    })
}

#[update]
fn vote(entry: String) -> String {
    VOTES.with(|votes| {
        let mut votes = votes.borrow_mut();
        let count = votes.entry(entry).or_insert(0);
        *count += 1;
        json!(votes
            .iter()
            .map(|(k, v)| vec![k.clone(), v.to_string()])
            .collect::<Vec<_>>())
        .to_string()
    })
}

#[update]
fn reset_votes() -> String {
    VOTES.with(|votes| {
        let mut votes = votes.borrow_mut();
        for key in votes.keys().cloned().collect::<Vec<String>>() {
            votes.insert(key, 0);
        }
        json!(votes
            .iter()
            .map(|(k, v)| vec![k.clone(), v.to_string()])
            .collect::<Vec<_>>())
        .to_string()
    })
}

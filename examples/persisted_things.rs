use std::env;
use serde::{de::DeserializeOwned, Serialize};
use atomic_store::{
    append_log::AppendLog,
    atomic_store::{AtomicStore, AtomicStoreLoader},
    rolling_log::RollingLog,
    error::PersistenceError,
};

#[derive(Debug, Serialize, Deserialize)]
struct ThingA {
    a1: i32,
    a2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThingB {
    b1: i32,
    b2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThingC {
    c1: i32,
    c2: i32,
}

fn main() -> Result<(), PersistenceError> {
    let a_list: Vec<ThingA>;
    let b_list: Vec<ThingB>;
    let c: ThingC;

    let persisted_state: AtomicStore;
    let persisted_a: AppendLog<ThingA>;
    let persisted_b: AppendLog<ThingB>;
    let persisted_c: RollingLog<ThingC>;

    let store_loader = AtomicStoreLoader::new(env::current_dir()?.as_path(), "persisted_things_store");
}



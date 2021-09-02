use atomic_store::{
    append_log::AppendLog,
    atomic_store::{AtomicStore, AtomicStoreLoader},
    error::PersistenceError,
    rolling_log::RollingLog,
};
use serde::{Deserialize, Serialize};

use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThingA {
    a1: i64,
    a2: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThingB {
    b1: i64,
    b2: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThingC {
    c1: i64,
    c2: i64,
}

fn single_threaded_create_and_populate() -> Result<(), PersistenceError> {
    let mut test_path =
        env::current_dir().map_err(|e| PersistenceError::StdIoDirOpsError { source: e })?;
    test_path.push("testing_tmp");

    let mut store_loader =
        AtomicStoreLoader::create_new(test_path.as_path(), "persisted_things_store")?;
    let persisted_a = AppendLog::<ThingA>::create_new(&mut store_loader, "a_store", 1024)?;
    let persisted_b = AppendLog::<ThingB>::create_new(&mut store_loader, "b_store", 16)?;
    let persisted_c = RollingLog::<ThingC>::create_new(&mut store_loader, "c_store", 16)?;

    let mut atomic_store = AtomicStore::load_store(store_loader);

    Ok(())
}

fn single_threaded_load_from_files() -> Result<(), PersistenceError> {
    let mut test_path =
        env::current_dir().map_err(|e| PersistenceError::StdIoDirOpsError { source: e })?;
    test_path.push("testing_tmp");

    let mut store_loader =
        AtomicStoreLoader::load_from_stored(test_path.as_path(), "persisted_things_store")?;
    let persisted_a = AppendLog::<ThingA>::open(&mut store_loader, "a_store", 1024);
    let persisted_b = AppendLog::<ThingB>::open(&mut store_loader, "b_store", 16);
    let persisted_c = RollingLog::<ThingC>::open(&mut store_loader, "c_store", 16);

    let mut atomic_store = AtomicStore::load_store(store_loader);
    Ok(())
}

fn main() -> Result<(), PersistenceError> {
    single_threaded_create_and_populate()?;
    single_threaded_load_from_files()?;
    Ok(())
}

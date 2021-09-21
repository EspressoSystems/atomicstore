use array_init;
use atomic_store::{
    append_log::AppendLog,
    atomic_store::{AtomicStore, AtomicStoreLoader},
    error::PersistenceError,
    load_store::BincodeLoadStore,
    rolling_log::RollingLog,
    Result,
};
use serde::{Deserialize, Serialize};

use std::env;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThingA {
    a1: i64,
    a2: i64,
}
impl fmt::Display for ThingA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A{{{},{}}}", self.a1, self.a2)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThingB {
    b1: i64,
    b2: i64,
}
impl fmt::Display for ThingB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B{{{},{}}}", self.b1, self.b2)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThingC {
    c1: i64,
    c2: i64,
}
impl fmt::Display for ThingC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "C{{{},{}}}", self.c1, self.c2)
    }
}

#[test]
fn create_populate_drop_load() -> Result<()> {
    single_threaded_create_and_populate()?;
    single_threaded_load_from_files()?;
    Ok(())
}

fn single_threaded_create_and_populate() -> Result<()> {
    let mut test_path =
        env::current_dir().map_err(|e| PersistenceError::StdIoDirOpsError { source: e })?;
    test_path.push("testing_tmp");

    let mut store_loader =
        AtomicStoreLoader::create(test_path.as_path(), "persisted_things_store")?;
    let persisted_a =
        AppendLog::<BincodeLoadStore<ThingA>>::create(&mut store_loader, "a_store", 1024)?;
    let persisted_b =
        AppendLog::<BincodeLoadStore<ThingB>>::create(&mut store_loader, "b_store", 16)?;
    let persisted_c =
        RollingLog::<BincodeLoadStore<ThingC>>::create(&mut store_loader, "c_store", 16)?;

    let mut atomic_store = AtomicStore::open(store_loader)?;

    // enough to flow to a second file
    let first_array_of_a: [ThingA; 100] = array_init::array_init(|i| ThingA {
        a1: i as i64,
        a2: i as i64 * 2,
    });
    println!("Persisting resource A, {} elements", first_array_of_a.len());
    // write scope
    {
        let mut a_writer = persisted_a.write()?;

        let first_a_locations: Vec<_> = first_array_of_a
            .iter()
            .map(|a| a_writer.store_resource(&a).unwrap())
            .collect();
        println!(
            "Stored in files a_store_*, at locations {:?}",
            first_a_locations
        );
        a_writer.commit_version()?;
    }

    let first_array_of_b: [ThingB; 10] = array_init::array_init(|i| ThingB {
        b1: i as i64 * 7,
        b2: 30 - i as i64,
    });
    println!("Persisting resource B, {} elements", first_array_of_b.len());
    // write scope
    {
        let mut b_writer = persisted_b.write()?;
        let first_b_locations: Vec<_> = first_array_of_b
            .iter()
            .map(|b| b_writer.store_resource(&b).unwrap())
            .collect();

        println!(
            "Stored in files b_store_*, at locations {:?}",
            first_b_locations
        );
        b_writer.commit_version()?;
    }

    let first_c = ThingC { c1: 42, c2: 2021 };
    println!("Persisting resource C, 1 element");
    // write scope
    {
        let mut c_writer = persisted_c.write()?;
        let first_c_location = c_writer.store_resource(&first_c).unwrap();
        println!("Stored in file b_store_*, at location {}", first_c_location);
        c_writer.commit_version();
    }

    println!("Committing resources");
    // will block until all persisted stores call either commit_version() or skip_version()
    atomic_store.commit_version()?;

    println!("Resources committed");

    let second_array_of_a: [ThingA; 100] = array_init::array_init(|i| ThingA {
        a1: i as i64 + 100,
        a2: i as i64 * 2,
    });
    {
        let mut a_writer = persisted_a.write()?;

        let _locations: Vec<_> = second_array_of_a
            .iter()
            .map(|a| a_writer.store_resource(&a).unwrap())
            .collect();
        a_writer.commit_version()?;
    }
    let second_array_of_b: [ThingB; 10] = array_init::array_init(|i| ThingB {
        b1: i as i64 * 5,
        b2: 40 - i as i64,
    });
    {
        let mut b_writer = persisted_b.write()?;

        let _locations: Vec<_> = second_array_of_b
            .iter()
            .map(|b| b_writer.store_resource(&b).unwrap())
            .collect();
        b_writer.commit_version()?;
    }
    let second_c = ThingC { c1: 99, c2: 1492 };
    {
        let mut c_writer = persisted_c.write()?;
        c_writer.store_resource(&second_c).unwrap();
        c_writer.commit_version();
    }

    atomic_store.commit_version()?;

    let third_array_of_a: [ThingA; 100] = array_init::array_init(|i| ThingA {
        a1: i as i64 + 200,
        a2: i as i64 * 2,
    });
    {
        let mut a_writer = persisted_a.write()?;

        let _locations: Vec<_> = third_array_of_a
            .iter()
            .map(|a| a_writer.store_resource(&a).unwrap())
            .collect();
        a_writer.commit_version()?;
    }

    let third_array_of_b: [ThingB; 10] = array_init::array_init(|i| ThingB {
        b1: i as i64 * 3,
        b2: 50 - i as i64,
    });
    {
        let mut b_writer = persisted_b.write()?;

        let _locations: Vec<_> = third_array_of_b
            .iter()
            .map(|b| b_writer.store_resource(&b).unwrap())
            .collect();
        // don't commit this one.
    }

    let third_c = ThingC { c1: 17, c2: 113 };
    {
        let mut c_writer = persisted_c.write()?;
        c_writer.store_resource(&third_c).unwrap();
        c_writer.commit_version();
    }

    // drop round three on the floor

    Ok(())
}

fn single_threaded_load_from_files() -> Result<()> {
    let mut test_path =
        env::current_dir().map_err(|e| PersistenceError::StdIoDirOpsError { source: e })?;
    test_path.push("testing_tmp");

    let mut store_loader = AtomicStoreLoader::load(test_path.as_path(), "persisted_things_store")?;
    let persisted_a =
        AppendLog::<BincodeLoadStore<ThingA>>::load(&mut store_loader, "a_store", 1024)?;
    let persisted_b =
        AppendLog::<BincodeLoadStore<ThingB>>::load(&mut store_loader, "b_store", 16)?;
    let persisted_c =
        RollingLog::<BincodeLoadStore<ThingC>>::load(&mut store_loader, "c_store", 16)?;

    let mut atomic_store = AtomicStore::open(store_loader)?;

    let third_array_of_a: [ThingA; 100] = array_init::array_init(|i| ThingA {
        a1: i as i64 + 200,
        a2: i as i64 * 2,
    });
    {
        let mut a_writer = persisted_a.write()?;

        let _locations: Vec<_> = third_array_of_a
            .iter()
            .map(|a| a_writer.store_resource(&a).unwrap())
            .collect();
        a_writer.commit_version()?;
    }

    let third_array_of_b: [ThingB; 10] = array_init::array_init(|i| ThingB {
        b1: i as i64 * 3,
        b2: 50 - i as i64,
    });
    {
        let mut b_writer = persisted_b.write()?;

        let _locations: Vec<_> = third_array_of_b
            .iter()
            .map(|b| b_writer.store_resource(&b).unwrap())
            .collect();
        b_writer.commit_version()?;
    }

    let third_c = ThingC { c1: 17, c2: 113 };
    {
        let mut c_writer = persisted_c.write()?;
        c_writer.store_resource(&third_c).unwrap();
        c_writer.commit_version();
    }

    atomic_store.commit_version()?;

    Ok(())
}

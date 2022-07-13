#![allow(dead_code)]

use crate::load_store::BincodeLoadStore;
use crate::storage_location::StorageLocation;
use crate::AppendLog;
use crate::AtomicStore;
use crate::AtomicStoreLoader;
use crate::PersistenceError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;

struct KVStore<K: DeserializeOwned + Serialize + Eq + Hash + Clone, V: DeserializeOwned + Serialize>
{
    atomic_store: AtomicStore,
    // Add todo hardcode to BincodeLoadStore, challenge is that serialization may not be available from key and val
    kv_snapshots: AppendLog<BincodeLoadStore<(K, Option<V>)>>,
    pending_kv_lookup: HashMap<K, StorageLocation>,
    committed_kv_lookup: HashMap<K, StorageLocation>,
}

impl<
        K: DeserializeOwned + Serialize + Eq + Hash + Clone + Debug,
        V: DeserializeOwned + Serialize + Debug,
    > KVStore<K, V>
{
    pub fn new(store_path: &Path, key_tag: &str) -> Result<KVStore<K, V>, PersistenceError> {
        let mut loader = AtomicStoreLoader::load(store_path, key_tag)?;
        let kv_tag = format!("{}_type_y", key_tag);
        let kv_snapshots = AppendLog::load(&mut loader, Default::default(), &kv_tag, 1024)?;
        let mut committed_kv_lookup = HashMap::new();
        let atomic_store = AtomicStore::open(loader)?;
        for location in kv_snapshots.iter().to_indexed_iter() {
            let location = location.unwrap();
            let (key, _) = kv_snapshots.load_specified(&location)?;
            committed_kv_lookup.insert(key, location);
        }
        Ok(KVStore {
            atomic_store,
            kv_snapshots,
            committed_kv_lookup,
            pending_kv_lookup: HashMap::new(),
        })
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.update_value(k, Some(v))
    }

    pub fn remove(&mut self, k: K) -> Option<V> {
        self.update_value(k, None)
    }

    pub fn get<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let location = self.committed_kv_lookup.get(k);
        if let Some(loc) = location {
            self.kv_snapshots.load_specified(loc).unwrap().1
        } else {
            None
        }
    }

    fn update_value(&mut self, k: K, v: Option<V>) -> Option<V> {
        let committed_value = self.get(&k);
        let kv_resource = (k.clone(), v);
        let location = self.kv_snapshots.store_resource(&kv_resource).unwrap();
        self.pending_kv_lookup.insert(k, location);
        committed_value
    }

    pub fn commit(&mut self) {
        self.kv_snapshots.commit_version().unwrap();
        self.atomic_store.commit_version().unwrap();
        self.committed_kv_lookup
            .extend(self.pending_kv_lookup.clone());
        self.pending_kv_lookup.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn smoke_test() {
        let key_tag = "test_key";
        let dir: TempDir = tempfile::Builder::new().tempdir().unwrap();
        let mut store: KVStore<u64, u64> = KVStore::new(dir.path(), key_tag).unwrap();
        store.insert(1, 1);
        store.commit();
        assert!(store.get(&1).unwrap() == 1);
        store.remove(1);
        store.commit();
        assert!(store.get(&1) == None);
    }
}

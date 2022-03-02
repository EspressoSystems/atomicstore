# AtomicStore: synchronized persistence library

Provides a per-entity coordinated persistence index, `AtomicStore`, that tracks multiple resource
persistence handlers (which use a `VersionSyncHandle` to coordinate) and records a entity-wide atomic
recovery point each time the global state is committed.

This insures that recovery after a failure will always reflect a consistent logical point in time
for the entire entity.

# Usage

Each logical component with a persistable state must initialize an instance of AtomicStore, and a store for each element of its state that can be updated independently.
There are two fields used to define the domain of the logical component: a `storage_path: &Path`, and a `component_tag: &str`. Note that the first field contains an assumption of storage in a file system; this may change in the future.

At the time of logical component initialization, a temporary `AtomicStoreLoader` must be used to load the prior state indexes, or clear them if restoring the initial global state. This must then be used to initialize each associated stateful element. Once all elements are initialized, the global `AtomicStore` instance can be initialized, and should be kept in scope until the logical component terminates.

Each element that is persisted should specify its stateful representation in terms of one or more log types. The atomic_store crate currently provides three types of log: `AppendLog`, `FixedAppendLog`, and `RollingLog`, but it will work with custom logs as well.

`AppendLog` provides an iterable append log, with random access support. The entire history can be loaded with an iterator, or a specific index can be loaded. `FixedAppendLog` is a more efficient version of the same concept where the serialization of the type being stored is always a consistent size. `RollingLog` only keeps a fixed number of the persisted element, and is suitable for snapshots or transient fields.

Each time the state of a element has meaningfully changed, it can persist this change with its log representation, using `log.store_resource(value);`, and when the element's changes are ready for inclusion in the global state, it can syncronize it to the logical compenent state using `log.commit_version();`. The logical component state can then update the persisted state with `atomic_store.commit_version();`, and this will guarantee an atomically consistent persisted state.

If all stateful data can be accessed in the same place, this can be simplified with the following pattern:

```rust
struct ComponentPersistence {
    atomic_store: AtomicStore,
    type_x_array_history: AppendLog<BincodeLoadStore<TypeX>>,
    type_y_snapshots: RollingLog<BincodeLoadStore<TypeY>>,
}

impl ComponentPersistence {
    pub fn new((store_path: &Path, key_tag: &str) -> Result<ComponentPersistence, PersistenceError> {
        let mut loader = AtomicStoreLoader::load(&transform_path(store_path), key_tag)?;
        let type_x_tag = format!("{}_type_x", key_tag);
        let type_y_tag = format!("{}_type_y", key_tag);
        let type_x_array_history = AppendLog::load(&mut loader, Default::default(), &type_x_tag, 1024)?;
        let type_y_snapshots = RollingLog::load(&mut loader, Default::default(), &type_y_tag, 1024)?;
        let atomic_store = AtomicStore::open(loader)?;
        Ok(ComponentPersistence {
            atomic_store,
            type_x_array_history,
            type_y_snapshots,
        })
    }
    pub fn store_x((&mut self, x_element: &TypeX) {
        self.type_x_array_history.store_resource(x_element).unwrap();
    }

    pub fn store_y((&mut self, y_element: &TypeY) {
        self.type_y_snapshots.store_resource(y_element).unwrap();
    }

    pub fn fn commit(&mut self) {
        self.type_x_array_history.commit_version().unwrap();
        self.type_y_snapshots.commit_version().unwrap();
        self.atomic_store.commit_version().unwrap();
    }

    pub fn x_iter(&self) -> Iter<BincodeLoadStore<TypeX>> {
        self.type_x_array_history.iter()
    }

    pub fn last_y(&self) -> Option<TypeY> {
        self.type_y_snapshots.load_latest().ok()
    }
}

```


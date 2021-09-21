# AtomicStore: synchronized persistence library

Provides a per-entity coordinated persistence index, `AtomicStore`, that tracks multiple resource
persistence handlers (which use a `VersionSyncHandle` to coordinate) and records a entity-wide atomic
recovery point each time the global state is committed.

This insures that recovery after a failure will always reflect a consistent logical point in time
for the entire entity.

# Usage


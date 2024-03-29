// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

#![deny(warnings)]
#![allow(dead_code)]

use crate::{
    append_log::AppendLog,
    atomic_store::{AtomicStore, AtomicStoreLoader},
    load_store::BincodeLoadStore,
    rolling_log::RollingLog,
    storage_location::StorageLocation,
    Result,
};
use core::iter::once;
use rand::Rng;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use tempfile::TempDir;

fn poisson_uni<R: Rng>(prng: &mut R, lambda: u16) -> u16 {
    let lambda = lambda as f64;
    let lambda = if lambda >= 1e-6 { lambda } else { 1e-6 };

    // fake the poisson distribution with a sum-of-uniform for speed

    let n = 100;
    let mut tot = 0f64;
    for _ in 0..n {
        tot += prng.gen_range(0.0..1.0) / (n as f64);
    }

    tot *= lambda / 2.0;

    (tot as i16) as u16
}

#[derive(Clone, Debug)]
enum SizeDistribution {
    Constant(u16),
    UniformRange(u16, u16),
    Poisson { rate: u16 },
}

impl SizeDistribution {
    fn sample<R: Rng>(&self, prng: &mut R) -> u16 {
        match self {
            SizeDistribution::Constant(c) => *c,
            SizeDistribution::UniformRange(lo, hi) => {
                let (lo, hi) = (*core::cmp::min(lo, hi), *core::cmp::max(lo, hi));
                let (lo, hi) = if lo != hi {
                    (lo, hi)
                } else if lo == 0 {
                    (0, 1)
                } else {
                    (lo - 1, hi)
                };

                prng.gen_range(lo..hi)
            }
            SizeDistribution::Poisson { rate } => poisson_uni(prng, *rate),
        }
    }
}

impl quickcheck::Arbitrary for SizeDistribution {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut logdist_sample = || {
            let ret: f32 = <_>::arbitrary(g);
            (ret.abs() + 1e-8).log2() as i16 as u16
        };
        let options = [
            Self::Constant(logdist_sample()),
            Self::UniformRange(logdist_sample(), logdist_sample()),
            Self::Poisson {
                rate: logdist_sample(),
            },
        ];
        g.choose(&options).unwrap().clone()
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        match self {
            Self::Constant(c) => Box::new(c.shrink().map(Self::Constant)),

            Self::UniformRange(lo, hi) => {
                let (lo, hi) = (*core::cmp::min(lo, hi), *core::cmp::max(lo, hi));
                if lo == hi {
                    Box::new(once(Self::Constant(lo)))
                } else {
                    Box::new(
                        (hi - lo)
                            .shrink()
                            .map(move |diff| Self::UniformRange(lo, lo + diff))
                            .chain(lo.shrink().map(move |lo| Self::UniformRange(lo, hi)))
                            .chain(hi.shrink().map(move |hi| Self::UniformRange(lo, hi))),
                    )
                }
            }

            Self::Poisson { rate } => Box::new(
                rate.shrink()
                    .map(|rate| Self::Poisson { rate })
                    .chain(once(Self::Constant(*rate)))
                    .chain(once(Self::UniformRange(0, rate << 1))),
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum DataDistribution {
    // RandomNoise,
    Byte(u8, Vec<u8>),
    U16(u16, Vec<u16>),
    U32(u32, Vec<u32>),
    U64(u64, Vec<u64>),
}

impl DataDistribution {
    fn sample<R: Rng>(&self, _prng: &mut R, buf: &mut [u8]) {
        match self {
            // Self::RandomNoise => {
            // prng.fill(buf);
            // },
            Self::Byte(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = all_vs[i % all_vs.len()];
                }
            }
            Self::U16(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    let ix = i / 2;
                    let shift = 8 * (i % 2);
                    *b = ((all_vs[ix % all_vs.len()] >> shift) & 0xff) as u8;
                }
            }
            Self::U32(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    let ix = i / 4;
                    let shift = 8 * (i % 4);
                    *b = ((all_vs[ix % all_vs.len()] >> shift) & 0xff) as u8;
                }
            }
            Self::U64(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    let ix = i / 8;
                    let shift = 8 * (i % 8);
                    *b = ((all_vs[ix % all_vs.len()] >> shift) & 0xff) as u8;
                }
            }
        }
    }
}

impl quickcheck::Arbitrary for DataDistribution {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let options = [
            // Self::RandomNoise,
            Self::Byte(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::U16(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::U32(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::U64(<_>::arbitrary(g), <_>::arbitrary(g)),
        ];
        g.choose(&options).unwrap().clone()
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        match self {
            // Self::RandomNoise => Box::new(std::iter::empty()),
            Self::Byte(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::Byte(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::Byte(v, vs.clone()))),
                )
            }

            Self::U16(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::U16(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::U16(v, vs.clone()))),
                )
            }

            Self::U32(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::U32(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::U32(v, vs.clone()))),
                )
            }

            Self::U64(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::U64(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::U64(v, vs.clone()))),
                )
            }
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum StorageType {
    Append,
    Rolling,
}

impl quickcheck::Arbitrary for StorageType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&[Self::Rolling, Self::Append]).unwrap()
    }
}

#[derive(Clone, Debug)]
struct LogDescription {
    file_fill_size: u16,
    log_type: StorageType,

    // How many things get stored per "tick"
    data_rate: SizeDistribution,

    // How big the data should be
    size_dist: SizeDistribution,
    // How the contents should be derived
    data_dist: DataDistribution,
}

impl quickcheck::Arbitrary for LogDescription {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // TODO: we should fix `file_fill_size == 0`
        let file_fill_size = core::cmp::max(1, <_>::arbitrary(g));
        // let file_fill_size = <_>::arbitrary(g);
        let log_type = <_>::arbitrary(g);
        let data_rate = <_>::arbitrary(g);
        let size_dist = <_>::arbitrary(g);
        let data_dist = <_>::arbitrary(g);
        Self {
            file_fill_size,
            log_type,
            data_rate,
            size_dist,
            data_dist,
        }
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        Box::new(
            self.file_fill_size
                .shrink()
                .filter(|x| *x >= 1)
                .map({
                    let ret = self.clone();
                    move |file_fill_size| {
                        let mut ret = ret.clone();
                        ret.file_fill_size = file_fill_size;
                        ret
                    }
                })
                .chain(self.log_type.shrink().map({
                    let ret = self.clone();
                    move |log_type| {
                        let mut ret = ret.clone();
                        ret.log_type = log_type;
                        ret
                    }
                }))
                .chain(self.data_rate.shrink().map({
                    let ret = self.clone();
                    move |data_rate| {
                        let mut ret = ret.clone();
                        ret.data_rate = data_rate;
                        ret
                    }
                }))
                .chain(self.size_dist.shrink().map({
                    let ret = self.clone();
                    move |size_dist| {
                        let mut ret = ret.clone();
                        ret.size_dist = size_dist;
                        ret
                    }
                }))
                .chain(self.data_dist.shrink().map({
                    let ret = self.clone();
                    move |data_dist| {
                        let mut ret = ret.clone();
                        ret.data_dist = data_dist;
                        ret
                    }
                })),
        )
    }
}

#[derive(Clone, Debug)]
struct StoreDescription {
    logs: Vec<(String, LogDescription)>,
}

impl quickcheck::Arbitrary for StoreDescription {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut logs = <Vec<_>>::arbitrary(g);

        // Quickcheck sometimes generates very long vectors. Limit the length to keep the test
        // runtime reasonable.
        logs.truncate(4);

        // Ensure there is at least one log, otherwise the test is useless.
        logs.push(<_>::arbitrary(g));

        Self { logs }
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        Box::new(self.logs.shrink().map(|logs| Self { logs }))
    }
}

#[derive(Clone, Debug)]
enum StorageAction {
    Commit,
    Revert {
        which_log: u16,
    },
    Reconstruct,
    // Write data to each log based on their descriptions
    WriteData {
        seed: u64,
        max_num: u16,
        max_buf_size: u16,
    },
    LoadLatest,

    // TODO: iterator API coverage
    // indexed back in time
    ReadData {
        i1: u32,
        i2: u32,
    },
    // TODO: metadata corruption?
}

impl quickcheck::Arbitrary for StorageAction {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        use StorageAction::*;
        let logdist_sample = |g: &mut quickcheck::Gen| {
            let ret: f32 = <_>::arbitrary(g);
            (ret.abs() + 1e-8).log2() as i16 as u16
        };

        let options = [
            Commit,
            Revert {
                which_log: <_>::arbitrary(g),
            },
            Reconstruct,
            WriteData {
                seed: <_>::arbitrary(g),
                max_num: logdist_sample(g),
                max_buf_size: logdist_sample(g),
            },
            LoadLatest,
            ReadData {
                i1: <_>::arbitrary(g),
                i2: <_>::arbitrary(g),
            },
        ];
        g.choose(&options).unwrap().clone()
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        use StorageAction::*;

        match self {
            Revert { which_log } => {
                Box::new(which_log.shrink().map(|which_log| Revert { which_log }))
                    as Box<dyn Iterator<Item = _> + 'static>
            }
            WriteData {
                seed,
                max_num,
                max_buf_size,
            } => {
                let seed = *seed;
                let max_num = *max_num;
                let max_buf_size = *max_buf_size;
                Box::new((max_num, max_buf_size, seed).shrink().map(
                    move |(max_num, max_buf_size, seed)| WriteData {
                        seed,
                        max_num,
                        max_buf_size,
                    },
                )) as Box<dyn Iterator<Item = _> + 'static>
            }
            ReadData { i1, i2 } => {
                Box::new((*i1, *i2).shrink().map(|(i1, i2)| ReadData { i1, i2 }))
                    as Box<dyn Iterator<Item = _> + 'static>
            }
            _ => Box::new(core::iter::empty()),
        }
    }
}

#[derive(Debug)]
enum Log {
    Append(AppendLog<BincodeLoadStore<Vec<u8>>>),
    Rolling(RollingLog<BincodeLoadStore<Vec<u8>>>),
}

impl Log {
    fn commit_version(&mut self) -> Result<()> {
        match self {
            Log::Append(append) => append.commit_version(),
            Log::Rolling(rolling) => rolling.commit_version(),
        }
    }

    #[allow(clippy::ptr_arg)]
    fn store_resource(&mut self, val: &Vec<u8>) -> Result<StorageLocation> {
        match self {
            Log::Append(append) => append.store_resource(val),
            Log::Rolling(rolling) => rolling.store_resource(val),
        }
    }

    fn load_latest(&self) -> Result<Vec<u8>> {
        match self {
            Log::Append(append) => append.load_latest(),
            Log::Rolling(rolling) => rolling.load_latest(),
        }
    }

    fn load_specified(&self, loc: &StorageLocation) -> Result<Vec<u8>> {
        match self {
            Log::Append(append) => append.load_specified(loc),
            Log::Rolling(rolling) => rolling.load_specified(loc),
        }
    }

    fn skip_version(&mut self) -> Result<()> {
        match self {
            Log::Append(append) => append.skip_version(),
            Log::Rolling(rolling) => rolling.skip_version(),
        }
    }

    fn revert_version(&mut self) -> Result<()> {
        match self {
            Log::Append(append) => append.revert_version(),
            Log::Rolling(rolling) => rolling.revert_version(),
        }
    }
}

struct IdealLog {
    orig_name: String,
    name: String,
    desc: LogDescription,
    log: Log,
    // TODO: vecdeque?
    stored_items: Vec<(StorageLocation, Vec<u8>)>,
    num_pending_items: usize,
}

struct StorageRunner {
    store: AtomicStore,
    logs: Vec<IdealLog>,
    directory: TempDir,
}

impl StorageRunner {
    fn new_with_path(desc: StoreDescription, directory: TempDir) -> Result<Self> {
        let test_path = directory.path();
        let mut store_loader = AtomicStoreLoader::load(test_path, "storage_runner_store").unwrap();

        let logs = desc
            .logs
            .into_iter()
            .enumerate()
            .map(|(i, (name, log_desc))| -> Result<_> {
                let orig_name = name.clone();
                let name = format!("{}_{}", i, name).escape_default().to_string();
                let name = name.replace('/', "_");
                let name = name[..core::cmp::min(10, name.len())].to_string();
                let log = match log_desc.log_type {
                    StorageType::Append => Log::Append(
                        AppendLog::load(
                            &mut store_loader,
                            <BincodeLoadStore<Vec<u8>>>::default(),
                            &name,
                            log_desc.file_fill_size as u64,
                        )
                        .unwrap(),
                    ),
                    StorageType::Rolling => Log::Rolling(
                        RollingLog::load(
                            &mut store_loader,
                            <BincodeLoadStore<Vec<u8>>>::default(),
                            &name,
                            log_desc.file_fill_size as u64,
                        )
                        .unwrap(),
                    ),
                };

                Ok(IdealLog {
                    orig_name,
                    name,
                    desc: log_desc,
                    log,
                    stored_items: vec![],
                    num_pending_items: 0,
                })
            })
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let store = AtomicStore::open(store_loader)?;

        Ok(Self {
            store,
            logs,
            directory,
        })
    }

    fn new(desc: StoreDescription) -> Result<Self> {
        Self::new_with_path(
            desc,
            tempfile::Builder::new()
                .prefix("atomicstore_test")
                .tempdir()
                .unwrap(),
        )
    }

    fn run_action(mut self, action: StorageAction) -> Self {
        use StorageAction::*;
        match action {
            Commit => {
                for log in self.logs.iter_mut() {
                    if log.num_pending_items > 0 {
                        log.log.commit_version().unwrap();
                    } else {
                        log.log.skip_version().unwrap();
                    }
                    log.num_pending_items = 0;
                }
                self.store.commit_version().unwrap();
            }

            Revert { which_log } => {
                if !self.logs.is_empty() {
                    let which_log = which_log as usize;
                    let ix = which_log % self.logs.len();
                    let log = self.logs.get_mut(ix).unwrap();
                    let num_to_trim = log.num_pending_items;
                    log.num_pending_items = 0;
                    for _ in 0..num_to_trim {
                        log.stored_items.pop();
                    }

                    log.log.revert_version().unwrap();
                }
            }

            Reconstruct => {
                let mut log_stored_items = vec![];

                let logs = self
                    .logs
                    .into_iter()
                    .map(|mut lg| {
                        drop(lg.log);
                        for _ in 0..lg.num_pending_items {
                            lg.stored_items.pop();
                        }
                        log_stored_items.push(lg.stored_items);
                        (lg.orig_name, lg.desc)
                    })
                    .collect();

                drop(self.store);

                self = Self::new_with_path(StoreDescription { logs }, self.directory).unwrap();
                for (lg, stored_items) in self.logs.iter_mut().zip(log_stored_items.into_iter()) {
                    lg.stored_items = stored_items;
                }
            }

            WriteData {
                seed,
                max_num,
                max_buf_size,
            } => {
                let mut base_prng = ChaChaRng::seed_from_u64(seed);

                let mut writes_remaining = max_num;

                for log in self.logs.iter_mut() {
                    let mut log_prng = ChaChaRng::from_rng(&mut base_prng).unwrap();
                    // let num_resources = log.desc.data_rate.sample(&mut log_prng) & ((1u32 << 4)-1);
                    let num_resources = log.desc.data_rate.sample(&mut log_prng);
                    let num_resources = core::cmp::min(writes_remaining, num_resources);
                    writes_remaining -= num_resources;
                    for _ in 0..num_resources {
                        let buf_size = (log.desc.size_dist.sample(&mut log_prng)) as usize;
                        let buf_size = core::cmp::min(1 + max_buf_size as usize, buf_size);
                        let buf_size = core::cmp::max(1, buf_size);

                        let mut buf = vec![0; buf_size];
                        log.desc.data_dist.sample(&mut log_prng, &mut buf);

                        let loc = log.log.store_resource(&buf).unwrap();
                        log.stored_items.push((loc, buf));
                        log.num_pending_items += 1;
                    }
                }
            }

            LoadLatest => {
                for log in self.logs.iter_mut() {
                    match log.log.load_latest() {
                        Ok(v) => {
                            assert_eq!(
                                v,
                                log.stored_items
                                    .get(log.stored_items.len() - 1 - log.num_pending_items)
                                    .unwrap()
                                    .1
                            );
                        }
                        Err(crate::PersistenceError::FailedToFindExpectedResource { .. }) => {
                            assert!(log.stored_items.len() <= log.num_pending_items);
                        }
                        Err(e) => {
                            panic!("load_latest error {:?}", e);
                        }
                    }
                }
            }

            ReadData { i1, i2 } => {
                for log in self.logs.iter_mut() {
                    if log.stored_items.is_empty() {
                        continue;
                    }

                    let i1 = i1 as usize;
                    let i2 = i2 as usize;

                    let num_items = log.stored_items.len() - log.num_pending_items;
                    if num_items == 0 {
                        continue;
                    }

                    let i1 = num_items - 1 - (i1 % num_items);
                    let i2 = num_items - 1 - (i2 % num_items);
                    let lo = core::cmp::min(i1, i2);
                    let hi = core::cmp::max(i1, i2);

                    let (lo_loc, lo_val) = log.stored_items.get(lo).unwrap();
                    let (hi_loc, hi_val) = log.stored_items.get(hi).unwrap();

                    let lo_res = log.log.load_specified(lo_loc);
                    let hi_res = log.log.load_specified(hi_loc);

                    match (lo_res, hi_res) {
                        (Ok(lv), Ok(hv)) if &lv == lo_val && &hv == hi_val => {}
                        (
                            Err(crate::PersistenceError::FailedToFindExpectedResource { .. }),
                            Ok(hv),
                        ) if lo < hi && &hv == hi_val => {}
                        e => {
                            panic!(
                                "load_specified {:?}, {:?} failed: {:?}",
                                (lo_loc, lo_val),
                                (hi_loc, hi_val),
                                e
                            );
                        }
                    }
                }
            }
        }

        self
    }
}

fn store_test_scenario(actions: Vec<StorageAction>, desc: StoreDescription) {
    println!(
        "running {} actions, {} logs",
        actions.len(),
        desc.logs.len()
    );
    let mut runner = StorageRunner::new(desc).unwrap();
    for action in actions {
        runner = runner.run_action(action);
    }
}

#[test]
fn store_test_scenario_quickcheck() {
    fn property(mut actions: Vec<StorageAction>, desc: StoreDescription) {
        // Quickcheck sometimes generates very long vectors; limit the length to keep the runtime
        // reasonable.
        actions.truncate(10);
        store_test_scenario(actions, desc);
    }
    quickcheck::QuickCheck::new()
        .tests(10)
        .quickcheck(property as fn(Vec<StorageAction>, StoreDescription))
}

#[test]
fn store_test_scenario_regressions() {
    use DataDistribution::*;
    use SizeDistribution::*;
    use StorageAction::*;
    use StorageType::*;

    // catches .append(true)
    store_test_scenario(
        vec![
            WriteData {
                seed: 0,
                max_num: 1,
                max_buf_size: 0,
            },
            Commit,
            WriteData {
                seed: 0,
                max_num: 1,
                max_buf_size: 0,
            },
            Commit,
            ReadData { i1: 0, i2: 0 },
        ],
        StoreDescription {
            logs: vec![(
                "".to_string(),
                LogDescription {
                    file_fill_size: 18,
                    log_type: Rolling,
                    data_rate: Constant(1),
                    size_dist: Constant(0),
                    data_dist: U32(0, vec![]),
                },
            )],
        },
    );

    // catches ???
    store_test_scenario(
        vec![
            WriteData {
                seed: 0,
                max_num: 1,
                max_buf_size: 0,
            },
            Commit,
            LoadLatest,
            Reconstruct,
            LoadLatest,
        ],
        StoreDescription {
            logs: vec![(
                "".to_string(),
                LogDescription {
                    file_fill_size: 1,
                    log_type: Append,
                    data_rate: Constant(1),
                    size_dist: Constant(0),
                    data_dist: U16(0, vec![]),
                },
            )],
        },
    );

    store_test_scenario(
        vec![
            WriteData {
                seed: 0,
                max_num: 2,
                max_buf_size: 0,
            },
            Commit,
            Reconstruct,
            WriteData {
                seed: 0,
                max_num: 1,
                max_buf_size: 0,
            },
        ],
        StoreDescription {
            logs: vec![(
                "".to_string(),
                LogDescription {
                    file_fill_size: 14,
                    log_type: Rolling,
                    data_rate: Constant(2),
                    size_dist: Constant(0),
                    data_dist: Byte(1, vec![]),
                },
            )],
        },
    );
}

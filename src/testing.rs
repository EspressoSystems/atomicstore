#![deny(warnings)]

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
use std::sync::Arc;
use tempdir::TempDir;

fn poisson_uni<R: Rng>(prng: &mut R, lambda: f64) -> u32 {
    let lambda = lambda.abs();
    let lambda = if lambda >= 1e-6 { lambda } else { 1e-6 };

    // fake the poisson distribution with a sum-of-uniform for speed

    let n = 100;
    let mut tot = 0f64;
    for _ in 0..n {
        tot += prng.gen_range(0.0..1.0) / (n as f64);
    }

    tot *= lambda / 2.0;

    (tot as i32) as u32
}

#[derive(Clone, Debug)]
enum SizeDistribution {
    Constant(u32),
    UniformRange(u32, u32),
    Poisson { rate: f64 },
}

impl SizeDistribution {
    fn sample<R: Rng>(&self, prng: &mut R) -> u32 {
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
        let options = [
            Self::Constant(<_>::arbitrary(g)),
            Self::UniformRange(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::Poisson {
                rate: <_>::arbitrary(g),
            },
        ];
        g.choose(&options).unwrap().clone()
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        match self {
            Self::Constant(c) => Box::new(c.shrink().map(|c| Self::Constant(c))),

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
                    .chain(once(Self::Constant(rate.abs() as i32 as u32)))
                    .chain(once(Self::UniformRange(
                        0,
                        (rate * 2.0).abs() as i32 as u32,
                    ))),
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum DataDistribution {
    // RandomNoise,
    BytePattern(u8, Vec<u8>),
    U16Pattern(u16, Vec<u16>),
    U32Pattern(u32, Vec<u32>),
    U64Pattern(u64, Vec<u64>),
}

impl DataDistribution {
    fn sample<R: Rng>(&self, _prng: &mut R, buf: &mut [u8]) {
        match self {
            // Self::RandomNoise => {
            // prng.fill(buf);
            // },
            Self::BytePattern(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = all_vs[i % all_vs.len()];
                }
            }
            Self::U16Pattern(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    let ix = i / 2;
                    let shift = 8 * (i % 2);
                    *b = ((all_vs[ix % all_vs.len()] >> shift) & 0xff) as u8;
                }
            }
            Self::U32Pattern(v, vs) => {
                let mut all_vs = vec![*v];
                all_vs.extend(vs.iter().cloned());
                for (i, b) in buf.iter_mut().enumerate() {
                    let ix = i / 4;
                    let shift = 8 * (i % 4);
                    *b = ((all_vs[ix % all_vs.len()] >> shift) & 0xff) as u8;
                }
            }
            Self::U64Pattern(v, vs) => {
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
            Self::BytePattern(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::U16Pattern(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::U32Pattern(<_>::arbitrary(g), <_>::arbitrary(g)),
            Self::U64Pattern(<_>::arbitrary(g), <_>::arbitrary(g)),
        ];
        g.choose(&options).unwrap().clone()
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        match self {
            // Self::RandomNoise => Box::new(std::iter::empty()),
            Self::BytePattern(v, vs) => {
                let v = v.clone();
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::BytePattern(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::BytePattern(v, vs.clone()))),
                )
            }

            Self::U16Pattern(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::U16Pattern(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::U16Pattern(v, vs.clone()))),
                )
            }

            Self::U32Pattern(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::U32Pattern(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::U32Pattern(v, vs.clone()))),
                )
            }

            Self::U64Pattern(v, vs) => {
                let v = *v;
                let vs = vs.clone();

                Box::new(
                    vs.shrink()
                        .map(move |vs| Self::U64Pattern(v, vs.clone()))
                        .chain(v.shrink().map(move |v| Self::U64Pattern(v, vs.clone()))),
                )
            }
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum StorageType {
    Rolling,
    Append,
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
        let file_fill_size = core::cmp::max(1, <_>::arbitrary(g));
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
        let self_arc = Arc::<Self>::new(self.clone());

        Box::new(
            self.file_fill_size
                .shrink()
                .map((|self_arc: Arc<Self>| {
                    move |file_fill_size| -> Self {
                        let mut ret = (*self_arc).clone();
                        ret.file_fill_size = file_fill_size;
                        ret
                    }
                })(self_arc.clone()))
                .chain(self.log_type.shrink().map((|self_arc: Arc<Self>| {
                    move |log_type| -> Self {
                        let mut ret = (*self_arc).clone();
                        ret.log_type = log_type;
                        ret
                    }
                })(self_arc.clone())))
                .chain(self.data_rate.shrink().map((|self_arc: Arc<Self>| {
                    move |data_rate| -> Self {
                        let mut ret = (*self_arc).clone();
                        ret.data_rate = data_rate;
                        ret
                    }
                })(self_arc.clone())))
                .chain(self.size_dist.shrink().map((|self_arc: Arc<Self>| {
                    move |size_dist| {
                        let mut ret = (*self_arc).clone();
                        ret.size_dist = size_dist;
                        ret
                    }
                })(self_arc.clone())))
                .chain(self.data_dist.shrink().map((|self_arc: Arc<Self>| {
                    move |data_dist| {
                        let mut ret = (*self_arc).clone();
                        ret.data_dist = data_dist;
                        ret
                    }
                })(self_arc))),
        )
    }
}

#[derive(Clone, Debug)]
struct StoreDescription {
    logs: Vec<(String, LogDescription)>,
}

impl quickcheck::Arbitrary for StoreDescription {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            logs: <_>::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<(dyn Iterator<Item = Self> + 'static)> {
        Box::new(self.logs.shrink().map(|logs| Self { logs }))
    }
}

#[derive(Clone, Debug)]
enum StorageAction {
    Commit,
    Revert { which_log: u16 },
    Reconstruct,
    // Write data to each log based on their descriptions
    WriteData { seed: u64 },
    LoadLatest,

    // TODO: iterator API coverage
    // indexed back in time
    ReadData { i1: u32, i2: u32 },
    // TODO: metadata corruption?
}

impl quickcheck::Arbitrary for StorageAction {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        use StorageAction::*;
        let options = [
            Commit,
            Revert {
                which_log: <_>::arbitrary(g),
            },
            Reconstruct,
            WriteData {
                seed: <_>::arbitrary(g),
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
            WriteData { seed } => Box::new(seed.shrink().map(|seed| WriteData { seed }))
                as Box<dyn Iterator<Item = _> + 'static>,
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
        let mut store_loader =
            AtomicStoreLoader::create(test_path, "storage_runner_store").unwrap();

        let logs = desc
            .logs
            .into_iter()
            .enumerate()
            .map(|(i, (name, log_desc))| -> Result<_> {
                let name = format!("{}_{}", i, name).escape_default().to_string();
                let name = name.replace("/", "_");
                let name = name[..core::cmp::min(10, name.len())].to_string();
                let log = match log_desc.log_type {
                    StorageType::Append => Log::Append(
                        AppendLog::create(
                            &mut store_loader,
                            <BincodeLoadStore<Vec<u8>>>::default(),
                            &name,
                            log_desc.file_fill_size as u64,
                        )
                        .unwrap(),
                    ),
                    StorageType::Rolling => Log::Rolling(
                        RollingLog::create(
                            &mut store_loader,
                            <BincodeLoadStore<Vec<u8>>>::default(),
                            &name,
                            log_desc.file_fill_size as u64,
                        )
                        .unwrap(),
                    ),
                };

                Ok(IdealLog {
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
        Self::new_with_path(desc, TempDir::new("atomicstore_test").unwrap())
    }

    fn run_action(mut self, action: StorageAction) -> Self {
        use StorageAction::*;
        println!("{:?}", &action);
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
                drop(self.store);

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
                        (lg.name, lg.desc)
                    })
                    .collect();

                self = Self::new_with_path(StoreDescription { logs }, self.directory).unwrap();
                for (lg, stored_items) in self.logs.iter_mut().zip(log_stored_items.into_iter()) {
                    lg.stored_items = stored_items;
                }
            }

            WriteData { seed } => {
                let mut base_prng = ChaChaRng::seed_from_u64(seed);

                for log in self.logs.iter_mut() {
                    let mut log_prng = ChaChaRng::from_rng(&mut base_prng).unwrap();
                    let num_resources = log.desc.data_rate.sample(&mut log_prng) / (1u32 << 28);
                    for _ in 0..num_resources {
                        let buf_size =
                            (log.desc.size_dist.sample(&mut log_prng) / (1u32 << 16)) as usize;
                        let mut buf = Vec::with_capacity(buf_size);
                        buf.resize(buf_size, 0);
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

                    let i1 = log.stored_items.len() - 1 - (i1 % log.stored_items.len());
                    let i2 = log.stored_items.len() - 1 - (i2 % log.stored_items.len());
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
    println!("{:?}", desc);
    let mut runner = StorageRunner::new(desc).unwrap();
    for action in actions {
        runner = runner.run_action(action);
    }
}

#[quickcheck]
fn store_test_scenario_quickcheck(actions: Vec<StorageAction>, desc: StoreDescription) {
    store_test_scenario(actions, desc)
}

#[test]
fn store_test_scenario_regressions() {
    use DataDistribution::*;
    use SizeDistribution::*;
    use StorageAction::*;
    store_test_scenario(
        vec![
            WriteData { seed: 0 },
            Commit,
            Reconstruct,
            ReadData { i1: 0, i2: 0 },
        ],
        StoreDescription {
            logs: vec![(
                "".to_string(),
                LogDescription {
                    file_fill_size: 0,
                    log_type: StorageType::Rolling,
                    data_rate: UniformRange(0, 296925441),
                    size_dist: Constant(0),
                    data_dist: U16Pattern(0, vec![]),
                },
            )],
        },
    )
}

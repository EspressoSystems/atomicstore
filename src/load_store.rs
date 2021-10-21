use crate::error::{BincodeDeError, BincodeSerError, PersistenceError};
use crate::storage_location::StorageLocation;
use crate::Result;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{de::DeserializeOwned, Serialize};
use snafu::ResultExt;

use std::marker::PhantomData;

pub trait LoadStore {
    type ParamType;

    fn load(&self, stream: &[u8]) -> Result<Self::ParamType>;
    fn store(&mut self, param: &Self::ParamType) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct BincodeLoadStore<ParamType: Serialize + DeserializeOwned> {
    phantom: PhantomData<ParamType>,
}

impl<ParamType: Serialize + DeserializeOwned> LoadStore for BincodeLoadStore<ParamType> {
    type ParamType = ParamType;

    fn load(&self, stream: &[u8]) -> Result<Self::ParamType> {
        bincode::deserialize(stream).context(BincodeDeError)
    }
    fn store(&mut self, param: &Self::ParamType) -> Result<Vec<u8>> {
        bincode::serialize(param).context(BincodeSerError)
    }
}

impl<ParamType: Serialize + DeserializeOwned> Default for BincodeLoadStore<ParamType> {
    fn default() -> Self {
        BincodeLoadStore {
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ArkLoadStore<ParamType: CanonicalSerialize + CanonicalDeserialize> {
    phantom: PhantomData<ParamType>,
}

impl<ParamType: CanonicalSerialize + CanonicalDeserialize> LoadStore for ArkLoadStore<ParamType> {
    type ParamType = ParamType;

    fn load(&self, stream: &[u8]) -> Result<Self::ParamType> {
        Self::ParamType::deserialize(stream).map_err(|err| PersistenceError::ArkDeError { err })
    }
    fn store(&mut self, param: &Self::ParamType) -> Result<Vec<u8>> {
        let mut ser_bytes: Vec<u8> = Vec::new();
        param
            .serialize(&mut ser_bytes)
            .map_err(|err| PersistenceError::ArkSerError { err })?;
        Ok(ser_bytes)
    }
}

impl<ParamType: CanonicalSerialize + CanonicalDeserialize> Default for ArkLoadStore<ParamType> {
    fn default() -> Self {
        ArkLoadStore {
            phantom: PhantomData,
        }
    }
}

// #[derive(Debug, Default)]
// pub struct StorageLocationLoadStore;

// impl LoadStore for StorageLocationLoadStore {
//     type ParamType = StorageLocation;

//     fn load(stream: &[u8]) -> Result<Self::ParamType> {
//         bincode::deserialize(stream).context(BincodeDeError)
//     }
//     fn store(param: &Self::ParamType) -> Result<Vec<u8>> {
//         bincode::serialize(param).context(BincodeSerError)
//     }
// }

pub type StorageLocationLoadStore = BincodeLoadStore<StorageLocation>;

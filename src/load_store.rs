// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::error::{BincodeDeSnafu, BincodeSerSnafu, PersistenceError};
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
        bincode::deserialize(stream).context(BincodeDeSnafu)
    }
    fn store(&mut self, param: &Self::ParamType) -> Result<Vec<u8>> {
        bincode::serialize(param).context(BincodeSerSnafu)
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
        Self::ParamType::deserialize(stream).map_err(|err| PersistenceError::ArkDe { err })
    }
    fn store(&mut self, param: &Self::ParamType) -> Result<Vec<u8>> {
        let mut ser_bytes: Vec<u8> = Vec::new();
        param
            .serialize(&mut ser_bytes)
            .map_err(|err| PersistenceError::ArkSer { err })?;
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

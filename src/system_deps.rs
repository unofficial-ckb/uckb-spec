// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{collections::HashMap, convert::TryFrom};

use property::Property;
use serde_derive::Deserialize;

use ckb_types::H256;

use crate::{
    blockchain::{Bundled, DepGroupId, Network},
    error::{Error, Result},
};

lazy_static::lazy_static! {
    static ref SYSTEM_DEPS: SystemDeps = SystemDeps::initialize().unwrap();
}

#[derive(Deserialize)]
struct RawSpecHashes {
    genesis: H256,
    cellbase: H256,
    system_cells: Vec<RawSystemCell>,
    dep_groups: Vec<RawDepGroup>,
}

#[derive(Deserialize)]
struct RawSystemCell {
    path: String,
    tx_hash: H256,
    index: usize,
    data_hash: H256,
    type_hash: Option<H256>,
}

#[derive(Deserialize)]
struct RawDepGroup {
    included_cells: Vec<String>,
    tx_hash: H256,
    index: usize,
}

pub struct SystemDeps(HashMap<Network, SpecHashes>);

#[derive(Property, Debug, Clone)]
#[property(get(public), set(disable), mut(disable))]
pub struct SpecHashes {
    genesis: H256,
    cellbase: H256,
    system_cells: HashMap<Bundled, SystemCell>,
    dep_groups: HashMap<DepGroupId, DepGroup>,
}

#[derive(Property, Debug, Clone)]
#[property(get(public), set(disable), mut(disable))]
pub struct SystemCell {
    tx_hash: H256,
    index: usize,
    data_hash: H256,
    type_hash: Option<H256>,
}

#[derive(Property, Debug, Clone)]
#[property(get(public), set(disable), mut(disable))]
pub struct DepGroup {
    tx_hash: H256,
    index: usize,
}

fn network_from_spec_name(spec_name: &str) -> Result<Network> {
    match spec_name {
        "ckb" => Ok("mainnet"),
        "ckb_testnet" => Ok("testnet"),
        "ckb_staging" => Ok("staging"),
        "ckb_dev" => Ok("develop"),
        s => Ok(s),
    }
    .and_then(Network::try_from)
}

impl SystemDeps {
    pub fn read() -> &'static Self {
        &SYSTEM_DEPS
    }

    fn initialize() -> Result<Self> {
        let raw_spec_hashes: HashMap<String, RawSpecHashes> =
            toml::from_str(include_str!("resources/hashes.toml")).unwrap();
        raw_spec_hashes
            .into_iter()
            .map(|(spec_name, raw_spec_values)| {
                let network = network_from_spec_name(&spec_name)?;
                let RawSpecHashes {
                    genesis,
                    cellbase,
                    system_cells,
                    dep_groups,
                } = raw_spec_values;
                let system_cells = system_cells
                    .into_iter()
                    .map(|raw_system_cell| {
                        let RawSystemCell {
                            path,
                            tx_hash,
                            index,
                            data_hash,
                            type_hash,
                        } = raw_system_cell;
                        let bundled = Bundled::try_from(path.as_str())?;
                        let system_cell = SystemCell {
                            tx_hash,
                            index,
                            data_hash,
                            type_hash,
                        };
                        Ok((bundled, system_cell))
                    })
                    .collect::<Result<HashMap<_, _>>>()?;
                let dep_groups = dep_groups
                    .into_iter()
                    .map(|raw_dep_group| {
                        let RawDepGroup {
                            included_cells,
                            tx_hash,
                            index,
                        } = raw_dep_group;
                        let included_cells = included_cells
                            .into_iter()
                            .map(|path| Bundled::try_from(path.as_str()))
                            .collect::<Result<Vec<Bundled>>>()?;
                        let dep_group_id = DepGroupId::try_from(&included_cells[..])?;
                        let dep_group = DepGroup { tx_hash, index };
                        Ok((dep_group_id, dep_group))
                    })
                    .collect::<Result<HashMap<_, _>>>()?;
                let spec_hashes = SpecHashes {
                    genesis,
                    cellbase,
                    system_cells,
                    dep_groups,
                };
                Ok((network, spec_hashes))
            })
            .collect::<Result<HashMap<_, _>>>()
            .map(Self)
    }

    pub fn lookup_system_cell(&self, network: Network, key: Bundled) -> Result<&SystemCell> {
        self.0
            .get(&network)
            .and_then(|spec| spec.system_cells.get(&key))
            .ok_or_else(|| Error::UnknownSystemCell(network, key))
    }

    pub fn lookup_dep_group(&self, network: Network, key: DepGroupId) -> Result<&DepGroup> {
        self.0
            .get(&network)
            .and_then(|spec| spec.dep_groups.get(&key))
            .ok_or_else(|| Error::UnknownDepGroup(network, key))
    }
}

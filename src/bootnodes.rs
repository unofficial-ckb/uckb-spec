// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{collections::HashMap, fs};

use tentacle::multiaddr::Multiaddr;

use crate::{
    blockchain::Network,
    error::{Error, Result},
};

lazy_static::lazy_static! {
    static ref BOOTNODES: BootNodes = BootNodes::initialize().unwrap();
}

pub struct BootNodes(HashMap<Network, Vec<Multiaddr>>);

fn load_raw_bootnodes_from_file(chain: &str) -> Result<Vec<String>> {
    let out_dir = env!("OUT_DIR");
    let file_path = format!("{}/{}/ckb.toml", out_dir, chain);
    fs::read_to_string(&file_path)?
        .parse::<toml::Value>()?
        .get("network")
        .ok_or_else(|| Error::Unreachable(format!("`network` for chain {} was not found", chain)))?
        .get("bootnodes")
        .ok_or_else(|| {
            Error::Unreachable(format!(
                "`network::bootnodes` for chain {} was not found",
                chain
            ))
        })?
        .as_array()
        .ok_or_else(|| {
            Error::Unreachable(format!(
                "`network::bootnodes` for chain {} was not array",
                chain
            ))
        })?
        .iter()
        .map(|v| {
            v.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                Error::Unreachable(format!(
                    "at least one item in `network::bootnodes` for chain {} was not string",
                    chain
                ))
            })
        })
        .collect::<Result<Vec<_>>>()
}

fn parse_multiaddrs(multiaddrs_str: &[String]) -> Result<Vec<Multiaddr>> {
    multiaddrs_str
        .iter()
        .map(|addr| {
            addr.parse()
                .map_err(|_| Error::Unknown("bootnode", (*addr).to_owned()))
        })
        .collect()
}

impl BootNodes {
    pub fn read() -> &'static Self {
        &BOOTNODES
    }

    fn initialize() -> Result<Self> {
        let mut bootnodes = Self(HashMap::new());
        bootnodes.initialize_network(Network::Mainnet)?;
        bootnodes.initialize_network(Network::Testnet)?;
        bootnodes.initialize_network(Network::Staging)?;
        bootnodes.initialize_network(Network::Develop)?;
        Ok(bootnodes)
    }

    fn initialize_network(&mut self, network: Network) -> Result<()> {
        let chain = network.to_string().to_lowercase();
        let raw_bootnodes = load_raw_bootnodes_from_file(&chain)?;
        self.0
            .insert(network, parse_multiaddrs(&raw_bootnodes[..])?);
        Ok(())
    }

    pub fn lookup(&self, network: Network) -> &[Multiaddr] {
        self.0.get(&network).unwrap()
    }
}

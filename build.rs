// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{collections, env, fs, io::Write, path};

use serde_derive::{Deserialize, Serialize};

use ckb_chain_spec as spec;
use ckb_resource as res;
use ckb_types::{packed, prelude::*, H256};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SystemCell {
    path: String,
    tx_hash: H256,
    index: usize,
    data_hash: H256,
    type_hash: Option<H256>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DepGroupCell {
    included_cells: Vec<String>,
    tx_hash: H256,
    index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecHashes {
    genesis: H256,
    cellbase: H256,
    system_cells: Vec<SystemCell>,
    dep_groups: Vec<DepGroupCell>,
}

fn create_template_context<'a>(spec: &'a str) -> res::TemplateContext<'a> {
    res::TemplateContext {
        spec,
        spec_source: "bundled",
        rpc_port: res::DEFAULT_RPC_PORT,
        p2p_port: res::DEFAULT_P2P_PORT,
        log_to_file: false,
        log_to_stdout: false,
        block_assembler: "",
    }
}

fn load_hashes_from_chain_spec(mut chain_spec: spec::ChainSpec) -> SpecHashes {
    let spec_name = &chain_spec.name;
    let hash_option = chain_spec.genesis.hash.take();
    let consensus = chain_spec
        .build_consensus()
        .expect(&format!("failed to build consensus for {}", spec_name));
    if let Some(hash) = hash_option {
        let genesis_hash: H256 = consensus.genesis_hash().unpack();
        if hash != genesis_hash {
            panic!(
                "Genesis hash unmatched in {} chainspec config file: in file {:#x}, actual {:#x}",
                spec_name, hash, genesis_hash
            );
        }
    }

    let block = consensus.genesis_block();
    let cellbase = &block.transactions()[0];
    let dep_group_tx = &block.transactions()[1];

    let cells_hashes = chain_spec
        .genesis
        .system_cells
        .iter()
        .map(|system_cell| &system_cell.file)
        .zip(
            cellbase
                .outputs()
                .into_iter()
                .zip(cellbase.outputs_data().into_iter())
                .skip(1),
        )
        .enumerate()
        .map(|(index_minus_one, (resource, (output, data)))| {
            let data_hash: H256 = packed::CellOutput::calc_data_hash(&data.raw_data()).unpack();
            let type_hash: Option<H256> = output
                .type_()
                .to_opt()
                .map(|script| script.calc_script_hash().unpack());
            SystemCell {
                path: resource.to_string(),
                tx_hash: cellbase.hash().unpack(),
                index: index_minus_one + 1,
                data_hash,
                type_hash,
            }
        })
        .collect();

    let dep_groups = chain_spec
        .genesis
        .dep_groups
        .iter()
        .enumerate()
        .map(|(index, dep_group)| DepGroupCell {
            included_cells: dep_group
                .files
                .iter()
                .map(|res| res.to_string())
                .collect::<Vec<_>>(),
            tx_hash: dep_group_tx.hash().unpack(),
            index,
        })
        .collect::<Vec<_>>();

    SpecHashes {
        genesis: consensus.genesis_hash().unpack(),
        cellbase: cellbase.hash().unpack(),
        system_cells: cells_hashes,
        dep_groups,
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("environment variable OUT_DIR should be existed");

    let hashes_filename = "hashes.toml";
    let hashes_file = path::Path::new(&out_dir).join(hashes_filename);

    let mut hashes_fd =
        fs::File::create(&hashes_file).expect(&format!("failed to create file {:?}", hashes_file));
    let mut first = true;

    for (name, spec_name) in &[
        ("mainnet", "mainnet"),
        ("testnet", "testnet"),
        ("staging", "staging"),
        ("develop", "dev"),
    ] {
        if first {
            first = false;
        } else {
            hashes_fd
                .write_all("\n".as_bytes())
                .expect(&format!("failed to write file {:?}", hashes_file));
        }
        let dir = path::Path::new(&out_dir).join(name);
        fs::create_dir_all(&dir).expect(&format!("failed to create directory {:?}", dir));
        res::Resource::bundled_ckb_config()
            .export(&create_template_context(spec_name), dir)
            .expect(&format!("failed to export ckb config for {}", name));
        let bundled = res::Resource::bundled(format!("specs/{}.toml", spec_name));
        let chain_spec = spec::ChainSpec::load_from(&bundled)
            .expect(&format!("failed to load ckb chain spec for {}", name));
        let spec_name = chain_spec.name.clone();
        let spec_hashes = load_hashes_from_chain_spec(chain_spec);
        let mut spec_hashes_map = collections::BTreeMap::default();
        spec_hashes_map.insert(spec_name, spec_hashes);
        let spec_hashes_string = toml::to_string(&spec_hashes_map).unwrap();
        hashes_fd
            .write_all(spec_hashes_string.as_bytes())
            .expect(&format!("failed to write file {:?}", hashes_file));
    }
}

// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use property::Property;

use ckb_types::{bytes, packed, prelude::*};

use crate::error::{Error, Result};

lazy_static::lazy_static! {
    static ref BASE_STRUCT: BaseStruct = BaseStruct::initialize();
    static ref BASE_SERIALIZED_SIZE: BaseSerializedSize= BaseSerializedSize::initialize().unwrap();
}

#[derive(Property, Debug)]
#[property(get(public), set(disable), mut(disable))]
pub struct BaseStruct {
    block: packed::Block,
    transaction: packed::Transaction,
}

#[derive(Property, Debug)]
#[property(get(public), set(disable), mut(disable))]
pub struct BaseSerializedSize {
    block: usize,
    transaction: usize,
    cell_input: usize,
    cell_output: usize,
    witness: usize,
}

impl BaseStruct {
    pub fn read() -> &'static Self {
        &BASE_STRUCT
    }

    fn initialize() -> Self {
        let block = Self::construct_base_block();
        let transaction = Self::construct_base_transaction();
        Self { block, transaction }
    }

    fn construct_base_block() -> packed::Block {
        let header = {
            let empty_dao = packed::Byte32::default();
            let raw_header = packed::RawHeader::new_builder().dao(empty_dao).build();
            packed::Header::new_builder().raw(raw_header).build()
        };
        let cellbase = {
            let input = packed::CellInput::new_builder().build();
            let output = packed::CellOutput::new_builder().build();
            let output_data = packed::Bytes::new_builder().build();
            let cellbase_witness = packed::CellbaseWitness::new_builder().build();
            let raw_tx = packed::RawTransaction::new_builder()
                .inputs(vec![input].pack())
                .outputs(vec![output].pack())
                .outputs_data(vec![output_data].pack())
                .build();
            let witness = cellbase_witness.as_bytes().pack();
            packed::Transaction::new_builder()
                .raw(raw_tx)
                .witnesses(vec![witness].pack())
                .build()
        };
        packed::Block::new_builder()
            .header(header)
            .transactions(vec![cellbase].pack())
            .build()
    }

    fn construct_base_transaction() -> packed::Transaction {
        let cell_dep = packed::CellDep::new_builder().build();
        let input = packed::CellInput::new_builder().build();
        let args = bytes::Bytes::from(vec![0u8; 20]);
        let script = packed::Script::new_builder().args(args.pack()).build();
        let output = packed::CellOutput::new_builder().lock(script).build();
        let output_data = packed::Bytes::new_builder().build();
        let signature = bytes::Bytes::from(vec![0u8; 65]);
        let witness = packed::WitnessArgs::new_builder()
            .lock(Some(signature).pack())
            .build()
            .as_bytes()
            .pack();
        let raw_tx = packed::RawTransaction::new_builder()
            .cell_deps(vec![cell_dep].pack())
            .inputs(vec![input].pack())
            .outputs(vec![output].pack())
            .outputs_data(vec![output_data].pack())
            .build();
        packed::Transaction::new_builder()
            .raw(raw_tx)
            .witnesses(vec![witness].pack())
            .build()
    }
}

impl BaseSerializedSize {
    pub fn read() -> &'static Self {
        &BASE_SERIALIZED_SIZE
    }

    fn initialize() -> Result<Self> {
        let block = Self::estimate_base_block_serialized_size()?;
        let cell_input = Self::estimate_base_cell_input_serialized_size()?;
        let cell_output = Self::estimate_base_cell_output_serialized_size()?;
        let witness = Self::estimate_base_witness_serialized_size()?;
        let transaction = BaseStruct::read().transaction().serialized_size_in_block()
            - (cell_input + cell_output + witness);
        Ok(Self {
            block,
            transaction,
            cell_input,
            cell_output,
            witness,
        })
    }

    fn estimate_base_block_serialized_size() -> Result<usize> {
        let block = BaseStruct::read().block();
        let tx = BaseStruct::read().transaction();
        let base_block_size = block.serialized_size_without_uncle_proposals();
        let tx_size = tx.serialized_size_in_block();
        let cellbase = block.clone().transactions().get_unchecked(0);
        let block1 = block
            .clone()
            .as_builder()
            .transactions(vec![cellbase.clone(), tx.clone()].pack())
            .build();
        let block2 = block
            .clone()
            .as_builder()
            .transactions(vec![cellbase.clone(), tx.clone(), tx.clone()].pack())
            .build();
        if block1.serialized_size_without_uncle_proposals() != base_block_size + tx_size
            || block2.serialized_size_without_uncle_proposals() != base_block_size + tx_size * 2
        {
            Err(Error::Unreachable(
                "failed to estimate serialized size for blocks".to_owned(),
            ))
        } else {
            Ok(base_block_size)
        }
    }

    fn estimate_base_cell_input_serialized_size() -> Result<usize> {
        let tx0 = BaseStruct::read().transaction();
        let input = tx0.clone().raw().inputs().get_unchecked(0);
        let raw_tx1 = tx0
            .clone()
            .raw()
            .as_builder()
            .inputs(vec![input.clone(), input.clone()].pack())
            .build();
        let raw_tx2 = tx0
            .clone()
            .raw()
            .as_builder()
            .inputs(vec![input.clone(), input.clone(), input.clone()].pack())
            .build();
        let tx1 = tx0.clone().as_builder().raw(raw_tx1).build();
        let tx2 = tx0.clone().as_builder().raw(raw_tx2).build();
        let tx0_size = tx0.serialized_size_in_block();
        let tx1_size = tx1.serialized_size_in_block();
        let tx2_size = tx2.serialized_size_in_block();
        if tx0_size + tx2_size != tx1_size * 2 {
            Err(Error::Unreachable(
                "failed to estimate serialized size for cell inputs".to_owned(),
            ))
        } else {
            Ok(tx1_size - tx0_size)
        }
    }

    fn estimate_base_cell_output_serialized_size() -> Result<usize> {
        let tx0 = BaseStruct::read().transaction();
        let output = tx0.clone().raw().outputs().get_unchecked(0);
        let output_data = tx0.clone().raw().outputs_data().get_unchecked(0);
        let raw_tx1 = tx0
            .clone()
            .raw()
            .as_builder()
            .outputs(vec![output.clone(), output.clone()].pack())
            .outputs_data(vec![output_data.clone(), Default::default()].pack())
            .build();
        let raw_tx2 = tx0
            .clone()
            .raw()
            .as_builder()
            .outputs(vec![output.clone(), output.clone(), output.clone()].pack())
            .outputs_data(vec![output_data.clone(), Default::default(), Default::default()].pack())
            .build();
        let tx1 = tx0.clone().as_builder().raw(raw_tx1).build();
        let tx2 = tx0.clone().as_builder().raw(raw_tx2).build();
        let tx0_size = tx0.serialized_size_in_block();
        let tx1_size = tx1.serialized_size_in_block();
        let tx2_size = tx2.serialized_size_in_block();
        if tx0_size + tx2_size != tx1_size * 2 {
            Err(Error::Unreachable(
                "failed to estimate serialized size for cell outputs".to_owned(),
            ))
        } else {
            Ok(tx1_size - tx0_size)
        }
    }

    fn estimate_base_witness_serialized_size() -> Result<usize> {
        let tx0 = BaseStruct::read().transaction();
        let witness = tx0.clone().witnesses().get_unchecked(0);
        let tx1 = tx0
            .clone()
            .as_builder()
            .witnesses(vec![witness.clone(), witness.clone()].pack())
            .build();
        let tx2 = tx0
            .clone()
            .as_builder()
            .witnesses(vec![witness.clone(), witness.clone(), witness.clone()].pack())
            .build();
        let tx0_size = tx0.serialized_size_in_block();
        let tx1_size = tx1.serialized_size_in_block();
        let tx2_size = tx2.serialized_size_in_block();
        if tx0_size + tx2_size != tx1_size * 2 {
            Err(Error::Unreachable(
                "failed to estimate serialized size for witnesses".to_owned(),
            ))
        } else {
            Ok(tx1_size - tx0_size)
        }
    }
}

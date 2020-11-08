// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use thiserror::Error;

use crate::blockchain;

#[derive(Debug, Error)]
pub enum Error {
    #[error("internal error: should be unreachable, {0}")]
    Unreachable(String),

    #[error("io error: {0}")]
    IO(#[from] io::Error),
    #[error("toml error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("unknown {0} '{1}'")]
    Unknown(&'static str, String),
    #[error("unknown system cell: {0:?}, {1:?}")]
    UnknownSystemCell(blockchain::Network, blockchain::Bundled),
    #[error("unknown dep group: {0:?}, {1:?}")]
    UnknownDepGroup(blockchain::Network, blockchain::DepGroupId),
}

pub type Result<T> = ::std::result::Result<T, Error>;

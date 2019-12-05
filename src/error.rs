// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use failure::Fail;

use crate::blockchain;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown {} '{}'", _0, _1)]
    Unknown(&'static str, String),

    #[fail(display = "unknown system cell: {:?}, {:?}", _0, _1)]
    UnknownSystemCell(blockchain::Network, blockchain::Bundled),
    #[fail(display = "unknown dep group: {:?}, {:?}", _0, _1)]
    UnknownDepGroup(blockchain::Network, blockchain::DepGroupId),
}

pub type Result<T> = ::std::result::Result<T, Error>;

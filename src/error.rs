// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use failure::Fail;

use crate::blockchain;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal error: should be unreachable, {}", _0)]
    Unreachable(String),

    #[fail(display = "io error: {}", _0)]
    IO(io::Error),
    #[fail(display = "toml error: {}", _0)]
    Toml(toml::de::Error),

    #[fail(display = "unknown {} '{}'", _0, _1)]
    Unknown(&'static str, String),
    #[fail(display = "unknown system cell: {:?}, {:?}", _0, _1)]
    UnknownSystemCell(blockchain::Network, blockchain::Bundled),
    #[fail(display = "unknown dep group: {:?}, {:?}", _0, _1)]
    UnknownDepGroup(blockchain::Network, blockchain::DepGroupId),
}

pub type Result<T> = ::std::result::Result<T, Error>;

macro_rules! convert_error {
    ($name:ident, $inner_error:ty) => {
        impl ::std::convert::From<$inner_error> for Error {
            fn from(error: $inner_error) -> Self {
                Self::$name(error)
            }
        }
    };
}

convert_error!(IO, io::Error);
convert_error!(Toml, toml::de::Error);

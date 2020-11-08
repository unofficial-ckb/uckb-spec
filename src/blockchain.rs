// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{convert::TryFrom, fmt, str::FromStr};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Network {
    Mainnet,
    Testnet,
    Staging,
    Develop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bundled {
    Secp256k1Blake160,
    Secp256k1Blake160MultiSig,
    Secp256k1Data,
    Dao,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepGroupId {
    Secp256k1Blake160,
    Secp256k1Blake160MultiSig,
}

impl Network {
    pub(crate) const NAME: &'static str = "network";
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Mainnet => "Mainnet",
            Self::Testnet => "Testnet",
            Self::Staging => "Staging",
            Self::Develop => "Develop",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Network {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "mainnet" => Ok(Self::Mainnet),
            "testnet" => Ok(Self::Testnet),
            "staging" => Ok(Self::Staging),
            "develop" => Ok(Self::Develop),
            _ => Err(Error::Unknown(Self::NAME, s.to_owned())),
        }
    }
}

impl TryFrom<&str> for Network {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl Bundled {
    pub(crate) const NAME: &'static str = "bundled";
}

impl FromStr for Bundled {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Bundled(specs/cells/secp256k1_blake160_sighash_all)" => Ok(Self::Secp256k1Blake160),
            "Bundled(specs/cells/secp256k1_blake160_multisig_all)" => {
                Ok(Self::Secp256k1Blake160MultiSig)
            }
            "Bundled(specs/cells/secp256k1_data)" => Ok(Self::Secp256k1Data),
            "Bundled(specs/cells/dao)" => Ok(Self::Dao),
            _ => Err(Error::Unknown(Self::NAME, s.to_owned())),
        }
    }
}

impl TryFrom<&str> for Bundled {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl DepGroupId {
    pub(crate) const NAME: &'static str = "dep-group";
}

impl TryFrom<&[Bundled]> for DepGroupId {
    type Error = Error;
    fn try_from(value: &[Bundled]) -> Result<Self> {
        if value.len() == 2 {
            match (value[0], value[1]) {
                (Bundled::Secp256k1Data, Bundled::Secp256k1Blake160) => Ok(Self::Secp256k1Blake160),
                (Bundled::Secp256k1Data, Bundled::Secp256k1Blake160MultiSig) => {
                    Ok(Self::Secp256k1Blake160MultiSig)
                }
                _ => Err(Error::Unknown(Self::NAME, format!("{:?}", value))),
            }
        } else {
            Err(Error::Unknown(Self::NAME, format!("{:?}", value)))
        }
    }
}

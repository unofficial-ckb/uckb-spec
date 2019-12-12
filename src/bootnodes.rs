// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;

use tentacle::multiaddr::Multiaddr;

use crate::{
    blockchain::Network,
    error::{Error, Result},
};

lazy_static::lazy_static! {
    static ref BOOTNODES: BootNodes = BootNodes::initialize().unwrap();
}

const BOOTNODES_MAINNET: &[&str] = &[
    "/ip4/47.110.15.57/tcp/8114/p2p/QmXS4Kbc9HEeykHUTJCm2tNmqghbvWyYpUp6BtE5b6VrAU",
    "/ip4/47.245.31.79/tcp/8114/p2p/QmUaSuEdXNGJEKvkE4rCn3cwBrpRFUm5TsouF4M3Sjursv",
    "/ip4/13.234.144.148/tcp/8114/p2p/QmbT7QimcrcD5k2znoJiWpxoESxang6z1Gy9wof1rT1LKR",
    "/ip4/3.218.170.86/tcp/8114/p2p/QmShw2vtVt49wJagc1zGQXGS6LkQTcHxnEV3xs6y8MAmQN",
    "/ip4/52.59.155.249/tcp/8114/p2p/QmRHqhSGMGm5FtnkW8D6T83X7YwaiMAZXCXJJaKzQEo3rb",
    "/ip4/39.107.100.85/tcp/8114/p2p/QmeQwD2GGuZyFzDPbQEKFJUjmNaY9FG3X7WHPL823zFD3a",
    "/ip4/47.103.44.208/tcp/8114/p2p/QmWVt9kNFv8XM1CMqeJj2nmG4tJ4ViqgwzJMr7yYynp6qd",
    "/ip4/120.24.85.239/tcp/8114/p2p/QmaJP1sDiWZuwAEMghNHt7TrTgaMCyYaEMMYLc4YvoUGSV",
    "/ip4/47.91.238.128/tcp/8114/p2p/QmTjLAewCM6SivjpW7BJfSj1ABuPA7x6FsFu5ga7Xy2xig",
    "/ip4/147.139.137.53/tcp/8114/p2p/QmQBLw9TqkS8yu2Kg8UtiYzvxEQ7DfKiXLx8iD7bF8XRyj",
    "/ip4/13.52.18.181/tcp/8114/p2p/QmcEK1wUR287qSYdw8eHNWeQrFitQsCaZHHTM9wgvakxnS",
    "/ip4/18.140.240.153/tcp/8114/p2p/QmVi7reKhqVnoBYzW2nJYnrujVeckrZXhwuYbX7P2whPJg",
    "/ip4/35.183.172.68/tcp/8114/p2p/QmNRAvtC6L85hwp6vWnqaKonJw3dz1q39B4nXVQErzC4Hx",
    "/ip4/3.10.216.39/tcp/8114/p2p/QmagxSv7GNwKXQE7mi1iDjFHghjUpbqjBgqSot7PmMJqHA",
    "/ip4/3.105.209.193/tcp/8114/p2p/QmQidJaxciY3NT2PjsaCR4Gz8vB8kFn3Avwz96u6b3jGc1",
    "/ip4/18.229.214.173/tcp/8114/p2p/QmVeeCh81GTLGRwB7vRHXeTRdUHRYcfn6qKEfewhtiRJZC",
];
const BOOTNODES_TESTNET: &[&str] = &[
    "/ip4/47.111.169.36/tcp/8111/p2p/QmNQ4jky6uVqLDrPU7snqxARuNGWNLgSrTnssbRuy3ij2W",
    "/ip4/18.217.146.65/tcp/8111/p2p/QmT6DFfm18wtbJz3y4aPNn3ac86N4d4p4xtfQRRPf73frC",
    "/ip4/18.136.60.221/tcp/8111/p2p/QmTt6HeNakL8Fpmevrhdna7J4NzEMf9pLchf1CXtmtSrwb",
    "/ip4/35.176.207.239/tcp/8111/p2p/QmSJTsMsMGBjzv1oBNwQU36VhQRxc2WQpFoRu1ZifYKrjZ",
];
const BOOTNODES_STAGING: &[&str] =
    &["/ip4/47.103.65.40/tcp/8116/p2p/QmaroeHayKSUoQod7idrERhxNsM6dX7qSMTQaCLkyU9ygE"];
const BOOTNODES_DEVELOP: &[&str] = &[];

pub struct BootNodes(HashMap<Network, Vec<Multiaddr>>);

fn parse_multiaddrs(multiaddrs_str: &[&str]) -> Result<Vec<Multiaddr>> {
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
        let mut inner = HashMap::new();
        inner.insert(Network::Mainnet, parse_multiaddrs(BOOTNODES_MAINNET)?);
        inner.insert(Network::Testnet, parse_multiaddrs(BOOTNODES_TESTNET)?);
        inner.insert(Network::Staging, parse_multiaddrs(BOOTNODES_STAGING)?);
        inner.insert(Network::Develop, parse_multiaddrs(BOOTNODES_DEVELOP)?);
        Ok(Self(inner))
    }

    pub fn lookup(&self, network: Network) -> &[Multiaddr] {
        self.0.get(&network).unwrap()
    }
}

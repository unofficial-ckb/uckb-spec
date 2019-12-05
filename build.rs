// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{fs::OpenOptions, io::Read};

use sha2::{Digest, Sha256};
use slices::u8_slice as x;

fn check_checksum(path: &str, checksum: &[u8]) {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut hasher = Sha256::new();
    hasher.input(&buffer);
    let result = hasher.result();
    assert_eq!(&result[..], checksum);
}

fn main() {
    let checksum = if cfg!(windows) {
        x!("0x4a3c9776861fb0eb7f5dff851730be5505734ec84a2180f78fa56b7641e9405a")
    } else {
        x!("0xcbe8be3fefa486ce070607ea51634d7e98a338713abe1542f7a284f57c08ef33")
    };
    check_checksum("src/resources/hashes.toml", checksum);
}

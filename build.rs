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
        x!("0x94aea560d82f606a4101b384a7e43672431d5477630060a987a93f35dfbd07c9")
    } else {
        x!("0x6340a9c0a5d8b419fc6a6529d02089e5f3b5b53e65552a666323e61385b236e7")
    };
    check_checksum("src/resources/hashes.toml", checksum);
}

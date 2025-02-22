//
// Copyright 2023 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Utilities to handle encoded keys and certificates

use alloc::{format, string::String, vec::Vec};
use core::result::Result;

/// Extracts the bytes used to encode a CBOR object from a slice that might include unused bytes by
/// deserializing and re-serializing the encoded CBOR object.
pub fn cbor_encoded_bytes_to_vec(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = Vec::new();
    let value: ciborium::Value =
        ciborium::from_reader(bytes).map_err(|err| format!("failed to read bytes: {:?}", err))?;
    ciborium::into_writer(&value, &mut result)
        .map_err(|err| format!("failed to write bytes: {:?}", err))?;
    Ok(result)
}

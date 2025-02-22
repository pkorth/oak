//
// Copyright 2022 The Project Oak Authors
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

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod proto {
    pub mod oak {
        tonic::include_proto!("oak");
        pub mod attestation {
            pub mod v1 {
                tonic::include_proto!("oak.attestation.v1");
            }
        }
    }
}

pub mod endorsement;
pub mod rekor;
pub mod util;
pub mod verifier;

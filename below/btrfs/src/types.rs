// Copyright (c) Facebook, Inc. and its affiliates.
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

use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BtrfsStat {
    pub name: Option<String>,
    pub disk_fraction: Option<f64>,
    pub disk_bytes: Option<u64>,
}

pub type BtrfsMap = BTreeMap<String, BtrfsStat>;

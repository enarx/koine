// SPDX-License-Identifier: Apache-2.0

use super::super::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type KeepList = Arc<Mutex<Vec<Keep>>>;
pub type ContractList = Arc<Mutex<Vec<KeepContract>>>;
pub type KeepLdrConnList = Arc<Mutex<Vec<KeepLdrConnection>>>;

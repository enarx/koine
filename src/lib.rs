// SPDX-License-Identifier: Apache-2.0

use http::response::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::os::unix::net::UnixStream;
use uuid::Uuid;
pub mod attestation;
pub mod threading;
pub const LOCAL_LISTEN_ADDRESS: &str = "0.0.0.0";

pub const PROTO_VERSION: f32 = 0.2;
pub const PROTO_NAME: &str = "Enarx-Keep-Manager";
pub const BIND_PORT: u16 = 3030;

#[derive(Serialize, Deserialize, Clone)]
pub enum LoaderState {
    Indeterminate,
    Ready,
    Running,
    Shutdown,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum Backend {
    Nil,
    Sev,
    Sgx,
    Kvm,
}

impl Backend {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Backend::Nil => "nil",
            Backend::Sev => "sev",
            Backend::Sgx => "sgx",
            Backend::Kvm => "kvm",
        }
    }
}

impl Backend {
    pub fn file_match(&self) -> &'static str {
        match *self {
            Backend::Nil => "/",
            Backend::Sev => "/dev/sev",
            Backend::Sgx => "/dev/sgx/enclave",
            Backend::Kvm => "/dev/kvm",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeepMgr {
    //pub ipaddr: IpAddr,
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeepContract {
    pub keepmgr: KeepMgr,
    pub backend: Backend,
    pub uuid: Uuid,
    pub socket_path: PathBuf,
    //TODO - add duration of contract availability
    //TODO - add further information
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Wasmldr {
    pub wasmldr_ipaddr: String,
    pub wasmldr_port: u16,
}

#[derive(Debug)]
pub struct KeepLdrConnection {
    pub kuuid: Uuid,
    pub keepldrstream: Option<UnixStream>,
    //TODO - allow time information for time-outs
    //pub established_time: ?
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Keep {
    pub backend: Backend,
    pub kuuid: Uuid,
    pub socket_path: PathBuf,
    pub state: LoaderState,
    pub wasmldr: Option<Wasmldr>,
    pub human_readable_info: Option<String>,
    pub certificate_as_pem: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Workload {
    pub wasm_binary: Vec<u8>,
    pub human_readable_info: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum CommsComplete {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeepVec {
    pub klvec: Vec<Keep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UndefinedReply {
    pub text: String,
}

//--------------cbor pieces below

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CborReply {
    pub msg: Vec<u8>,
}

impl warp::reply::Reply for CborReply {
    fn into_response(self) -> warp::reply::Response {
        Response::new(self.msg.into())
    }
}

#[derive(Debug)]
struct LocalCborErr {
    details: String,
}

impl fmt::Display for LocalCborErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LocalCborErr {
    fn description(&self) -> &str {
        &self.details
    }
}

impl warp::reject::Reject for LocalCborErr {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// SPDX-License-Identifier: Apache-2.0

//! The `sev` module exports the Message types required for both the
//! client and server components of the remote AMD SEV attestation
//! protocol[1].
//!
//! [1] https://github.com/enarx/enarx-keepldr/wiki/AMD-SEV-Remote-Attestation-Protocol

//taken wholesale for https://github.com/connorkuehl/koine/commit/8659386bbdce554872231636d00a4b94c69f3aa2
use serde::{Deserialize, Serialize};
use sev::certs::Chain;
use sev::launch;
use sev::Build;

/// The `Finish` struct contains useful information regarding the launch
/// configuration for the successfully launched VM.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Finish;

/// The `Measurement` struct is a container payload for concatenating
/// a `sev` crate `Build` type with a `sev` crate `Measurement` type.
///
/// The `build` and `measurement` fields are CBOR-encoded structures.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Measurement {
    pub build: Build,
    pub measurement: launch::Measurement,
}

/// A CBOR-encoded attestation message.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Message {
    /// The Naples certificate chain carries the SEV platform
    /// certificate chain, but the AMD ARK and ASK are "small"
    /// CA certificates (using 256 byte components).
    CertificateChainNaples(Chain),

    /// The Rome certificate chain also carries the SEV platform
    /// certificate chain, but the AMD ARK and ASK are "large"
    /// CA certificates (using 512 byte components).
    CertificateChainRome(Chain),

    /// The launch start buffer establishes a secure channel with
    /// the remote SEV platform and furnishes with information
    /// that the tenant has tailored to match their expectations
    /// for the secure VM.
    LaunchStart(launch::Start),

    /// The measurement buffer contains the AMD Secure Processor's
    /// Build and Measurement buffers so that the tenant may
    /// individually determine if the environment is correct.
    Measurement(Measurement),

    /// The secret packet contains the secret to inject into the
    /// secure VM. The payload is a CBOR-encoded `sev::launch::Secret`.
    Secret(Option<launch::Secret>),

    /// The finish message signals a successful attestation & launch.
    /// Its payload describes the relevant launch configuration details.
    Finish(Finish),
}

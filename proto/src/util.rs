// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use grpcio::{
    ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials, ServerCredentialsBuilder,
};

#[cfg(all(feature = "protobuf-codec", not(feature = "prost-codec")))]
use crate::testing::messages::{Payload, ResponseParameters};
#[cfg(feature = "prost-codec")]
use crate::testing::{Payload, ResponseParameters};

/// Create a payload with the specified size.
pub fn new_payload(size: usize) -> Payload {
    let mut payload = Payload::default();
    payload.set_body(vec![0; size]);
    payload
}

pub fn new_parameters(size: i32) -> ResponseParameters {
    let mut parameter = ResponseParameters::default();
    parameter.set_size(size);
    parameter
}

pub fn create_test_server_credentials() -> ServerCredentials {
    let private_key = include_str!("../data/server1.key");
    let cert = include_str!("../data/server1.pem");
    ServerCredentialsBuilder::new()
        .add_cert(cert.into(), private_key.into())
        .build()
}

pub fn create_test_channel_credentials() -> ChannelCredentials {
    let ca = include_str!("../data/ca.pem");
    ChannelCredentialsBuilder::new()
        .root_cert(ca.into())
        .build()
}

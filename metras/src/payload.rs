use base64::prelude::*;
use prost::Message;
use rama::{crypto::dep::x509_parser::nom::AsBytes, error::OpaqueError, net::user::Basic};
use secrecy::SecretString;

pub struct CredentialPayload {
    username: String,
    password: SecretString,
}

impl TryFrom<crate::schemas::proto::CredentialPayload> for CredentialPayload {
    type Error = OpaqueError;

    fn try_from(value: crate::schemas::proto::CredentialPayload) -> Result<Self, Self::Error> {
        Ok(Self {
            username: value.username,
            password: value.password.into(),
        })
    }
}

impl TryFrom<&str> for CredentialPayload {
    type Error = OpaqueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut payload = None;

        {
            let basic = Basic::try_from(value)?;
            let pass_payload_b64 = basic.password().as_bytes();
            let pass_payload_decoded = BASE64_URL_SAFE.decode(pass_payload_b64).unwrap();
            payload = Some(
                crate::schemas::proto::CredentialPayload::decode(pass_payload_decoded.as_bytes())
                    .unwrap(),
            );
        }

        let payload = payload
            .ok_or_else(|| OpaqueError::from_display("Failed to decode credential payload"))?;

        Ok(Self {
            username: payload.username,
            password: payload.password.into(),
        })
    }
}

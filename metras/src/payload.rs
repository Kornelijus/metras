use base64::prelude::*;
use prost::Message;
use rama::{crypto::dep::x509_parser::nom::AsBytes, error::OpaqueError, net::user::Basic};
use secrecy::SecretString;
use thiserror::Error;


type HttpBasicCredentials = rama::net::user::Basic;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing HTTP basic auth credential")]
    MissingCredential,
    #[error("Invalid credential payload")]
    InvalidCredentialPayload,
}

pub struct Credential {
    username: String,
    payload: CredentialPayloadIn,
}

pub struct CredentialPayloadIn {
    proxy_session_id: String,
    upstream_proxy_url: SecretString,
}

impl TryFrom<&HttpBasicCredentials> for Credential {
    type Error = self::Error;

    fn try_from(value: &HttpBasicCredentials) -> Result<Self, Self::Error> {
        let username = value.username();

        let pass_payload_b64 = value.password().as_bytes();
        let pass_payload_decoded = BASE64_URL_SAFE.decode(pass_payload_b64).map_err(|_| Error::InvalidCredentialPayload)?;
        let payload_message = 
            crate::proto::CredentialPayload::decode(pass_payload_decoded.as_bytes())
                .map_err(|_| Error::InvalidCredentialPayload)?;

        Ok(Self {
            username: username.into(),
            payload: CredentialPayloadIn::try_from(payload_message)?,
        })
    }
}

impl TryFrom<crate::proto::CredentialPayload> for CredentialPayloadIn {
    type Error = self::Error;

    fn try_from(value: crate::proto::CredentialPayload) -> Result<Self, Self::Error> {
        if value.proxy_session_id.is_empty() || value.upstream_proxy_url.is_empty() {
            return Err(Error::InvalidCredentialPayload);
        }

        Ok(Self {
            proxy_session_id: value.proxy_session_id,
            upstream_proxy_url: value.upstream_proxy_url.into(),
        })
    }
}

impl TryFrom<&str> for Credential {
    type Error = self::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let basic = Basic::try_from(value).map_err(|_| Error::MissingCredential)?;
        
        Self::try_from(&basic)
    }
}

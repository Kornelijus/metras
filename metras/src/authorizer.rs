use rama::net::user::authority::{AuthorizeResult, Authorizer, Unauthorized};

use crate::payload::CredentialPayloadIn;

struct PayloadAuthorizer;

impl Authorizer<CredentialPayloadIn> for PayloadAuthorizer {
    type Error = Unauthorized;

    async fn authorize(
        &self,
        credentials: CredentialPayloadIn,
    ) -> AuthorizeResult<CredentialPayloadIn, Self::Error> {
        // TODO: authorization for remote / external usage
        let result = true;
        result.authorize(credentials).await
    }
}

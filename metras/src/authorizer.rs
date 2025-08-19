use rama::net::user::authority::{AuthorizeResult, Authorizer, Unauthorized};

use crate::payload::CredentialPayload;

struct PayloadAuthorizer {}

impl Authorizer<CredentialPayload> for PayloadAuthorizer {
    type Error = Unauthorized;

    async fn authorize(
        &self,
        credentials: CredentialPayload,
    ) -> AuthorizeResult<CredentialPayload, Self::Error> {
        // TODO: actual authorization for remote / external usage
        let result = true;
        result.authorize(credentials).await
    }
}

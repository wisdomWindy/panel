use crate::{
    application::member::{
        AuthPollState, AuthProvider, AuthQrChallenge, EntitlementVerifier, LlmProvider,
        MemberServiceError, MembershipProvider, ModerationProvider, OrderRecord,
    },
    domain::{live_data::UserUid, member::MemberAccount},
};

pub struct UnavailableAuthProvider;

#[async_trait::async_trait]
impl AuthProvider for UnavailableAuthProvider {
    async fn generate_qr(&self) -> Result<AuthQrChallenge, MemberServiceError> {
        Err(MemberServiceError::unavailable("MEMBER_AUTH_QR_UNAVAILABLE"))
    }

    async fn poll_qr(&self, _provider_key: &str) -> Result<AuthPollState, MemberServiceError> {
        Err(MemberServiceError::unavailable("MEMBER_AUTH_QR_UNAVAILABLE"))
    }

    async fn current_account(&self) -> Result<MemberAccount, MemberServiceError> {
        Err(MemberServiceError::unavailable("MEMBER_AUTH_ACCOUNT_UNAVAILABLE"))
    }

    async fn query_uid(&self, _user_uid: &UserUid) -> Result<MemberAccount, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "MEMBER_AUTH_UID_LOOKUP_UNAVAILABLE",
        ))
    }

    async fn clear_session(&self) -> Result<(), MemberServiceError> {
        Ok(())
    }
}

pub struct UnavailableMembershipProvider;

pub struct UnavailableEntitlementVerifier;

impl EntitlementVerifier for UnavailableEntitlementVerifier {
    fn verify(&self, _payload: &[u8], _signature: &[u8]) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "MEMBER_ENTITLEMENT_VERIFIER_UNAVAILABLE",
        ))
    }
}

#[async_trait::async_trait]
impl MembershipProvider for UnavailableMembershipProvider {
    async fn plans(&self) -> Result<Vec<(String, String, u64, String)>, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "MEMBER_SERVICE_UNAVAILABLE",
        ))
    }

    async fn purchase(&self, _plan_id: &str) -> Result<OrderRecord, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "MEMBER_SERVICE_UNAVAILABLE",
        ))
    }

    async fn cancel_order(
        &self,
        _order_id: &crate::domain::member::OrderId,
    ) -> Result<OrderRecord, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "MEMBER_SERVICE_UNAVAILABLE",
        ))
    }
}

pub struct UnavailableLlmProvider;

#[async_trait::async_trait]
impl LlmProvider for UnavailableLlmProvider {
    async fn generate(&self, _prompt: &str) -> Result<String, MemberServiceError> {
        Err(MemberServiceError::unavailable("NET_LLM_UNAVAILABLE"))
    }
}

pub struct UnavailableModerationProvider;

#[async_trait::async_trait]
impl ModerationProvider for UnavailableModerationProvider {
    async fn mute(&self, _user_uid: &UserUid, _duration: &str) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "MEMBER_MODERATION_API_UNAVAILABLE",
        ))
    }
}

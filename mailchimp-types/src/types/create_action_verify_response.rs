pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The verified domains currently on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionVerifyResponse {
    /// Whether domain authentication is enabled for this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated: Option<bool>,
    /// The name of this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Returns whether the domain used is a public / free email provider. See [Limitations of Free Email Addresses](https://mailchimp.com/help/limitations-of-free-email-addresses/) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free_email_provider: Option<bool>,
    /// The Domain's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateActionVerifyResponseStatus>,
    /// The e-mail address receiving the two-factor challenge for this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_email: Option<String>,
    /// The date/time that the two-factor challenge was sent to the verification email.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub verification_sent: Option<DateTime<FixedOffset>>,
    /// Whether the domain has been verified for sending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl CreateActionVerifyResponse {
    pub fn builder() -> CreateActionVerifyResponseBuilder {
        <CreateActionVerifyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionVerifyResponseBuilder {
    authenticated: Option<bool>,
    domain: Option<String>,
    is_free_email_provider: Option<bool>,
    status: Option<CreateActionVerifyResponseStatus>,
    verification_email: Option<String>,
    verification_sent: Option<DateTime<FixedOffset>>,
    verified: Option<bool>,
}

impl CreateActionVerifyResponseBuilder {
    pub fn authenticated(mut self, value: bool) -> Self {
        self.authenticated = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn is_free_email_provider(mut self, value: bool) -> Self {
        self.is_free_email_provider = Some(value);
        self
    }

    pub fn status(mut self, value: CreateActionVerifyResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn verification_email(mut self, value: impl Into<String>) -> Self {
        self.verification_email = Some(value.into());
        self
    }

    pub fn verification_sent(mut self, value: DateTime<FixedOffset>) -> Self {
        self.verification_sent = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionVerifyResponse`].
    pub fn build(self) -> Result<CreateActionVerifyResponse, BuildError> {
        Ok(CreateActionVerifyResponse {
            authenticated: self.authenticated,
            domain: self.domain,
            is_free_email_provider: self.is_free_email_provider,
            status: self.status,
            verification_email: self.verification_email,
            verification_sent: self.verification_sent,
            verified: self.verified,
        })
    }
}

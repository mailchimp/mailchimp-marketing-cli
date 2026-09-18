pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactEmailChannel {
    /// A computation performed by the Mailchimp platform, triggered whenever any of its inputs change. Some inputs are controlled by API users, while others are tracked internally by the platform. Computation is based on: audience opt-in configuration (single vs. double opt-in), marketing consent status, and deliverability status (an internal state for a contact, maintained by Mailchimp for a specific marketing channel instance). This new API field is distinct from how contacts are displayed in the UI. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_subscription_status: Option<AudiencesContactEmailChannelEffectiveSubscriptionStatus>,
    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// MD5 hash of the email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_email: Option<String>,
    /// A contact's current consent status for email marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_consent: Option<AudiencesContactEmailChannelMarketingConsent>,
    /// The source from which the parent's entity was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AudiencesContactEmailChannelSource>,
}

impl AudiencesContactEmailChannel {
    pub fn builder() -> AudiencesContactEmailChannelBuilder {
        <AudiencesContactEmailChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactEmailChannelBuilder {
    effective_subscription_status: Option<AudiencesContactEmailChannelEffectiveSubscriptionStatus>,
    email: Option<String>,
    hashed_email: Option<String>,
    marketing_consent: Option<AudiencesContactEmailChannelMarketingConsent>,
    source: Option<AudiencesContactEmailChannelSource>,
}

impl AudiencesContactEmailChannelBuilder {
    pub fn effective_subscription_status(mut self, value: AudiencesContactEmailChannelEffectiveSubscriptionStatus) -> Self {
        self.effective_subscription_status = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn hashed_email(mut self, value: impl Into<String>) -> Self {
        self.hashed_email = Some(value.into());
        self
    }

    pub fn marketing_consent(mut self, value: AudiencesContactEmailChannelMarketingConsent) -> Self {
        self.marketing_consent = Some(value);
        self
    }

    pub fn source(mut self, value: AudiencesContactEmailChannelSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactEmailChannel`].
    pub fn build(self) -> Result<AudiencesContactEmailChannel, BuildError> {
        Ok(AudiencesContactEmailChannel {
            effective_subscription_status: self.effective_subscription_status,
            email: self.email,
            hashed_email: self.hashed_email,
            marketing_consent: self.marketing_consent,
            source: self.source,
        })
    }
}
